use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use postgis::ewkb::Point;

use db::extensions::*;
use db::models::default_user_id;

pub struct WifiReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub ssid: String,
    pub level: f32,
    pub frequency: f32,
    pub point: Point
}

impl WifiReading {
    pub fn into_wifi_reading_json(self) -> WifiReadingJson {
        WifiReadingJson {
            created_at: self.created_at,
            ssid: self.ssid,
            level: self.level,
            frequency: self.frequency,
            latitude: self.point.x,
            longitude: self.point.y,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WifiReadingJson {
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub ssid: String,
    pub level: f32,
    pub frequency: f32,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64
}

impl WifiReadingJson {
    pub fn into_wifi_reading(self, user_id: i32) -> WifiReading {
        let WifiReadingJson { created_at, ssid, level, frequency, latitude, longitude } = self;
        WifiReading { id: None, user_id, created_at, ssid, level, frequency, point: Point::new(longitude, latitude, Some(4326)) }
    }
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
            frequency: row.get("frequency"),
            point: row.get("point")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec!["user_id", "created_at", "ssid", "level", "frequency", "point"]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.user_id, &self.created_at, &self.ssid, &self.level, &self.frequency, &self.point ]
    }
}
