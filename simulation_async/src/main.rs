#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

extern crate env_logger;
extern crate time;
extern crate redis;
extern crate serde;
extern crate serde_json;
extern crate soundlines_core;

#[macro_use] mod timer;
mod models;

use std::env;
use std::collections::HashMap;

use soundlines_core::db;
use soundlines_core::db::models::*;
use soundlines_core::db::SqlType;
use soundlines_core::db::extensions::*;

mod errors {
	error_chain! {
		foreign_links {
			DbError(::soundlines_core::postgres::Error);
			RedisError(::redis::RedisError);
			SerdeError(::serde_json::error::Error);
		}
	}
}

fn run() -> errors::Result<()> {
	let redis_client = redis::Client::open("redis://localhost")?;
	let redis_conn = redis_client.get_connection()?;

	let dnas = fetch_dnas(&conn)?;
	let entities = fetch_entities(&conn, &dnas)?;
	let seeds = fetch_seeds(&conn, &dnas)?;

	trace!("Writing {} entities to redis", entities.len());

	measure!("Writing entities", {
		for (id, entity) in entities.iter() {
			redis_conn.set(format!("entity:{}", id), serde_json::to_string(entity)?);
		}
	});

	trace!("Writing {} seeds to redis", seeds.len());
}

fn run_x() -> errors::Result<()> {
	init_logger();

	trace!("Hello world");
	let conn = db::init_with_url("postgres://umurgdk@%2Fvar%2Frun%2Fpostgresql/soundlines");

	let overall_timer = timer::new("overall process");
	{
		let _dnas = fetch_dnas(&conn)?;
		let _entities = fetch_entities(&conn, &dnas)?;
		let _seeds = fetch_seeds(&conn, &dnas)?;

		measure!("update entities", conn.update_batch_hash(&_entities)?);
		measure!("update seeds", conn.update_batch_hash(&_seeds)?);
		measure!("update dnas", conn.update_batch_hash(&_dnas)?);
	}
	timer_finish!(overall_timer);
	Ok(())
}

fn fetch_entities(conn: &db::Connection, dnas: &HashMap<i32, Dna>) -> errors::Result<HashMap<i32, models::Entity>> {
	let fetching_entities = timer::new("fetching entities (hashmap)");
	let entities = conn.query("select * from entities", &[])?.into_iter()
		.map(Entity::from_sql_row)
		.map(|e| (e.id, models::entity_from_db(e, dnas)))
		.collect::<HashMap<_,_>>();
	timer_finish!(fetching_entities);
	Ok(entities)
}

fn fetch_seeds(conn: &db::Connection, dnas: &HashMap<i32, Dna>) -> errors::Result<HashMap<i32, Seed>> {
	let fetching_seeds = timer::new("fetching seeds (hashmap)");
	let seeds = conn.query("select * from seeds", &[])?.into_iter()
		.map(Seed::from_sql_row)
		.map(|s| (s.id.unwrap(), ))
		.collect::<HashMap<_, _>>();
	timer_finish!(fetching_seeds);
	Ok(seeds)
}

fn fetch_dnas(conn: &db::Connection) -> errors::Result<HashMap<i32, models::Dna>> {
	let fetching_dnas = timer::new("fetching dnas (hashmap)");
	let dnas = conn.query("select * from dnas", &[])?.into_iter()
		.map(Dna::from_sql_row)
		.map(models::dna_from_db)
		.map(|d| (d.id, d))
		.collect::<HashMap<_, _>>();
	timer_finish!(fetching_dnas);
	Ok(dnas)
}

quick_main!(run);

fn init_logger() {
	let format = |record: &log::LogRecord| {
		let now = time::now();
		let now_str = now.strftime("%d/%m/%y-%T").unwrap();

		if record.level() == log::LogLevel::Trace {
			format!("{}:{} {}::{}::{}", record.location().file(), record.location().line(), record.level(), now_str, record.args())
		} else {
			format!("{}::{}::{}::{}", record.level(), now_str, record.location().module_path(), record.args())
		}
	};

	let mut builder = env_logger::LogBuilder::new();
	builder.format(format);

	if env::var("RUST_LOG").is_ok() {
		builder.parse(&env::var("RUST_LOG").unwrap());
	}

	builder.init().expect("Failed to initialize logger");
}