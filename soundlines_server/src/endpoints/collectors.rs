use rocket::response::status;
use rocket_contrib::Json;
use serde_json::Value;
use chrono::prelude::*;

use soundlines_core::db::Result;
use soundlines_core::db::extensions::*;
use soundlines_core::db::models::*;
use soundlines_core::postgis::ewkb::Point;

use db_guard::*;
use user::Auth;

#[derive(Deserialize, Serialize)]
pub struct WifiReadingsPayload {
    latitude: f64,
    longitude: f64,
    wifi_items: Vec<WifiReadingJson>
}

#[post("/wifi", data = "<payload>")]
pub fn wifi(auth: Auth, conn: DbConn, payload: Json<WifiReadingsPayload>) -> Result<status::NoContent> {
    let user = auth.into_user();
    let user_id = user.id;

    let WifiReadingsPayload { latitude, longitude, wifi_items } = payload.into_inner();
    let readings: Vec<_> = wifi_items.into_iter().map(|mut r| {
        r.latitude = latitude;
        r.longitude = longitude;

        r.into_wifi_reading(user_id)
    }).collect();

    match Cell::find_containing_core(&*conn, &Point::new(longitude, latitude, Some(4326)))? {
        Some(mut cell) => {
            for reading in readings.iter() {
                cell.wifi_total += reading.level;
                cell.wifi_count += 1.0;
            }

            cell.wifi = cell.wifi_total / cell.wifi_count;
            conn.update(cell.id, &cell)?;
        },

        None => return Ok(status::NoContent)
    };

    Ok(status::NoContent)
}

#[derive(Deserialize, Serialize)]
pub struct SoundReadingPayload {
    latitude: f64,
    longitude: f64,
    level: f32
}

#[post("/sound", data = "<payload>")]
pub fn sound(auth: Auth, conn: DbConn, payload: Json<SoundReadingPayload>) -> Result<status::NoContent> {
    let user = auth.into_user();
    let payload = payload.into_inner();

    let reading = SoundReading {
        id: None,
        user_id: user.id,
        created_at: Utc::now(),
        level: payload.level,
        point: Point::new(payload.longitude, payload.latitude, Some(4326))
    };

    match Cell::find_containing_core(&*conn, &reading.point)? {
        Some(mut cell) => {
            cell.sound_total += payload.level;
            cell.sound_count += 1.0;
            cell.sound = cell.sound_total / cell.sound_count;

            conn.update(cell.id, &cell)?;
        },
        None => return Ok(status::NoContent)
    };

    conn.insert(&reading)?;
    Ok(status::NoContent)
}

#[derive(Deserialize, Serialize)]
pub struct LightReadingPayload {
    latitude: f64,
    longitude: f64,
    level: f32
}

#[post("/light", data = "<payload>")]
pub fn light(auth: Auth, conn: DbConn, payload: Json<LightReadingPayload>) -> Result<status::NoContent> {
    let user = auth.into_user();
    let payload = payload.into_inner();

    let reading = LightReading {
        id: None,
        user_id: user.id,
        created_at: Utc::now(),
        level: payload.level,
        point: Point::new(payload.longitude, payload.latitude, Some(4326))
    };

    match Cell::find_containing_core(&*conn, &reading.point)? {
        Some(mut cell) => {
            cell.light_total += reading.level;
            cell.light_count += 1.0;
            cell.light = cell.light_total / cell.light_count;

            conn.update(cell.id, &cell)?;
        },
        None => return Ok(status::NoContent)
    };

    conn.insert(&reading)?;
    Ok(status::NoContent)
}

#[post("/gps", data = "<reading>")]
pub fn gps(auth: Auth, conn: DbConn, reading: Json<GpsReadingJson>) -> Result<Option<Json>> {
    let user = auth.into_user();
    let mut reading = reading.into_inner();
    reading.user_id = user.id;

    let other_users = User::get_all_locations(&*conn, Some(user.id))?;

    let gps_reading: GpsReading = reading.into_gps_reading(0);
    let CellNeighbours { entities, cells, seeds, current_cell_id } =
        Cell::find_neighbors(&*conn, &gps_reading.point, 120.0)?;

    if cells.len() == 0 {
        return Ok(Some(Json(json!({
            "user_id": gps_reading.user_id as i64,
	        "location": {
	            "latitude": gps_reading.point.y,
	            "longitude": gps_reading.point.x
	        },
	        "others": other_users,
	        "cell_id": current_cell_id as i64,
	        "neighbor_cells": cells.iter().map(|(id, _)| *id).collect::<Vec<i32>>(),
	        "entities": [],
	        "seeds": []
        }))));
    }

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

    Ok(Some(Json(json!({
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
    }))))
}
