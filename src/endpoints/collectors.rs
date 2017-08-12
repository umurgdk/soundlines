use rocket_contrib::Json;

use diesel::*;
use diesel::result::QueryResult;

use serde_json::Value;

use db::models::NewWifiReading;
use db::models::WifiReading;
use db::models::NewSoundReading;
use db::models::SoundReading;
use db::models::NewLightReading;
use db::models::LightReading;
use db::models::GpsReading;

use db::guard::*;

#[post("/wifi", data = "<reading>")]
pub fn wifi(conn: DbConn, reading: Json<NewWifiReading>) -> QueryResult<Json<WifiReading>> {
    use db::schema::wifi_readings;

    insert(&reading.0).into(wifi_readings::table)
        .get_result::<WifiReading>(&*conn)
        .map(Json)
}

#[post("/sound", data = "<reading>")]
pub fn sound(conn: DbConn, reading: Json<NewSoundReading>) -> QueryResult<Json<SoundReading>> {
    use db::schema::sound_readings;

    insert(&reading.0).into(sound_readings::table)
        .get_result::<SoundReading>(&*conn)
        .map(Json)
}

#[post("/light", data = "<reading>")]
pub fn light(conn: DbConn, reading: Json<NewLightReading>) -> QueryResult<Json<LightReading>> {
    use db::schema::light_readings;

    insert(&reading.0).into(light_readings::table)
        .get_result::<LightReading>(&*conn)
        .map(Json)
}

#[post("/gps", data = "<reading>")]
pub fn gps(conn: DbConn, reading: Json<GpsReading>) -> QueryResult<Json<Value>> {
    use chrono::prelude::*;

    unimplemented!();
}
