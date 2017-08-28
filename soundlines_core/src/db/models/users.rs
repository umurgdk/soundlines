use postgres::rows::Row;
use postgres::types::ToSql;
use chrono::prelude::*;

use db::extensions::*;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>
}

impl SqlType for User {
    fn table_name() -> &'static str { "users" }

    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self { id: row.get("id"), created_at: row.get("created_at") }
    }

    fn insert_fields() -> Vec<&'static str> { vec!["created_at"] }
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> { vec![&self.created_at] }
}
