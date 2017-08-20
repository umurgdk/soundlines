use rocket_contrib::Json;
use serde_json::Value;

use db::Result;
use db::extensions::*;
use db::models::*;
use db::guard::*;

#[post("/wifi", data = "<reading>")]
pub fn wifi(conn: DbConn, reading: Json<WifiReading>) -> Result<Json<WifiReading>> {
    let reading = conn.insert(&reading.0)?;
    Ok(Json(reading))
}

#[post("/sound", data = "<reading>")]
pub fn sound(conn: DbConn, reading: Json<SoundReading>) -> Result<Json<SoundReading>> {
    let reading = conn.insert(&reading.0)?;
    Ok(Json(reading))
}

#[post("/light", data = "<reading>")]
pub fn light(conn: DbConn, reading: Json<LightReading>) -> Result<Json<LightReading>> {
    let reading = conn.insert(&reading.0)?;
    Ok(Json(reading))
}

#[post("/gps", data = "<reading>")]
pub fn gps(conn: DbConn, reading: Json<GpsReadingJson>) -> Result<Json<Value>> {
    use serde_json;

    println!("{}", serde_json::to_string_pretty(&reading.0).unwrap());

    let gps_reading: GpsReading = reading.0.into_gps_reading(0);
    let gps_reading = conn.insert(&gps_reading)?;

    println!("Getting neighbors");
    let CellNeighbours { entities, cells, current_cell_id } =
        Cell::find_neighbors(&*conn, &gps_reading.point, 55.0)?;

    let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();
    let cells = cells.into_iter().map(|(_, c)| c.id as i64).collect::<Vec<_>>();

    Ok(Json(json!({
        "user_id": gps_reading.user_id as i64,
        "location": {
            "latitude": gps_reading.point.y,
            "longitude": gps_reading.point.x
        },
        "cell_id": current_cell_id as i64,
        "neighbor_cells": cells,
        "entities": entities
    })))
}
