use postgres::rows::Row;
use postgres::types::ToSql;
use postgis::ewkb::Point;
use chrono::prelude::*;
use serde_json::Value as JValue;

use db::extensions::*;

#[derive(Debug, Clone)]
pub struct Seed {
    pub id: Option<i32>,
    pub cell_id: i32,
    pub dna_id: i32,
    pub setting_id: i32,
    pub point: Point,
    pub created_at: DateTime<Utc>,
    pub age: f32,
    pub prefab: String
}

impl Seed {
    pub fn new(dna_id: i32, location: Point, cell_id: i32, setting_id: i32, prefab: String) -> Self {
        Self {
            id: None,
            cell_id,
            dna_id,
            setting_id,
            point: location,
            created_at: Utc::now(),
            age: 1.0,
            prefab,
        }
    }

    pub fn into_json(self) -> JValue {
        json!({
            "id": self.id,
            "cell_id": self.cell_id,
            "dna_id": self.dna_id,
            "setting_id": self.setting_id,
            "latitude": self.point.y,
            "longitude": self.point.x,
            "age": self.age,
            "prefab": self.prefab
        })
    }
}

impl SqlType for Seed {
    fn table_name() -> &'static str { "seeds" }

    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self { 
            id: Some(row.get("id")),
            cell_id: row.get("cell_id"),
            dna_id: row.get("dna_id"),
            setting_id: row.get("setting_id"),
            point: row.get("point"),
            created_at: row.get("created_at"),
            age: row.get("age"),
            prefab: row.get("prefab")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec![ "cell_id", "dna_id", "setting_id", "point", "created_at", "age", "prefab" ]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.cell_id, &self.dna_id, &self.setting_id, &self.point, &self.created_at, &self.age, &self.prefab ]
    }
}
