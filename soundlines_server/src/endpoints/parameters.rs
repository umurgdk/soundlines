use rocket::State;
use rocket_contrib::Json;
use serde_json::Value;

use soundlines_core::db::extensions::*;
use soundlines_core::db::models::Parameter;
use db_guard::*;

use super::state::*;

#[get("/parameters")]
pub fn get_parameters(parameters: State<ParametersManager>) -> Json<Parameter> {
    parameters.to_json()
}

#[post("/parameters", data = "<new_parameters>")]
pub fn update_parameters(conn: DbConn, new_parameters: Json<Value>, parameters: State<ParametersManager>) -> Json<Parameter> {
    let new_parameters = new_parameters.0;

    parameters.update_with(&*conn, |p| {
        p.cell_size = new_parameters["cell_size"]
            .as_i64()
            .map(|i| i as i32)
            .unwrap_or(p.cell_size);
    });

    parameters.to_json()
}
