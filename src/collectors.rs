use rocket_contrib::Json;
use rocket::response::status;

use diesel::*;
use diesel::result::QueryResult;

use serde_json::Value;

use db::models::NewWifiReading;
use db::models::WifiReading;
use db::models::NewSoundReading;
use db::models::SoundReading;
use db::models::NewLightReading;
use db::models::LightReading;
use db::models::NewGpsReading;
use db::models::GpsReading;

use db::guard::*;

#[post("/data/wifi", data = "<reading>")]
pub fn wifi(conn: DbConn, reading: Json<NewWifiReading>) -> QueryResult<Json<WifiReading>> {
    use db::schema::wifi_readings;

    insert(&reading.0).into(wifi_readings::table)
        .get_result::<WifiReading>(&*conn)
        .map(Json)
}

#[post("/data/sound", data = "<reading>")]
pub fn sound(conn: DbConn, reading: Json<NewSoundReading>) -> QueryResult<Json<SoundReading>> {
    use db::schema::sound_readings;

    insert(&reading.0).into(sound_readings::table)
        .get_result::<SoundReading>(&*conn)
        .map(Json)
}

#[post("/data/light", data = "<reading>")]
pub fn light(conn: DbConn, reading: Json<NewLightReading>) -> QueryResult<Json<LightReading>> {
    use db::schema::light_readings;

    insert(&reading.0).into(light_readings::table)
        .get_result::<LightReading>(&*conn)
        .map(Json)
}

#[post("/data/gps", data = "<reading>")]
pub fn gps(conn: DbConn, reading: Json<NewGpsReading>) -> QueryResult<Json<Value>> {
    use db::schema::gps_readings;

    insert(&reading.0).into(gps_readings::table)
        .get_result::<GpsReading>(&*conn)
        .map(|reading| {
            let response = json!({
                "echo": reading,
                "others": [],
                "cell": null
            });

            Json(response)
        })
}
