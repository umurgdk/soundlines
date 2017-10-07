#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate soundlines_core;
extern crate env_logger;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate fallible_iterator;
extern crate crossbeam;
extern crate chrono;

use std::collections::HashMap;

use soundlines_core::errors;
use soundlines_core::db;
use soundlines_core::helpers;
use soundlines_core::simulation::sync;
use soundlines_core::models::*;

quick_main!(run);

fn run() -> ::errors::Result<()> {
	::env_logger::init()?;
	let conn = db::connect();

	let mut entities = measure!("Loading entities", db::entities::all_hashmap(&conn)?);
	let mut seeds = measure!("Loading seeds", db::seeds::all_hashmap(&conn)?);
	let mut neighbors = measure!("Loading neighbors", db::entities::all_neighbors(&conn)?);
	let mut cells = measure!("Loading cells", db::cells::all_hashmap(&conn)?);

	let mut seed_drafts = Vec::new();
	let mut entity_drafts = Vec::new();
	let mut bloomed_seeds = Vec::new();
	let mut dead_entities = Vec::new();

	info!("{} entities loaded", entities.len());
	info!("{} seeds loaded", seeds.len());

	info!("Starting db sync thread...");
	let sync_sender = sync::start_db_writer();

	info!("Start listening for simulation events on database...");
	db::listen(&conn, "simulation")?;

	info!("Starting loop...");
	let mut step_timer = helpers::new_timer("Step timer");
	let mut frame_count = 0;
	loop {
		sync::handle_notifications(&conn, &mut entities, &mut neighbors, &mut seeds, &mut cells)?;

		crossbeam::scope(|scope| {
			// Update entities
			scope.spawn(|| {
				let (new_deads, new_seed_drafts) = update_entities(&mut entities, &neighbors, &cells);
				dead_entities.extend(new_deads);
				seed_drafts.extend(new_seed_drafts);
			});

			// Update seeds in parallel
			scope.spawn(|| {
				let (new_blooms, new_entitiy_drafts) = update_seeds(&mut seeds);
				bloomed_seeds.extend(new_blooms);
				entity_drafts.extend(new_entitiy_drafts);
			});
		});

		frame_count += 1;

		// Sync database every 5 seconds
		if step_timer.since().as_secs() > 5 {
			measure!("Sending db sync message", {
				use soundlines_core::errors::ResultExt;
				sync_sender.send(sync::DbSyncAction::DbWriteMessage {
					entities: entities.clone(),
					seeds: seeds.clone(),
					dead_entities: dead_entities.clone(),
					bloomed_seeds: bloomed_seeds.clone(),
					seed_drafts: seed_drafts.clone(),
					entity_drafts: entity_drafts.clone()
				}).chain_err(|| "Failed to send message to sync thread")?;
			});

			info!("~{} Steps are taken per second", frame_count / step_timer.since().as_secs());
			info!("{} seeds bloomed", bloomed_seeds.len());
			info!("{} entities died", dead_entities.len());
			info!("{} new seeds born", seed_drafts.len());

			step_timer.reset();
			frame_count = 0;
			seed_drafts.clear();
			entity_drafts.clear();
			bloomed_seeds.clear();
			dead_entities.clear();
		}
	}
}

fn update_seeds(seeds: &mut HashMap<i32, Seed>) -> (Vec<i32>, Vec<EntityDraft>) {
	seeds.iter_mut().filter_map(|(_, seed)| {
		seed.update();

		if seed.should_bloom() {
			return Some((seed.id, EntityDraft { point: seed.point.clone(), setting: seed.setting.clone(), dna: seed.dna.clone() }));
		}

		return None;
	}).unzip()
}

fn update_entities(entities: &mut HashMap<i32, Entity>, neighbors: &HashMap<i32, Vec<i32>>, cells: &HashMap<i32, Cell>) -> (Vec<i32>, Vec<SeedDraft>) {
	let old_entities = entities.clone();

	let matched_entities = entities.iter_mut().filter_map(|(_, entity)| {
		// FIXME: shouldn't fallback to first cell
		let cell = cells.get(&entity.cell_id).unwrap_or(cells.values().next().unwrap());
		update_entity(entity, &neighbors, &old_entities, cell)
	}).collect::<Vec<(i32, i32)>>();

	// Matching possible entity candidates
	let seed_drafts = matched_entities.into_iter().filter_map(|(entity1_id, entity2_id)| {
		match_entities(&entities[&entity1_id], &entities[&entity2_id]).map(|seed_draft| {
			entities.get_mut(&entity1_id).unwrap().mate();
			entities.get_mut(&entity2_id).unwrap().mate();
			seed_draft
		})
	}).collect();

	(vec![], seed_drafts)
}

fn update_entity(entity: &mut Entity, neighbors: &HashMap<i32, Vec<i32>>, entities: &HashMap<i32, Entity>, cell: &Cell) -> Option<(i32, i32)> {
	let no_neighbors = vec![];
	let entity_neighbors = neighbors.get(&entity.id).unwrap_or(&no_neighbors);
	entity.update_by_neighbors(cell, entity_neighbors.len() as i32);

	if entity.is_mating() {
		let matched_entity_id = ::helpers::random_choice_with(entity_neighbors, |candidate_id| {
			*candidate_id != entity.id && entities.get(&candidate_id).map(Entity::is_mating).unwrap_or(false)
		});

		return matched_entity_id.cloned().map(|match_id| (entity.id, match_id));
	}

	None
}

fn match_entities(entity1: &Entity, entity2: &Entity) -> Option<SeedDraft> {
	if ::helpers::random(0.0, 1.05) >= entity1.setting.birth_proba {
		return None;
	}

	let setting = entity1.setting.clone();
	let point = ::helpers::get_seed_location(entity1, entity2);
	let dna = entity1.dna.reproduce(&entity2.dna);

	Some(SeedDraft { dna, setting, point })
}