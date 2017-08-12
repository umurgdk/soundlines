use rocket::State;
use rocket_contrib::Json;
use diesel::*;

use db::DbConn;
use db::models::Cell;

use super::state::*;
use map;

#[post("/recreate")]
pub fn recreate(conn: DbConn, parameters: State<ParametersManager>) -> QueryResult<Json<u8>> {
    unimplemented!();
    //let new_cells = map::generate_cells(&*parameters.get());

    //delete(cells::table).execute(&*conn).expect("Failed to delete old cells...");
    //insert(&new_cells).into(cells::table).get_results(&*conn).map(Json)
}

#[get("/")]
pub fn index(conn: DbConn) -> QueryResult<Json<u8>> {
    unimplemented!();
    //cells::table.load(&*conn).map(Json)
}
