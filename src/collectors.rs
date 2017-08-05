use rocket_contrib::Json;
use rocket::response::status;

use diesel::*;
use diesel::result::QueryResult;

use serde_json::Value;

use db::models::NewWifiReading;
use db::models::NewSoundReading;
use db::models::NewLightReading;
use db::models::NewGpsReading;

use db::guard::*;

#[post("/data/wifi", data = "<reading>")]
pub fn wifi(conn: DbConn, reading: Json<NewWifiReading>) -> QueryResult<status::NoContent> {
    use db::schema::wifi_readings;

    insert(&reading.0).into(wifi_readings::table)
        .execute(&*conn.0)
        .map(|_| status::NoContent)
}

#[post("/data/sound", data = "<reading>")]
pub fn sound(conn: DbConn, reading: Json<NewSoundReading>) -> QueryResult<status::NoContent> {
    use db::schema::sound_readings;

    insert(&reading.0).into(sound_readings::table)
        .execute(&*conn.0)
        .map(|_| status::NoContent)
}

#[post("/data/light", data = "<reading>")]
pub fn light(conn: DbConn, reading: Json<NewLightReading>) -> QueryResult<status::NoContent> {
    use db::schema::light_readings;

    insert(&reading.0).into(light_readings::table)
        .execute(&*conn.0)
        .map(|_| status::NoContent)
}

#[post("/data/gps", data = "<reading>")]
pub fn gps(conn: DbConn, reading: Json<NewGpsReading>) -> QueryResult<Json<Value>> {
    use db::schema::gps_readings;

    insert(&reading.0).into(gps_readings::table)
        .execute(&*conn.0)
        .map(|_| {
            let response = json!({
                "others": [],
                "cell": null
            });

            Json(response)
        })
}
