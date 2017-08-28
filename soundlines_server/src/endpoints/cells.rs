use rocket::State;
use rocket_contrib::Json;

use soundlines_core::db::Result;
use soundlines_core::db::models::Cell;
use soundlines_core::db::extensions::*;
use soundlines_core::postgis::ewkb::Polygon;
use soundlines_core::postgis::ewkb::Point;
use soundlines_core::map;

use serde_json::Value;
use super::state::*;
use db_guard::DbConn;

#[post("/recreate")]
pub fn recreate(conn: DbConn, parameters: State<ParametersManager>) -> Result<Json<Vec<Value>>> {
    conn.execute("delete from cells", &[])?;
    let new_cells = map::generate_cells(&*parameters.get());
    let stmt = conn.prepare("insert into cells (geom) values ($1) returning *")?;

    let mut cells = vec![];
    for cell in new_cells.into_iter() {
        let rows = stmt.query(&[&cell.geom])?;
        let row = rows.get(0);
        let polygon: Polygon = row.get("geom");
        cells.push(json!({
            "id": row.get::<_, i32>("id") as i64,
            "points": polygon.rings[0].points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()
        }));
    }

    Ok(Json(cells))
}

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json<Value>> {
    let cells = conn.all::<Cell>()?;
    let cells = cells.into_iter().map(Cell::into_json).collect::<Vec<_>>();

    Ok(Json(json!({
        "cells": cells
    })))
}

#[get("/<latitude>/<longitude>")]
pub fn cells_at(conn: DbConn, latitude: f64, longitude: f64) -> Result<Json<Value>> {
    use geo::Point as GPoint;
    let cell = Cell::find_containing(&*conn, &GPoint::new(longitude, latitude))?;
    Ok(Json(json!({
        "cell": cell.map(Cell::into_json)
    })))
}
