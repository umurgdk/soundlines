use postgres::rows::Row;
use postgres::types::ToSql;

use db::extensions::*;

#[derive(Debug, Clone, Serialize)]
pub struct Dna {
    #[serde(default)]
    pub id: i32,
    pub setting_id: i32,
    pub size: f32,
    pub fitness: f32,
    pub life_expectancy: f32,
    pub growth_rate: f32,
    pub aging_rate: f32,
    pub mutation_rate: f32,
    pub stress_rate: f32,
    pub healthy_rate: f32
}

impl SqlType for Dna {
    fn table_name() -> &'static str { "dnas" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: row.get("id"),
            setting_id: row.get("setting_id"),
            size: row.get("size"),
            fitness: row.get("fitness"),
            life_expectancy: row.get("life_expectancy"),
            growth_rate: row.get("growth_rate"),
            aging_rate: row.get("aging_rate"),
            mutation_rate: row.get("mutation_rate"),
            stress_rate: row.get("stress_rate"),
            healthy_rate: row.get("healthy_rate")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec![
            "setting_id",
            "size",
            "fitness",
            "life_expectancy",
            "growth_rate",
            "aging_rate",
            "mutation_rate",
            "stress_rate",
            "healthy_rate",
        ]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![
            &self.setting_id,
            &self.size,
            &self.fitness,
            &self.life_expectancy,
            &self.growth_rate,
            &self.aging_rate,
            &self.mutation_rate,
            &self.stress_rate,
            &self.healthy_rate,
        ]
    }
}
