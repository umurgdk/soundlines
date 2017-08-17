use serde_json::Value;
use postgis::ewkb::Point;
use rocket_contrib::Json;

use db::DbConn;
use db::Result;

#[post("/generate")]
pub fn generate(conn: DbConn) -> Result<&'static str> {
    conn.execute("delete from entities", &[])?;

    let rows = conn.query("insert into entities (point) select b.* 
                          from cells
                          CROSS JOIN LATERAL (
                                select ST_SetSRID(ST_MakePoint(tmp.x, tmp.y), 4326) geom
                                from (
                                        select
                                            random() * (ST_XMax(cells.geom) - ST_XMin(cells.geom)) + ST_XMin(cells.geom) x,
                                            random() * (ST_YMax(cells.geom) - ST_YMin(cells.geom)) + ST_YMin(cells.geom) y
                                        from generate_series(0, 4)
                                    ) tmp
                         ) b
                         
                         LIMIT 2500", &[])?;

    
    Ok("")
}

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json> {
    let entities = conn.query("select * from entities", &[])?;
    let entities = entities.into_iter().map(|e| {
        let point: Point = e.get("point");
        json!({
            "id": e.get::<_, i32>("id") as i64,
            "latitude": point.y,
            "longitude": point.x,
            "prefab": e.get::<_, String>("prefab")
        })
    }).collect::<Vec<_>>();

    Ok(Json(json!({
        "entities": entities
    })))
}
