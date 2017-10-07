use super::*;
use ::models::Cell;
use std::collections::HashMap;

/// Get all cells
pub fn all(conn: &Connection) -> ::errors::Result<::postgres::rows::Rows> {
	let query = r#"
		select
			(
				to_jsonb(cells.*) ||
				jsonb_build_object('geom', st_asgeojson(geom)::jsonb->'coordinates'->0)
			)::text
		from cells;
	"#;

	conn.query(&query, &[]).map_err(|e| e.into())
}

pub fn all_hashmap(conn: &Connection) -> ::errors::Result<HashMap<i32, Cell>> {
	let rows = all(conn)?;
	let cells = rows.into_iter().map(|r| ::serde_json::from_str::<Cell>(&r.get::<_, String>(0)).unwrap()).map(|c| (c.id, c)).collect();
	Ok(cells)
}

pub fn all_vec(conn: &Connection) -> ::errors::Result<Vec<Cell>> {
	let rows = all(conn)?;
	let cells = rows.into_iter().map(|r| ::serde_json::from_str::<Cell>(&r.get::<_, String>(0)).unwrap()).collect();
	Ok(cells)
}

pub fn get(conn: &Connection, id: i32) -> ::errors::Result<Cell> {
	let query = r#"
		select
			(
				to_jsonb(cells.*) ||
				jsonb_build_object('geom', st_asgeojson(geom)::jsonb->'coordinates'->0)
			)::text
		from cells
		where cells.id = $1
	"#;

	let rows = conn.query(&query, &[&id])?;
	let row = rows.get(0);
	::serde_json::from_str(&row.get::<_, String>(0)).map_err(|e| e.into())
}

pub fn find_contains_any(conn: &Connection, points: Vec<::models::Point>) -> ::errors::Result<Vec<Option<i32>>> {
	let points = points.into_iter().map(|p| format!("POINT({} {})", p[0], p[1])).collect::<Vec<_>>().join(", ");
	let points = format!("'{{{}}}'", points);
	let query = format!(r#"
		select cells.id
		from (select st_geomfromtext(l, 4326) as point from unnest({}::text[]) as l) as points
        left join cells on st_contains(cells.geom, points.point);
	"#, points);

	let rows = conn.query(&query, &[])?;
	Ok(rows.into_iter().map(|r| r.get(0)).collect())
}