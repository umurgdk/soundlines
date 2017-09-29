use rocket_contrib::Json;

use soundlines_core::db::Result;
use soundlines_core::db::models::Cell;
use soundlines_core::db::extensions::*;

use serde_json::Value;
use db_guard::DbConn;

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json<Value>> {
    let cells = conn.all::<Cell>()?;
    let cells = cells.into_iter().map(Cell::into_json).collect::<Vec<_>>();

    Ok(Json(json!({
        "cells": cells
    })))
}

#[get("/<id>")]
pub fn show(conn: DbConn, id: i32) -> Result<Option<Json<Value>>> {
    let cell = conn.get::<Cell>(id)?;

    Ok(cell.map(|c| Json(c.to_json())))
}

#[get("/<latitude>/<longitude>")]
pub fn cells_at(conn: DbConn, latitude: f64, longitude: f64) -> Result<Json<Value>> {
    use geo::Point as GPoint;
    let cell = Cell::find_containing(&*conn, &GPoint::new(longitude, latitude))?;
    Ok(Json(json!({
        "cell": cell.map(Cell::into_json)
    })))
}
