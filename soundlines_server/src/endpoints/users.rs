use rocket_contrib::Json;
use serde_json::Value;

use rocket_extensions::*;

use soundlines_core::db::Result;
use soundlines_core::db::models::GpsReading;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::CellNeighbours;
use soundlines_core::db::extensions::*;

use db_guard::*;

#[get("/location")]
pub fn location(conn: DbConn) -> Result<Json> {
    let gps_readings = GpsReading::last_gps_readings_by_user(&*conn)?;

	let mut locations = vec![];
	for gps_reading in gps_readings.into_iter() {
		let CellNeighbours { cells, entities, current_cell_id } =
			Cell::find_neighbors(&conn, &gps_reading.point, 55.0)?;

		let neighbors = cells.into_iter().map(|(_, c)| c.id as i64).collect::<Vec<_>>();
		let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();

		locations.push(json!({
            "user_id": gps_reading.user_id,
            "location": {
                "latitude": gps_reading.point.y,
                "longitude": gps_reading.point.x
            },
            "cell_id": current_cell_id,
            "neighbor_cells": neighbors,
            "entities": entities
        }));
	}

	Ok(Json(json!({
		"locations": locations
	})))
}

#[get("/location/times")]
pub fn location_times(conn: DbConn) -> Result<Json> {
    let gps_readings = conn.all::<GpsReading>()?;
    Ok(Json(json!({
        "times": gps_readings.into_iter().map(|r| r.created_at).collect::<Vec<_>>()
    })))
}

#[get("/location/<since>/<until>")]
pub fn location_range(conn: DbConn, since: DateTimeUtc, until: DateTimeUtc) -> Result<Json> {
    let gps_readings = GpsReading::get_time_range(&conn, &since, &until)?;

    let mut locations: Vec<Value> = Vec::with_capacity(gps_readings.len());
    for gps_reading in gps_readings.into_iter() {
        let CellNeighbours { cells, entities, current_cell_id } =
            Cell::find_neighbors(&conn, &gps_reading.point, 55.0)?;

        let neighbors = cells.into_iter().map(|(_, c)| c.id as i64).collect::<Vec<_>>();
        let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();

        locations.push(json!({
            "user_id": gps_reading.user_id,
            "location": {
                "latitude": gps_reading.point.y,
                "longitude": gps_reading.point.x
            },
            "cell_id": current_cell_id,
            "neighbor_cells": neighbors,
            "entities": entities
        }));
    }

    Ok(Json(json!({
        "locations": locations
    })))
}
