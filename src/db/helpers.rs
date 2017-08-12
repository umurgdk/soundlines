use diesel::prelude::*;
use diesel::types::FromSqlRow;

use super::Connection;
use super::models::Parameter;
use super::schema::parameters;

impl Parameter {
    pub fn fetch(conn: &Connection) -> QueryResult<Parameter> {
        parameters::table.find(0).first(&*conn)
    }
}


