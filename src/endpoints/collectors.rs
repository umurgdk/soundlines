use std::error::Error;
use std::net::SocketAddr;

use rocket_contrib::Json;

use serde_json::Value;

use db::Result;
use db::models::WifiReading;
use db::models::SoundReading;
use db::models::LightReading;
use db::models::GpsReading;
use db::models::GpsReadingJson;
use db::guard::*;

use chrono::prelude::*;

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
    let position = ewkb::Point::new(reading.0.longitude, reading.0.latitude, Some(4326));
    let rows = conn.query("INSERT INTO gps_readings (user_id, created_at, point) values ($1, $2, $3) returning *",
        &[&1, &Utc::now(), &position])?;

    let row = rows.get(0);
    let point: ewkb::Point = row.get("point");

    let cells = {
        let rows = conn.query("SELECT id, geom, ST_Contains(geom, ST_SetSRID(ST_Point($1, $2),4326)) as inside FROM cells WHERE ST_DWithin(geom::geography, ST_SetSRID(ST_Point($1, $2),4326)::geography, 55)",
            &[&reading.longitude, &reading.latitude])?;

        rows.into_iter().map(|row| {
            row.get::<_, i32>("id") as i64
        }).collect::<Vec<_>>()
    };

    Ok(Json(json!({
        "echo": {
            "id": row.get::<_, i32>("id"),
            "user_id": row.get::<_, i32>("user_id"),
            "created_at": row.get::<_, DateTime<Utc>>("created_at"),
            "latitude": point.y,
            "longitude": point.x
        },

        "others": [],
        "cells": cells,
        "entities": []
    })))
}
