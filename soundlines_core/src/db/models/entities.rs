use postgis::ewkb::Point;
use postgres::rows::Row;
use postgres::types::ToSql;
use geo::Point as GPoint;

use serde_json::Value;
use chrono::prelude::*;

use db::extensions::*;
use db::models::PlantSetting;
use db::models::Dna;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub point: Point,
    pub prefab: String,
    pub cell_id: i32,
    pub setting_id: i32,
    pub dna_id: i32,
    pub fitness: f32,
    pub age: f32,
    pub size: f32,
    pub life_expectancy: f32,
    pub nickname: String,
    pub mating: bool,
    pub start_mating_at: DateTime<Utc>,
    pub last_seed_at: DateTime<Utc>
}

impl Entity {
    pub fn new(location: Point, cell_id: i32, setting: &PlantSetting, dna: &Dna) -> Self {
        Self {
            id: -1,
            point: location,
            prefab: setting.prefab.clone(),
            cell_id,
            setting_id: setting.id.expect("Trying to create an entity with unsaved setting (has no id)"),
            dna_id: dna.id,
            fitness: dna.fitness,
            age: 0.0,
            size: dna.size,
            life_expectancy: dna.life_expectancy,
            nickname: "".to_string(),
            mating: false,
            start_mating_at: Utc::now(),
            last_seed_at: Utc::now()
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id as i64,
            "cell_id": self.cell_id as i64,
            "latitude": self.point.y,
            "longitude": self.point.x,
            "prefab": self.prefab.clone(),
            "setting_id": self.setting_id,
            "dna_id": self.dna_id,
            "fitness": self.fitness,
            "age": self.age,
            "size": self.size,
            "life_expectancy": self.life_expectancy,
            "nickname": &self.nickname,
            "mating": self.mating,
            "start_mating_at": self.start_mating_at.clone(),
            "last_seed_at": self.last_seed_at.clone(),
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
            point: row.get("point"),
            prefab: row.get("prefab"),
            cell_id: row.get("cell_id"),
            setting_id: row.get("setting_id"),
            dna_id: row.get("dna_id"),
            fitness: row.get("fitness"),
            age: row.get("age"),
            size: row.get("size"),
            life_expectancy: row.get("life_expectancy"),
            nickname: row.get("nickname"),
            mating: row.get("mating"),
            start_mating_at: row.get("start_mating_at"),
            last_seed_at: row.get("last_seed_at"),
        }
    }
    fn insert_fields() -> Vec<&'static str> { vec!["point", "prefab", "cell_id"] }
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![
            &self.point, 
            &self.prefab, 
            &self.cell_id,
            &self.setting_id,
            &self.dna_id,
            &self.fitness,
            &self.age,
            &self.size,
            &self.life_expectancy,
            &self.nickname,
            &self.mating,
            &self.start_mating_at,
            &self.last_seed_at,
        ]
    }
}
