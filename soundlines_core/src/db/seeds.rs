use std::collections::HashMap;

use super::*;
use ::models::Seed;

pub fn all(conn: &Connection) -> ::errors::Result<::postgres::rows::Rows> {
	let query = r#"
		SELECT
			(
				seeds.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
				    'point', st_asgeojson(seeds.point)::jsonb->'coordinates',
				    'id', seeds.id
				)
			)::text
		FROM seeds_json AS seeds
		INNER JOIN settings ON settings.id = seeds.setting_id
	"#;

	conn.query(&query, &[]).map_err(|e| e.into())
}

pub fn all_hashmap(conn: &Connection) -> ::errors::Result<HashMap<i32, Seed>> {
	let rows = all(conn)?;
	let seeds = rows.into_iter().map(|r| ::serde_json::from_str::<Seed>(&r.get::<_, String>(0)).unwrap()).map(|s| (s.id, s)).collect();
	Ok(seeds)
}

pub fn all_by_setting_id(conn: &Connection, setting_id: i32) -> ::errors::Result<HashMap<i32, Seed>> {
	let sql = r#"
		SELECT
			(
				seeds.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
				    'point', st_asgeojson(seeds.point)::jsonb->'coordinates',
				    'id', seeds.id
				)
			)::text
		FROM seeds_json AS seeds
		INNER JOIN settings ON settings.id = seeds.setting_id
		WHERE seeds_json.setting_id = $1
	"#;

	let rows = conn.query(&sql, &[&setting_id])?;
	let seeds = rows.into_iter().map(|r| ::serde_json::from_str::<Seed>(&r.get::<_, String>(0)).unwrap()).map(|s| (s.id, s)).collect();
	Ok(seeds)
}

pub fn get(conn: &Connection, id: i32) -> ::errors::Result<Seed> {
	let query = r#"
		SELECT
			(
				seeds.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
				    'point', st_asgeojson(seeds.point)::jsonb->'coordinates',
				    'id', seeds.id
				)
			)::text
		FROM seeds_json AS seeds
		INNER JOIN settings ON settings.id = seeds.setting_id
		WHERE seeds_json.id = $1
	"#;

	let rows = conn.query(&query, &[&id])?;
	let row = rows.get(0);
	::serde_json::from_str(&row.get::<_, String>(0)).map_err(|e| e.into())
}

pub fn batch_update(conn: &Connection, seeds: HashMap<i32, Seed>) -> ::errors::Result<()> {
	use std::io::Cursor;
	let timer = ::helpers::new_timer(format!("Updating {} seeds on database", seeds.len()));

	let t = conn.transaction()?;
	t.batch_execute("create temp table seeds_batch_update (id integer primary key, data jsonb);")?;
	let copy = t.prepare("copy seeds_batch_update from stdin")?;

	for (id, seed) in seeds.iter() {
		let json = ::serde_json::to_string(seed)?;
		let mut row = Cursor::new(format!("{}\t{}", id, json));
		copy.copy_in(&[], &mut row)?;
	}

	copy.finish()?;
	let updated_rows = t.execute("update seeds_json e set data = u.data from seeds_batch_update u where u.id = e.id", &[])?;
	t.batch_execute("drop table seeds_batch_update;")?;
	t.commit()?;

	timer_finish!(timer);
	trace!("{} seeds updated on the database", updated_rows);

	Ok(())
}

pub fn batch_insert(conn: &Connection, seeds: &[Seed]) -> ::errors::Result<()> {
	let stmt = conn.prepare("insert into seeds_json (data, setting_id, cell_id, point) values ($1, $2, $3, st_geomfromtext($4, 4326))")?;
	for seed in seeds {
		let json = ::serde_json::to_string(seed)?;
		let point = format!("POINT({} {})", seed.point[0], seed.point[1]);
		stmt.execute(&[&json, &seed.setting.id, &seed.cell_id, &point])?;
	}

	stmt.finish().map_err(|e| e.into())
}

pub fn batch_delete(conn: &Connection, ids: &[i32]) -> ::errors::Result<()> {
	conn.execute("delete from seeds_json where id = any($1)", &[&ids])?;
	Ok(())
}