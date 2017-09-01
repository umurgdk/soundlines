use std::collections::HashMap;
use rocket_contrib::Json;
use serde_json::Value;

use soundlines_core::db::Result;
use soundlines_core::db::extensions::*;
use soundlines_core::db::models::*;

use db_guard::*;
use user::Auth;
use user::JsonUserIdCheck;

#[derive(Deserialize, Serialize)]
pub struct WifiReadingsPayload {
    latitude: f64,
    longitude: f64,
    readings: Vec<WifiReadingJson>
}

#[post("/wifi", data = "<payload>")]
pub fn wifi(auth: Auth, conn: DbConn, mut payload: Json<WifiReadingsPayload>) -> Result<Json<Vec<WifiReadingJson>>> {
    let user = auth.into_user();
    let user_id = user.id;

    let WifiReadingsPayload { latitude, longitude, readings } = payload.into_inner();
    let readings: Vec<_> = readings.into_iter().map(|mut r| {
        r.latitude = latitude;
        r.longitude = longitude;

        r.into_wifi_reading(user_id)
    }).collect();

    let readings: Vec<_> = conn.insert_batch_return(&readings, true)?.into_iter().map(|r| r.into_wifi_reading_json()).collect();
    Ok(Json(readings))
}

#[post("/sound", data = "<reading>")]
pub fn sound(auth: Auth, conn: DbConn, mut reading: Json<SoundReading>) -> Result<Json<SoundReading>> {
    let user = auth.into_user();
    reading.0.user_id = user.id;

    let reading = conn.insert(&reading.0)?;
    Ok(Json(reading))
}

#[post("/light", data = "<reading>")]
pub fn light(auth: Auth, conn: DbConn, mut reading: Json<LightReading>) -> Result<Json<LightReading>> {
    let user = auth.into_user();
    reading.0.user_id = user.id;

    let reading = conn.insert(&reading.0)?;
    Ok(Json(reading))
}

#[post("/gps", data = "<reading>")]
pub fn gps(auth: Auth, conn: DbConn, mut reading: Json<GpsReadingJson>) -> Result<Json<Value>> {
    use serde_json;

    let user = auth.into_user();
    reading.0.user_id = user.id;

    let other_users = User::get_all_locations(&*conn, Some(user.id))?;
    let prefabs = conn.all::<PlantSetting>()?
        .into_iter()
        .map(|s| (s.id.unwrap(), s.prefab))
        .collect::<HashMap<_, _>>();

    let gps_reading: GpsReading = reading.0.into_gps_reading(0);
    let gps_reading = conn.insert(&gps_reading)?;

    println!("Getting neighbors");
    let CellNeighbours { entities, cells, seeds, current_cell_id } =
        Cell::find_neighbors(&*conn, &gps_reading.point, 55.0)?;

    let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();
    let seeds = seeds
        .into_iter()
        .map(Seed::into_json)
        .collect::<Vec<_>>();

    let cells = cells.into_iter().map(|(_, c)| c.id as i64).collect::<Vec<_>>();

    Ok(Json(json!({
        "user_id": gps_reading.user_id as i64,
        "location": {
            "latitude": gps_reading.point.y,
            "longitude": gps_reading.point.x
        },
        "others": other_users,
        "cell_id": current_cell_id as i64,
        "neighbor_cells": cells,
        "entities": entities,
        "seeds": seeds
    })))
}
