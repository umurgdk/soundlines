use std::result::Result as StdResult;

use rocket::response::status;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

use soundlines_core::db::Result;
use soundlines_core::db::extensions::*;
use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Entity;

use db_guard::*;

#[post("/generate")]
pub fn generate(conn: DbConn) -> Result<&'static str> {
    conn.execute("delete from entities", &[])?;

    conn.execute(r#"
        insert into entities (point) select b.*
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

        LIMIT 2500
    "#, &[])?;

    conn.execute(r#"
        update entities set cell_id = cell.id
        from ( select id, geom from cells ) cell
        where st_contains(cell.geom, point)
    "#, &[])?;
    
    Ok("")
}

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json> {
    let entities: Vec<Entity> = conn.all()?;
    let entities = entities.into_iter().map(Entity::into_json).collect::<Vec<_>>();

    Ok(Json(json!({
        "entities": entities
    })))
}

#[delete("/<id>")]
pub fn delete(conn: DbConn, id: i32) -> StdResult<status::NoContent, Failure> {
    let entity = conn.get::<Entity>(id)
	    .map_err(|_| Failure(Status::InternalServerError))?
        .ok_or(Failure(Status::BadRequest))?;

    conn.delete::<Entity>(id).map_err(|_| Failure(Status::InternalServerError))?;
    conn.delete::<Dna>(entity.dna_id).map_err(|_| Failure(Status::InternalServerError))?;

	Ok(status::NoContent)
}
