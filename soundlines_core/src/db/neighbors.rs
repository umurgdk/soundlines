use std::collections::HashMap;
use postgres::rows::Rows;
use fallible_iterator::FallibleIterator;

use db::Connection;
use models::Point;
use models::NeighborEntry;

pub fn all(conn: &Connection) -> ::errors::Result<Rows> {
	let sql = "select entity_id, mating_neighbors, crowd_neighbors from entity_neighbors";
	let rows = conn.query(&sql, &[])?;
	Ok(rows)
}

pub fn all_hashmap(conn: &Connection) -> ::errors::Result<HashMap<i32, NeighborEntry>> {
	let rows = all(conn)?;
	let neighbors_iter = rows.into_iter().map(|r| {
		let entity_id = r.get(0);
		let mating_neighbors: Vec<i32> = r.get(1);
		let crowd_neighbors: Vec<i32> = r.get(2);

		Ok((entity_id, NeighborEntry { mating_neighbors, crowd_neighbors }))
	});

	::fallible_iterator::convert(neighbors_iter).collect()
}

pub fn recreate_all(conn: &Connection) -> ::errors::Result<HashMap<i32, NeighborEntry>> {
	let sql = r#"
	INSERT INTO entity_neighbors (entity_id, mating_neighbors, crowd_neighbors)
	  SELECT
	    e.id,
	    array_agg(n.id),
	    '{}'::int[] AS others
	  FROM entities_json e
	    INNER JOIN settings_json AS s ON e.setting_id = s.id
	    LEFT JOIN entities_json AS n
	      ON st_dwithin(e.point :: GEOGRAPHY, n.point :: GEOGRAPHY, (s.data -> 'mating_distance') :: TEXT :: NUMERIC)
	  WHERE e.id != n.id
	  GROUP BY e.id;

	WITH t AS (
	    SELECT
	      e.id            AS entity_id,
	      array_agg(c.id) AS crowd_neighbors
	    FROM entities_json e
	      INNER JOIN settings_json AS s ON e.setting_id = s.id
	      LEFT JOIN entities_json AS c
	        ON st_dwithin(e.point :: GEOGRAPHY, c.point :: GEOGRAPHY, (s.data -> 'crowd_distance') :: TEXT :: NUMERIC)
	    WHERE e.id != c.id
	    GROUP BY e.id
	)
	UPDATE entity_neighbors
	SET crowd_neighbors = t.crowd_neighbors
	FROM t
	WHERE entity_neighbors.entity_id = t.entity_id;
	"#;

	conn.batch_execute(&sql)?;
	all_hashmap(conn)
}

pub fn add_new_entity(conn: &Connection, entity_id: i32, point: &Point, crowd_distance: f32, mating_distance: f32) -> ::errors::Result<()> {
	let sql = "select id from entities_json where st_dwhitin(point::geography, st_setsrid(st_point($1, $2), 4326)::geography, $3)";
	let stmt = conn.prepare_cached(&sql)?;

	// Get mating neighbors for the given point and distance setting
	let rows = stmt.query(&[&point[0], &point[1], &mating_distance])?;
	let mating_neighbor_ids: Vec<i32> = rows.into_iter().map(|r| r.get(0)).collect();

	// Get crowd neighbors for the given point and distance setting
	let rows = stmt.query(&[&point[0], &point[1], &crowd_distance])?;
	let crowd_neighbor_ids: Vec<i32> = rows.into_iter().map(|r| r.get(0)).collect();

	// Add new entity id to all mating neighbors' neighbor entries
	let sql = "update entity_neighbors set mating_neighbors = mating_neighbors || $1 where entity_id in $2";
	let updated_rows = conn.execute(&sql, &[&entity_id, &mating_neighbor_ids])?;
	debug_assert_eq!(updated_rows, mating_neighbor_ids.len());

	// Add new entity id to all crowd neighbor' neighbor entries
	let sql = "update entity_neighbors set crowd_neighbors = crowd_neighbors || $1 where entity_id in $2";
	let updated_rows = conn.execute(&sql, &[&entity_id, &crowd_neighbor_ids])?;
	debug_assert_eq!(updated_rows, crowd_neighbor_ids.len());

	// Finally create new entity's its own neighbors entry
	let sql = "insert into entity_neighbors (entity_id, mating_neighbors, crowd_neighbors) values ($1, to_json($2::integer[]), to_json($3::integer[]))";
	conn.execute(&sql, &[&entity_id, &mating_neighbor_ids, &mating_neighbor_ids, &crowd_neighbor_ids])?;

	Ok(())
}

pub fn remove_entity(conn: &Connection, entity_id: i32, point: &Point, crowd_distance: f32, mating_distance: f32) -> ::errors::Result<()> {
	let sql = "update entity_neighbors set mating_neighbors = array_remove(mating_neighbors, $1), crowd_neighbors = array_remove(crowd_neighbors, $1)";
	let stmt = conn.prepare_cached(&sql)?;
	stmt.execute(&[&entity_id])?;

	let sql = "delete from entity_neighbors where entity_id = $1";
	let stmt = conn.prepare_cached(&sql)?;
	stmt.execute(&[&entity_id])?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_all_hashmap() {
		let conn = ::db::connect();
		let neighbors = all_hashmap(&conn).expect("Failed to fetch neighbors");
		assert_eq!(neighbors[&74006].crowd_neighbors, vec![75128, 76978, 74816, 75301]);
	}

	#[test]
	pub fn test_recreate_all() {
		let conn = ::db::connect();
		let neighbors = recreate_all(&conn).expect("Failed to fetch neighbors");
		assert_eq!(neighbors[&74006].crowd_neighbors, vec![75128, 76978, 74816, 75301]);
	}
}