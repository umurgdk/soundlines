use postgres::rows::Row;
use postgres::types::ToSql;

use db::extensions::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlantSetting {
    pub id: Option<i32>,
    pub name: String,
    pub prefab: String,
    pub growth_limit: f32,
    pub life_expectancy: f32,
    pub wifi_sensitivity: f32,
    pub light_sensitivity: f32,
    pub sound_sensitivity: f32,
    pub neighbor_tolerance: f32,
    pub birth_proba: f32,
    pub bloom_proba: f32,
    pub mating_freq: f32,
    pub mating_duration: f32,
    pub fruit_duration: f32,
    pub mating_distance: f32,
    pub crowd_distance: f32
}

impl SqlType for PlantSetting {
    fn table_name() -> &'static str { "settings" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            prefab: row.get("prefab"),
            growth_limit: row.get("growth_limit"),
            life_expectancy: row.get("life_expectancy"),
            wifi_sensitivity: row.get("wifi_sensitivity"),
            light_sensitivity: row.get("light_sensitivity"),
            sound_sensitivity: row.get("sound_sensitivity"),
            neighbor_tolerance: row.get("neighbor_tolerance"),
            birth_proba: row.get("birth_proba"),
            bloom_proba: row.get("bloom_proba"),
            mating_freq: row.get("mating_freq"),
            mating_duration: row.get("mating_duration"),
            fruit_duration: row.get("fruit_duration"),
            mating_distance: row.get("mating_distance"),
            crowd_distance: row.get("crowd_distance")
        }
    }

    fn insert_fields() -> Vec<&'static str> { 
        vec![ 
            "id",
            "name",
            "prefab",
            "growth_limit",
            "life_expectancy",
            "wifi_sensitivity",
            "light_sensitivity",
            "sound_sensitivity",
            "neighbor_tolerance",
            "birth_proba",
            "bloom_proba",
            "mating_freq",
            "mating_duration",
            "fruit_duration",
            "mating_distance",
            "crowd_distance"
        ]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![
            &self.id,
            &self.name,
            &self.prefab,
            &self.growth_limit,
            &self.life_expectancy,
            &self.wifi_sensitivity,
            &self.light_sensitivity,
            &self.sound_sensitivity,
            &self.neighbor_tolerance,
            &self.birth_proba,
            &self.bloom_proba,
            &self.mating_freq,
            &self.mating_duration,
            &self.fruit_duration,
            &self.mating_distance,
            &self.crowd_distance
        ]
    }
}
