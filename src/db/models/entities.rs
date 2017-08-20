use postgis::ewkb::Point;
use postgres::rows::Row;
use postgres::types::ToSql;
use serde_json::Value;

use db::extensions::*;

#[derive(Debug)]
pub struct Entity {
    pub id: i32,
    pub point: Point,
    pub prefab: String,
    pub cell_id: i32
}

impl Entity {
    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id as i64,
            "cell_id": self.cell_id as i64,
            "latitude": self.point.y,
            "longitude": self.point.x,
            "prefab": self.prefab.clone()
        })
    }

    pub fn into_json(self) -> Value {
        self.to_json()
    }
}

impl SqlType for Entity {
    fn table_name() -> &'static str { "entities" }

    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: row.get("id"),
            cell_id: row.get("cell_id"),
            point: row.get("point"),
            prefab: row.get("prefab")
        }
    }
    fn insert_fields() -> Vec<&'static str> { vec!["point", "prefab", "cell_id"] }
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.point, &self.prefab, &self.cell_id]
    }
}