use chrono::prelude::*;

use db::schema::wifi_readings;
use db::schema::sound_readings;
use db::schema::light_readings;
use db::schema::parameters;

fn default_user_id() -> i32 {
    1
}

#[derive(Queryable, Serialize)]
pub struct WifiReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub ssid: String,
    pub level: f32,
    pub frequency: f32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "wifi_readings"]
pub struct NewWifiReading {
    #[serde(default="default_user_id")]
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub ssid: String,
    pub level: f32,
    pub frequency: f32,
}

#[derive(Queryable, Serialize)]
pub struct SoundReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Insertable, Deserialize)]
#[table_name = "sound_readings"]
pub struct NewSoundReading {
    #[serde(default="default_user_id")]
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Queryable, Serialize)]
pub struct LightReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Insertable, Deserialize)]
#[table_name = "light_readings"]
pub struct NewLightReading {
    #[serde(default="default_user_id")]
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Clone)]
pub struct Parameter {
    pub id: i32,
    pub cell_size: i32
}

use geo::Polygon;
#[derive(Deserialize, Serialize, Clone)]
pub struct Cell {
    pub id: Option<i32>,
    pub geom: Polygon<f64>
}

use geo::Point;
#[derive(Deserialize, Serialize, Clone)]
pub struct GpsReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub point: Point<f64>
}
