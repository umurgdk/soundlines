use chrono::prelude::*;

fn default_user_id() -> i32 {
    1
}

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

#[derive(Serialize, Deserialize)]
pub struct SoundReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Serialize, Deserialize)]
pub struct LightReading {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub id: Option<i32>,
    pub cell_size: i32
}

use postgis::ewkb::Polygon;
#[derive(Clone)]
pub struct Cell {
    pub id: Option<i32>,
    pub geom: Polygon
}

use postgis::ewkb::Point;
#[derive(Clone)]
pub struct GpsReading {
    pub id: Option<i32>,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub point: Point
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GpsReadingJson {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64
}
