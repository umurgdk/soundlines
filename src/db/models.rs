use chrono::prelude::*;

use db::schema::wifi_readings;
use db::schema::sound_readings;
use db::schema::light_readings;
use db::schema::gps_readings;

#[derive(Queryable)]
pub struct WifiReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub channel_id: String,
    pub strength: f32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "wifi_readings"]
pub struct NewWifiReading {
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub channel_id: String,
    pub strength: f32,
}

#[derive(Queryable)]
pub struct SoundReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Insertable, Deserialize)]
#[table_name = "sound_readings"]
pub struct NewSoundReading {
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Queryable)]
pub struct LightReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Insertable, Deserialize)]
#[table_name = "light_readings"]
pub struct NewLightReading {
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub level: f32
}

#[derive(Queryable)]
pub struct GpsReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Insertable, Deserialize)]
#[table_name = "gps_readings"]
pub struct NewGpsReading {
    pub user_id: i32,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}



