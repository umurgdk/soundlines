use std::error::Error;
use std::collections::HashMap;

use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;

use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Seed;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::extensions::*;
use soundlines_core::db::Result as DbResult;
use soundlines_core::postgis::ewkb::Point;

use soundlines_simlib::sim_seed::SeedDraft;
use soundlines_simlib::sim_seed::generate as generate_seed;

use user::Auth;
use db_guard::DbConn;

#[get("/")]
pub fn index(conn: DbConn) -> DbResult<Json> {
	conn.all::<Seed>()
		.map(|seeds| seeds.into_iter().map(Seed::into_json).collect::<Vec<_>>())
		.map(|seeds| json!({ "seeds": seeds }))
		.map(Json)
}

#[derive(Deserialize)]
pub struct PickupPayload {
	id: i32
}

#[post("/pickup", data = "<payload>")]
pub fn pickup(_auth: Auth, payload: Json<PickupPayload>, conn: DbConn) -> DbResult<Option<Json>> {
	let seed_id = payload.into_inner().id;

	let seed = match conn.get::<Seed>(seed_id)? {
		Some(seed) => seed,
		None => return Ok(None)
	};

	conn.delete::<Seed>(seed_id)?;

	Ok(Some(Json(seed.into_json())))
}

#[get("/get/<count>")]
pub fn get(_auth: Auth, conn: DbConn, count: u32) -> DbResult<Json<Value>> {
	let plant_settings = conn.all::<PlantSetting>()?;
	let prefabs = plant_settings.iter().map(|setting| {
		(setting.id.unwrap(), &setting.prefab)
	}).collect::<HashMap<_, _>>();

	let cell = conn.first::<Cell>()?.expect("No cell found in database");

	let mut dnas_to_insert = Vec::<Dna>::with_capacity(count as usize);
	let mut locations = Vec::<Point>::with_capacity(count as usize);

	for _ in 0..count {
		let SeedDraft { dna, location, .. } = generate_seed(&plant_settings, &cell);
		dnas_to_insert.push(dna);
		locations.push(location);
	}

	let dnas: Vec<Dna> = conn.insert_batch_return(&dnas_to_insert, true)?;
	let seeds = locations.into_iter().enumerate().map(|(i, location)| {
		Seed::new(dnas[i].id, location, cell.id, dnas[i].setting_id, prefabs[&dnas[i].setting_id].to_owned())
			.into_json()
	}).collect::<Vec<_>>();

	Ok(Json(json!({
		"seeds": seeds
	})))
}

#[derive(Deserialize)]
pub struct DeployPayload {
	pub count: u32,
	pub cell_ids: Vec<i32>,
	pub prefab: Option<String>
}

#[post("/deploy", data = "<payload>")]
pub fn deploy(conn: DbConn, payload: Json<DeployPayload>) -> Result<Json, Failure> {
	let DeployPayload { count, cell_ids, prefab } = payload.into_inner();

	println!("Getting plant settings");
	let plant_settings = if prefab.is_some() {
		let setting = PlantSetting::find_by_prefab(&prefab.unwrap(), &*conn)
			.map_err(|_| Failure(Status::InternalServerError))?
			.ok_or(Failure(Status::BadRequest))?;

		vec![setting]
	} else {
		conn.all().map_err(|_| Failure(Status::InternalServerError))?
	};

	let prefabs = plant_settings.iter().map(|setting| {
		(setting.id.unwrap(), &setting.prefab)
	}).collect::<HashMap<_, _>>();

	println!("Getting cells");
	let cells = Cell::find_by_ids(&*conn, &cell_ids)
		.map_err(|_| Failure(Status::InternalServerError))?;

	let mut dnas_to_insert = Vec::<Dna>::with_capacity(count as usize);
	let mut locations = Vec::<Point>::with_capacity(count as usize);
	let mut cell_ids = Vec::<i32>::with_capacity(count as usize);

	for cell in cells {
		for _ in 0..count {
			let SeedDraft { dna, location, cell_id } = generate_seed(&plant_settings, &cell);
			dnas_to_insert.push(dna);
			locations.push(location);
			cell_ids.push(cell.id);
		}
	}

	println!("Inserting dnas");
	let dnas: Vec<Dna> = conn.insert_batch_return(&dnas_to_insert, true)
		.map_err(|_| Failure(Status::InternalServerError))?;

	println!("Inserting seeds");
	let seeds = locations.into_iter().enumerate().map(|(i, location)| {
		Seed::new(dnas[i].id, location, cell_ids[i], dnas[i].setting_id, prefabs[&dnas[i].setting_id].to_owned())
	}).collect::<Vec<_>>();

	let seeds = conn.insert_batch_return(&seeds, true)
		.map_err(|_| Failure(Status::InternalServerError))?
		.into_iter()
		.map(Seed::into_json)
		.collect::<Vec<_>>();

	Ok(Json(json!({
		"seeds": seeds
	})))
}

#[derive(Deserialize)]
pub struct SpreadSeedPayload {
	latitude: f64,
	longitude: f64,
	nickname: String,
	dna_id: i32,
	setting_id: i32
}

#[post("/spread", data = "<payload>")]
pub fn spread(_auth: Auth, payload: Json<SpreadSeedPayload>, conn: DbConn) -> Result<Json, Failure> {
	let payload = payload.into_inner();

	let location = Point::new(payload.longitude, payload.latitude, Some(4326));
	let cell = Cell::find_containing_core(&*conn, &location);
	let cell = cell
		.map_err(|_| Failure(Status::InternalServerError))?
		.ok_or(Failure(Status::BadRequest))?;

	let dna = conn.get::<Dna>(payload.dna_id)
		.map_err(|_| Failure(Status::InternalServerError))?
		.ok_or(Failure(Status::BadRequest))?;

	let setting = conn.get::<PlantSetting>(payload.setting_id)
		.map_err(|_| Failure(Status::InternalServerError))?
		.ok_or(Failure(Status::BadRequest))?;

	let mut entity = Entity::new(location, cell.id, &setting, &dna);
	entity.nickname = payload.nickname;

	let entity = conn.insert(&entity).map_err(|_| Failure(Status::InternalServerError))?;

	Ok(Json(entity.to_json()))
}
