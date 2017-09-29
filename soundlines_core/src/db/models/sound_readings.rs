use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use postgis::ewkb::Point;

use db::extensions::*;

#[derive(Serialize)]
pub struct SoundReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32,
    #[serde(skip)]
    pub point: Point
}

impl SqlType for SoundReading {
    fn table_name() -> &'static str { "sound_readings" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: Some(row.get("id")),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            level: row.get("level"),
            point: row.get("point")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec!["user_id", "created_at", "level", "point"]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.user_id, &self.created_at, &self.level, &self.point ]
    }
}
