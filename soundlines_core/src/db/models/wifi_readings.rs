use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;

use db::extensions::*;
use db::models::default_user_id;

#[derive(Serialize, Deserialize)]
pub struct WifiReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub ssid: String,
    pub level: f32,
    pub frequency: f32,
}

impl SqlType for WifiReading {
    fn table_name() -> &'static str { "wifi_readings" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: Some(row.get("id")),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            ssid: row.get("ssid"),
            level: row.get("level"),
            frequency: row.get("frequency")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec!["user_id", "created_at", "ssid", "level", "frequency"]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.user_id, &self.created_at, &self.ssid, &self.level, &self.frequency ]
    }
}
