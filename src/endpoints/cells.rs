use rocket::State;
use rocket_contrib::Json;

use db::DbConn;
use db::Result;
use db::models::Cell;
use db::extensions::*;

use serde_json::Value;
use super::state::*;
use map;

use postgis::ewkb::Polygon;

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
    let cells: Vec<Cell> = conn.all()?;
    let cells = cells.into_iter().map(Cell::into_json).collect::<Vec<_>>();

    Ok(Json(json!({
        "cells": cells
    })))
}

#[get("/<latitude>/<longitude>")]
pub fn cells_at(conn: DbConn, latitude: f64, longitude: f64) -> Result<Json<Value>> {
    use postgis::ewkb::Point;

    let cell = Cell::find_containing(&*conn, &Point::new(longitude, latitude, Some(4326)))?;
    Ok(Json(json!({
        "cell": cell.map(Cell::into_json)
    })))
}
