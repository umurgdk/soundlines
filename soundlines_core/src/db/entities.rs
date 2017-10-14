use super::*;
use std::collections::HashMap;
use models::Entity;
use models::NeighborEntry;

pub fn get(conn: &Connection, id: i32) -> ::errors::Result<Entity> {
	let sql = r#"
		SELECT
			(
				entities.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
					'point', st_asgeojson(entities.point)::jsonb->'coordinates',
					'id', entities.id
				)
			)::text
		FROM entities_json entities
		INNER JOIN settings ON settings.id = entities.setting_id
		WHERE entities.id = $1
	"#;

	let rows = conn.query(&sql, &[&id])?;
	let row = rows.get(0);
	::serde_json::from_str(&row.get::<_, String>(0)).map_err(|e| e.into())
}

pub fn all(conn: &Connection) -> ::errors::Result<::postgres::rows::Rows> {
	let sql = r#"
		SELECT
			(
				entities.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
					'point', st_asgeojson(entities.point)::jsonb->'coordinates',
					'id', entities.id
				)
			)::text
		FROM entities_json AS entities
		INNER JOIN settings ON settings.id = entities.setting_id
	"#;

	conn.query(sql, &[]).map_err(|e| e.into())
}

pub fn all_vec(conn: &Connection) -> ::errors::Result<Vec<Entity>> {
	let rows = all(conn)?;
	let entities = rows.into_iter().map(|r| ::serde_json::from_str(&r.get::<_, String>(0)).unwrap()).collect();
	Ok(entities)
}

pub fn all_hashmap(conn: &Connection) -> ::errors::Result<HashMap<i32, Entity>> {
	let rows = all(conn)?;
	let entities = rows.into_iter().map(|r| ::serde_json::from_str::<Entity>(&r.get::<_, String>(0)).unwrap()).map(|e| (e.id, e)).collect();
	Ok(entities)
}

pub fn all_by_setting_id(conn: &Connection, setting_id: i32) -> ::errors::Result<HashMap<i32, Entity>> {
	let sql = r#"
		SELECT
			(
				entities.data ||
				jsonb_build_object(
					'setting', to_jsonb("settings"),
					'point', st_asgeojson(entities.point)::jsonb->'coordinates',
					'id', entities.id
				)
			)::text
		FROM entities_json AS entities
		INNER JOIN settings ON settings.id = entities.setting_id
		WHERE entities.setting_id = $1
	"#;

	let rows = conn.query(&sql, &[&setting_id])?;
	let entities = rows.into_iter().map(|r| ::serde_json::from_str::<Entity>(&r.get::<_, String>(0)).unwrap()).map(|e| (e.id, e)).collect();
	Ok(entities)
}

pub fn get_neighbor(conn: &Connection, id: i32) -> ::errors::Result<NeighborEntry> {
	let sql = "select json_build_array(entity_id, mating_neighbors, crowd_neighbors)::text from entity_neighbors where entity_id = $1";
	let stmt = conn.prepare_cached(&sql)?;
	let rows = stmt.query(&[&id])?;
	let row = rows.get(0);
	::serde_json::from_str::<(i32, Vec<String>, Vec<String>)>(&row.get::<_, String>(0))
		.map(|(_, mating_neighbors, crowd_neighbors)|
			NeighborEntry {
				mating_neighbors: mating_neighbors.into_iter().map(|i| i.parse().unwrap()).collect(),
				crowd_neighbors: crowd_neighbors.into_iter().map(|i| i.parse().unwrap()).collect()
			}
		)
		.map_err(|e| e.into())
}

pub fn all_neighbors(conn: &Connection) -> ::errors::Result<HashMap<i32, NeighborEntry>> {
	let sql = "select json_build_array(entity_id, mating_neighbors, crowd_neighbors)::text from entity_neighbors";
	let rows = conn.query(sql, &[])?;
	let neighbors = rows.into_iter()
		.map(|r| ::serde_json::from_str::<(i32, Vec<String>, Vec<String>)>(&r.get::<_, String>(0)).unwrap())
		.map(|(entry_id, mating_neighbors, crowd_neighbors)| (entry_id, NeighborEntry {
			mating_neighbors: mating_neighbors.into_iter().map(|i| i.parse().unwrap()).collect(),
			crowd_neighbors: crowd_neighbors.into_iter().map(|i| i.parse().unwrap()).collect()
		}))
		.collect();
	Ok(neighbors)
}

pub fn batch_update(conn: &Connection, entities: HashMap<i32, Entity>) -> ::errors::Result<()> {
	use std::io::Cursor;
	let timer = ::helpers::new_timer(format!("Updating {} entities on database", entities.len()));

	let t = conn.transaction()?;
	t.batch_execute("create temp table entities_batch_update (id integer primary key, data jsonb);")?;
	let copy = t.prepare("copy entities_batch_update from stdin")?;

	let mut rows = String::new();
	for (id, entity) in entities.iter() {
		let json = ::serde_json::to_string(entity)?;
		rows.push_str(&format!("{}\t{}\n", id, json));
	}

	copy.copy_in(&[], &mut Cursor::new(rows))?;
	copy.finish()?;

	let updated_rows = t.execute("update entities_json e set data = u.data from entities_batch_update u where u.id = e.id", &[])?;
	t.batch_execute("drop table entities_batch_update;")?;
	t.commit()?;

	timer_finish!(timer);
	trace!("{} entities updated on the database", updated_rows);

	Ok(())
}

pub fn batch_insert(conn: &Connection, entities: &[Entity]) -> ::errors::Result<Vec<i32>> {
	let timer = ::helpers::new_timer(format!("Inserting {} entities", entities.len()));
	let stmt = conn.prepare("insert into entities_json (data, setting_id, cell_id, point) values ($1, $2, $3, st_geomfromtext($4, 4326)) returning id")?;

	let mut new_ids = Vec::new();
	for entity in entities {
		let json = ::serde_json::to_value(entity)?;
		let point = format!("POINT({} {})", entity.point[0], entity.point[1]);
		let rows = stmt.query(&[&json, &entity.setting.id, &entity.cell_id, &point])?;
		new_ids.push(rows.get(0).get(0));
	}

	stmt.finish()?;

	timer_finish!(timer);
	Ok(new_ids)
}

pub fn batch_delete(conn: &Connection, ids: &[i32]) -> ::errors::Result<()> {
	conn.execute("delete from entities_json where id = any($1)", &[&ids])?;
	Ok(())
}

//pub fn update_neighbors(conn: &Connection, new_entities: &[i32], deleted_entities: &[i32]) -> ::errors::Result<()> {
//	let delete_sql = r#"
//		update entity_neighbors
//		set
//		  mating_neighbors = mating_neighbors - '$1',
//		  crowd_neighbors = crowd_neighbors - '$1'
//		where mating_neighbors ? '31912' or crowd_neighbors ? '$1';
//	"#;
//
//	let stmt = conn.prepare_cached(&delete_sql)?;
//	for deleted_id in &deleted_entities {
//		stmt.execute(&[&deleted_id])?;
//	}
//
//	let find_mate_neighbors_stmt = conn.prepare_cached(r#"
//		SELECT id
//		FROM entities_json
//		INNER JOIN settings_json ON settings_json.id = entities_json.setting_id
//		WHERE st_dwithin(point :: GEOGRAPHY, $1 :: GEOGRAPHY, (settings_json.data -> 'mating_distance') :: TEXT :: NUMERIC)
//	"#)?;
//
//	let find_crowd_neighbors_stmt = conn.prepare_cached(r#"
//		SELECT id
//		FROM entities_json
//		INNER JOIN settings_json ON settings_json.id = entities_json.setting_id
//		WHERE st_dwithin(point :: GEOGRAPHY, $1 :: GEOGRAPHY, (settings_json.data -> 'mating_distance') :: TEXT :: NUMERIC)
//	"#)?;
//
//	for new_id in &new_entities {
//		let neighbors =
//	}
////	let sql = "select entity_id, mating_neighbors::text, crowd_neighbors::text from entity_neighbors where entity_id in $1";
////	let rows = conn.query(sql, &[deleted_entities])?;
////	for row in rows.iter() {
////		let entity_id = row.get(0);
////		let mating_neighbors = ::serde_json::from_str::<Vec<i32>>(&row.get::<_, String>(1))?;
////		let crowd_neighbors =
////	}
//}