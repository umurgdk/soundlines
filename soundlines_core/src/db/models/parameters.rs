use postgres::rows::Row;
use postgres::types::ToSql;

use db::extensions::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub id: Option<i32>,
    pub cell_size: i32
}

impl SqlType for Parameter {
    fn table_name() -> &'static str { "parameters" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: Some(row.get("id")),
            cell_size: row.get("cell_size")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec!["cell_size"]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.cell_size ]
    }
}
