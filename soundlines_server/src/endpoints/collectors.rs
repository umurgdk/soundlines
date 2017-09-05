use std::collections::HashMap;

use rocket_contrib::Json;
use serde_json::Value;
use chrono::prelude::*;

use soundlines_core::db::Result;
use soundlines_core::db::extensions::*;
use soundlines_core::db::models::*;
use soundlines_core::postgis::ewkb::Point;

use db_guard::*;
use user::Auth;
use user::JsonUserIdCheck;

#[derive(Deserialize, Serialize)]
pub struct WifiReadingsPayload {
    latitude: f64,
    longitude: f64,
    wifi_items: Vec<WifiReadingJson>
}

#[post("/wifi", data = "<payload>")]
pub fn wifi(auth: Auth, conn: DbConn, mut payload: Json<WifiReadingsPayload>) -> Result<Option<Json<Vec<WifiReadingJson>>>> {
    let user = auth.into_user();
    let user_id = user.id;

    let WifiReadingsPayload { latitude, longitude, wifi_items } = payload.into_inner();
    let readings: Vec<_> = wifi_items.into_iter().map(|mut r| {
        r.latitude = latitude;
        r.longitude = longitude;

        r.into_wifi_reading(user_id)
    }).collect();

    let mut cell = match Cell::find_containing_core(&*conn, &Point::new(longitude, latitude, Some(4326)))? {
        Some(cell) => cell,
        None       => return Ok(None)
    };

    let readings: Vec<_> = conn.insert_batch_return(&readings, true)?.into_iter().map(|r| r.into_wifi_reading_json()).collect();

    for reading in readings.iter() {
        cell.wifi_total += reading.level;
        cell.wifi_count += 1.0;
    }

    cell.wifi = cell.wifi_total / cell.wifi_count;
    
    conn.update(cell.id, &cell)?;

    Ok(Some(Json(readings)))
}

#[derive(Deserialize, Serialize)]
pub struct SoundReadingPayload {
    latitude: f64,
    longitude: f64,
    level: f32
}

#[post("/sound", data = "<payload>")]
pub fn sound(auth: Auth, conn: DbConn, mut payload: Json<SoundReadingPayload>) -> Result<Option<Json<SoundReading>>> {
    let user = auth.into_user();
    let payload = payload.into_inner();

    let reading = SoundReading {
        id: None,
        user_id: user.id,
        created_at: Utc::now(),
        level: payload.level,
        point: Point::new(payload.longitude, payload.latitude, Some(4326))
    };

    let mut cell = match Cell::find_containing_core(&*conn, &reading.point)? {
        Some(cell) => cell,
        None       => return Ok(None)
    };
    
    let reading = conn.insert(&reading)?;

    cell.sound_total += payload.level;
    cell.sound_count += 1.0;
    cell.sound = cell.sound_total / cell.sound_count;

    conn.update(cell.id, &cell)?;

    Ok(Some(Json(reading)))
}

#[derive(Deserialize, Serialize)]
pub struct LightReadingPayload {
    latitude: f64,
    longitude: f64,
    level: f32
}

#[post("/light", data = "<payload>")]
pub fn light(auth: Auth, conn: DbConn, mut payload: Json<LightReadingPayload>) -> Result<Option<Json<LightReading>>> {
    let user = auth.into_user();
    let payload = payload.into_inner();

    let reading = LightReading {
        id: None,
        user_id: user.id,
        created_at: Utc::now(),
        level: payload.level,
        point: Point::new(payload.longitude, payload.latitude, Some(4326))
    };

    let mut cell = match Cell::find_containing_core(&*conn, &reading.point)? {
        Some(cell) => cell,
        None       => return Ok(None)
    };

    let reading = conn.insert(&reading)?;

    cell.light_total = payload.level;
    cell.light_count += 1.0;
    cell.light = cell.light_total / cell.light_count;

    Ok(Some(Json(reading)))
}

#[post("/gps", data = "<reading>")]
pub fn gps(auth: Auth, conn: DbConn, reading: Json<GpsReadingJson>) -> Result<Json<Value>> {
    use serde_json;

    let user = auth.into_user();
    let mut reading = reading.into_inner();
    reading.user_id = user.id;

    let other_users = User::get_all_locations(&*conn, Some(user.id))?;
    let prefabs = conn.all::<PlantSetting>()?
        .into_iter()
        .map(|s| (s.id.unwrap(), s.prefab))
        .collect::<HashMap<_, _>>();

    let gps_reading: GpsReading = reading.into_gps_reading(0);
    let CellNeighbours { entities, cells, seeds, current_cell_id } =
        Cell::find_neighbors(&*conn, &gps_reading.point, 55.0)?;

    let same_cell = conn.last::<GpsReading>()?
        .and_then(|r| r.get_cell(&*conn).ok().and_then(|c| c))
        .map(|c| c.id == current_cell_id)
        .unwrap_or(false);

    let gps_reading = conn.insert(&gps_reading)?;

    let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();
    let seeds = seeds
        .into_iter()
        .map(Seed::into_json)
        .collect::<Vec<_>>();

    let mut cell_to_update_visits = None;
    let cells = cells.into_iter().map(|(_, c)| {
        let cell_id = c.id;
        if c.id == current_cell_id && !same_cell {
            cell_to_update_visits = Some(c);
        }

        cell_id as i64
    }).collect::<Vec<_>>();

    cell_to_update_visits.map(|mut cell| {
        cell.visit += 1;
        conn.update(cell.id, &cell)
    }).unwrap_or(Ok(()))?;

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
