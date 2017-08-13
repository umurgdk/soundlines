use rocket::State;
use rocket_contrib::Json;

use db::DbConn;
use db::models::Cell;
use db::Result;

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
    let cells = conn.query("select * from cells", &[])?.into_iter().map(|row| {
        let polygon: Polygon = row.get("geom");
        json!({
            "id": row.get::<_, i32>("id") as i64,
            "points": polygon.rings[0].points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()
        })
    }).collect::<Vec<_>>();

    Ok(Json(json!({
        "cells": cells
    })))
}

#[get("/<latitude>/<longitude>")]
pub fn cells_at(conn: DbConn, latitude: f64, longitude: f64) -> Result<Json> {
    let cells = {
        let rows = conn.query("SELECT id, geom, ST_Contains(geom, ST_SetSRID(ST_Point($1, $2),4326)) as inside FROM cells WHERE ST_DWithin(geom::geography, ST_SetSRID(ST_Point($1, $2),4326)::geography, 55)",
            &[&longitude, &latitude])?;

        rows.into_iter().map(|row| {
            let id = row.get::<_, i32>("id") as i64;

            if row.get("inside") {
                println!("Inside: {}", id);
            }

            id
        }).collect::<Vec<_>>()
    };

    Ok(Json(json!({
        "cells": cells
    })))
}
