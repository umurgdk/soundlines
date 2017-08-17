use serde_json::Value;
use postgis::ewkb::Point;
use rocket_contrib::Json;

use db::DbConn;
use db::Result;

#[get("/location")]
pub fn location(conn: DbConn) -> Result<Json> {
    let rows = conn.query("select * from gps_readings ORDER BY created_at DESC LIMIT 1", &[])?;

    let gps_row = rows.get(0);
    let point: Point = gps_row.get("point");

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
        "locations": [{
            "user_id": gps_row.get::<_, i32>("user_id") as i64,
            "location": {
                "latitude": point.y,
                "longitude": point.x
            },
            "cell_id": cell_id,
            "neighbor_cells": neighbor_ids,
            "entities": entities
        }]
    })))
}
