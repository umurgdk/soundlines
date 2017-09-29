use rocket_contrib::Json;
use soundlines_core::db::Result as DbResult;
use soundlines_core::db::models::Weather;
use soundlines_core::db::extensions::*;
use db_guard::*;

#[get("/weather")]
pub fn get(conn: DbConn) -> DbResult<Json<Option<Weather>>> {
	conn.first::<Weather>().map(Json)
}