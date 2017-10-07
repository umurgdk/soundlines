use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use postgres::notification::Notification;

use models::*;
use db;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
enum DbOperation {
	Insert,
	Delete,
	Update
}

/// Actions for database synchronization thread
#[derive(Debug, Clone)]
pub enum DbSyncAction {
	/// Collection of data needs to be updated/created on the database
	DbWriteMessage {
		/// Updated entities
		entities: HashMap<i32, Entity>,
		/// Updated seeds
		seeds: HashMap<i32, Seed>,
		/// Dead entities to be deleted
		dead_entities: Vec<i32>,
		/// Bloomed seeds to be deleted
		bloomed_seeds: Vec<i32>,
		/// New seeds to be inserted
		seed_drafts: Vec<SeedDraft>,
		/// New entities to be inserted
		entity_drafts: Vec<EntityDraft>
	},
	/// Stops the synchronization thread
	Quit
}

#[derive(Debug, Clone, Deserialize)]
struct DbNotification {
	pub table: String,
	pub operation: DbOperation,
	pub id: i32
}

/// Checks if there are any pending/incoming database notifications and updates cached values of
/// the simulation if necessary
pub fn handle_notifications(conn: &db::Connection, mut entities: &mut HashMap<i32, Entity>, mut neighbors: &mut HashMap<i32, Vec<i32>>, mut seeds: &mut HashMap<i32, Seed>, mut cells: &mut HashMap<i32, Cell>) -> ::errors::Result<()> {
	use fallible_iterator::FallibleIterator;

	let notifications = conn.notifications();
	let one_millisecond = ::std::time::Duration::from_millis(1);

	// Timeout iterator waits until given duration if there is no notifications pending
	let mut notification_iter = notifications.timeout_iter(one_millisecond);
	while let Some(notification) = notification_iter.next()? {
		process_notification(&conn, notification, &mut entities, &mut neighbors, &mut seeds, &mut cells)?;
	}

	Ok(())
}

/// Updates simulation's cached values on insert and delete operations on selective tables. It may
/// make new database queries to fetch/update cached data.
fn process_notification(conn: &db::Connection, pg_notification: Notification, entities: &mut HashMap<i32, Entity>, neighbors: &mut HashMap<i32, Vec<i32>>, seeds: &mut HashMap<i32, Seed>, cells: &mut HashMap<i32, Cell>) -> ::errors::Result<()> {
	if pg_notification.channel != "simulation" {
		return Ok(());
	}

	let notification = match ::serde_json::from_str::<DbNotification>(&pg_notification.payload) {
		Ok(notification) => notification,
		_ => return Ok(warn!("Got a database notification in different shape, ignoring it... {}", pg_notification.payload))
	};

	match notification.table.as_str() {
		"entities_json" => handle_entities_notification(conn, notification.operation, notification.id, entities)?,
		"entity_neighbors" => handle_neighbors_notification(conn, notification.operation, notification.id, neighbors)?,
		"seeds_json" => handle_seeds_notification(conn, notification.operation, notification.id, seeds)?,
		"cells" => handle_cells_notification(conn, notification.operation, notification.id, cells)?,
		"settings" => handle_settings_notification(conn, notification.operation, notification.id, entities, seeds)?,
		_ => warn!("Got a database notification on table {}, don't know what to do...", notification.table)
	}

	Ok(())
}

fn handle_entities_notification(conn: &db::Connection, operation: DbOperation, subject_id: i32, entities: &mut HashMap<i32, Entity>) -> ::errors::Result<()> {
	match operation {
		DbOperation::Insert => {entities.insert(subject_id, db::entities::get(conn, subject_id)?);},
		DbOperation::Delete => {entities.remove(&subject_id);},
		_ => ()
	}

	Ok(())
}

fn handle_neighbors_notification(conn: &db::Connection, operation: DbOperation, subject_id: i32, neighbors: &mut HashMap<i32, Vec<i32>>) -> ::errors::Result<()> {
	match operation {
		DbOperation::Insert | DbOperation::Update => {neighbors.insert(subject_id, db::entities::get_neighbor(conn, subject_id)?);},
		DbOperation::Delete => {neighbors.remove(&subject_id);},
	}

	Ok(())
}

fn handle_seeds_notification(conn: &db::Connection, operation: DbOperation, subject_id: i32, seeds: &mut HashMap<i32, Seed>) -> ::errors::Result<()> {
	match operation {
		DbOperation::Insert => {seeds.insert(subject_id, db::seeds::get(conn, subject_id)?);},
		DbOperation::Delete => {seeds.remove(&subject_id);},
		_ => ()
	}

	Ok(())
}

fn handle_cells_notification(conn: &db::Connection, operation: DbOperation, subject_id: i32, cells: &mut HashMap<i32, Cell>) -> ::errors::Result<()> {
	match operation {
		DbOperation::Insert | DbOperation::Update => {cells.insert(subject_id, db::cells::get(conn, subject_id)?);},
		DbOperation::Delete => {cells.remove(&subject_id);}
	}

	Ok(())
}

fn handle_settings_notification(conn: &db::Connection, operation: DbOperation, subject_id: i32, entities: &mut HashMap<i32, Entity>, seeds: &mut HashMap<i32, Seed>) -> ::errors::Result<()> {
	if operation != DbOperation::Update {
		return Ok(());
	}

	let affected_entities = db::entities::all_by_setting_id(conn, subject_id)?;
	let affected_seeds = db::seeds::all_by_setting_id(conn, subject_id)?;

	entities.extend(affected_entities);
	seeds.extend(affected_seeds);

	Ok(())
}

pub fn start_db_writer() -> Sender<DbSyncAction> {
	let (sender, receiver) = channel::<DbSyncAction>();

	thread::spawn(move || {
		loop {
			let res = match receiver.recv() {
				Ok(DbSyncAction::Quit) => break,
				Ok(DbSyncAction::DbWriteMessage { entities, seeds, dead_entities, bloomed_seeds, seed_drafts, entity_drafts, }) =>
					write_to_db(entities, seeds, dead_entities, bloomed_seeds, seed_drafts, entity_drafts),
				Err(e) => { error!("{}", e); ::std::process::exit(1); }
			};

			if let Err(err) = res {
				error!("{}", err);
				::std::process::exit(1);
			}
		}
	});

	sender
}

fn write_to_db(entities: HashMap<i32, Entity>, seeds: HashMap<i32, Seed>, dead_entities: Vec<i32>, bloomed_seeds: Vec<i32>, seed_drafts: Vec<SeedDraft>, entity_drafts: Vec<EntityDraft>) -> ::errors::Result<()> {
	let timer = ::helpers::new_timer("Synchronizing database");

	// Run udpate queries
	::crossbeam::scope(|scope| {
		scope.spawn(move || db::entities::batch_update(&db::connect(), entities).unwrap());
		scope.spawn(move || db::seeds::batch_update(&db::connect(), seeds).unwrap());
	});

	// Do insertions and deletions after updates to prevent any concurrency issues
	::crossbeam::scope(|scope| {
		scope.spawn(move || db::entities::batch_delete(&db::connect(), &dead_entities).unwrap());
		scope.spawn(move || {
			let conn = db::connect();
			let points = entity_drafts.iter().map(|e| e.point.clone()).collect();
			let cell_ids = db::cells::find_contains_any(&conn, points).unwrap();
			let entities = ::models::make_entities_from_drafts(entity_drafts, &cell_ids);
			db::entities::batch_insert(&conn, &entities).unwrap();
		});

		scope.spawn(move || db::seeds::batch_delete(&db::connect(), &bloomed_seeds).unwrap());
		scope.spawn(move || {
			let conn = db::connect();
			let points = seed_drafts.iter().map(|s| s.point.clone()).collect();
			let cell_ids = db::cells::find_contains_any(&conn, points).unwrap();
			let seeds = ::models::make_seeds_from_drafts(seed_drafts, &cell_ids);
			db::seeds::batch_insert(&conn, &seeds).unwrap();
		});
	});

	timer_finish!(timer);
	Ok(())
}