use std::error::Error;
use std::net::SocketAddr;

use rocket_contrib::Json;
use serde_json::Value;
use chrono::prelude::*;
use postgis::ewkb::Point;

use db::Result;
use db::models::WifiReading;
use db::models::SoundReading;
use db::models::LightReading;
use db::models::GpsReading;
use db::models::GpsReadingJson;
use db::guard::*;

#[post("/wifi", data = "<reading>")]
pub fn wifi(conn: DbConn, reading: Json<WifiReading>) -> Result<Json<WifiReading>> {
    let rows = conn.query("INSERT INTO wifi_readings (user_id, created_at, ssid, level, frequency) values ($1, $2, $3, $4, $5) returning *",
        &[&1, &Utc::now(), &reading.ssid, &reading.level, &reading.frequency])?;

    let row = rows.get(0);
    Ok(Json(WifiReading {
        id: Some(row.get("id")),
        user_id: row.get("user_id"),
        created_at: row.get("created_at"),
        ssid: row.get("ssid"),
        level: row.get("level"),
        frequency: row.get("frequency")
    }))
}

#[post("/sound", data = "<reading>")]
pub fn sound(conn: DbConn, reading: Json<SoundReading>) -> Result<Json<SoundReading>> {
    let rows = conn.query("INSERT INTO sound_readings (user_id, created_at, level) values ($1, $2, $3) returning *",
        &[&1, &Utc::now(), &reading.level])?;

    let row = rows.get(0);
    Ok(Json(SoundReading {
        id: Some(row.get("id")),
        user_id: row.get("user_id"),
        created_at: row.get("created_at"),
        level: row.get("level")
    }))
}

#[post("/light", data = "<reading>")]
pub fn light(conn: DbConn, reading: Json<LightReading>) -> Result<Json<LightReading>> {
    let rows = conn.query("INSERT INTO light_readings (user_id, created_at, level) values ($1, $2, $3) returning *",
        &[&1, &Utc::now(), &reading.level])?;

    let row = rows.get(0);
    Ok(Json(LightReading {
        id: Some(row.get("id")),
        user_id: row.get("user_id"),
        created_at: row.get("created_at"),
        level: row.get("level")
    }))
}

//use postgis::*;
use postgis::ewkb;
use postgis::ewkb::Polygon;
use postgis::ewkb::AsEwkbPoint;
use db::helpers::*;
#[post("/gps", data = "<reading>")]
pub fn gps(conn: DbConn, reading: Json<GpsReadingJson>) -> Result<Json<Value>> {
    use serde_json;
    println!("{}", serde_json::to_string_pretty(&reading.0).unwrap_or("Json parsing error".to_string()));

    let position = ewkb::Point::new(reading.0.longitude, reading.0.latitude, Some(4326));
    let rows = conn.query("INSERT INTO gps_readings (user_id, created_at, point) values ($1, $2, $3) returning *",
        &[&1, &Utc::now(), &position])?;

    let gps_row = rows.get(0);
    let point: ewkb::Point = gps_row.get("point");

    let rows = conn.query("select id, geom from cells where ST_Contains(geom, $1) LIMIT 1", &[&point])?;
    let cell_id = rows.get(0).get::<_, i32>("id") as i64;

    let neighbor_rows = conn.query("SELECT id, geom, ST_Contains(geom, ST_SetSRID(ST_Point($1, $2),4326)) as inside FROM cells WHERE ST_DWithin(geom::geography, ST_SetSRID(ST_Point($1, $2),4326)::geography, 55)",
            &[&point.x, &point.y])?;
    let neighbor_ids = neighbor_rows.into_iter()
        .map(|row| row.get::<_, i32>("id"))
        .collect::<Vec<_>>();

    let entities = conn.query("select entities.id, entities.point, entities.prefab, cells.id as cell_id from entities inner join cells on ST_Contains(cells.geom, entities.point) where cells.id = any($1)", &[&neighbor_ids])?;

    let neighbor_ids = neighbor_ids.into_iter()
        .map(|id| id as i64)
        .filter(|id| *id != cell_id)
        .collect::<Vec<_>>();

    let entities = entities.into_iter().map(|e| {
        let point: Point = e.get("point");

        json!({
            "id": e.get::<_, i32>("id") as i64,
            "cell_id": e.get::<_, i32>("cell_id") as i64,
            "latitude": point.y,
            "longitude": point.x,
            "prefab": e.get::<_, String>("prefab")
        })
    }).collect::<Vec<_>>();

    Ok(Json(json!({
        "user_id": gps_row.get::<_, i32>("user_id") as i64,
        "location": {
            "latitude": point.y,
            "longitude": point.x
        },
        "cell_id": cell_id,
        "neighbor_cells": neighbor_ids,
        "entities": entities
    })))
}
