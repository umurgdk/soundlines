use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Seed;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::extensions::*;
use soundlines_core::db::Result as DbResult;
use soundlines_core::postgis::ewkb::Point;

use user::Auth;
use db_guard::DbConn;

#[derive(Deserialize)]
pub struct PickupPayload {
    id: i32
}

#[post("/pickup", data="<payload>")]
pub fn pickup(_auth: Auth, payload: Json<PickupPayload>, conn: DbConn) -> DbResult<Option<Json>> {
    let seed_id = payload.into_inner().id;

    let seed = match conn.get::<Seed>(seed_id)? {
        Some(seed) => seed,
        None       => return Ok(None)
    };

    conn.delete::<Seed>(seed_id)?;

    Ok(Some(Json(seed.into_json())))
}

#[derive(Deserialize)]
pub struct SpreadSeedPayload {
    latitude: f64,
    longitude: f64,
    nickname: String,
    dna_id: i32,
    setting_id: i32
}

#[post("/spread", data="<payload>")]
pub fn spread(_auth: Auth, payload: Json<SpreadSeedPayload>, conn: DbConn) -> Result<Json, Failure> {
    let payload = payload.into_inner();

    let location = Point::new(payload.longitude, payload.latitude, Some(4326));
    println!("Location: {:?}", location);
    let cell = Cell::find_containing_core(&*conn, &location);
    println!("Cell: {:?}", cell);
    let cell = cell
        .map_err(|_| Failure(Status::InternalServerError))?
        .ok_or(Failure(Status::BadRequest))?;

    println!("Cell found");

    let dna = conn.get::<Dna>(payload.dna_id)
        .map_err(|_| Failure(Status::InternalServerError))?
        .ok_or(Failure(Status::BadRequest))?;

    println!("Dna found");

    let setting = conn.get::<PlantSetting>(payload.setting_id)
        .map_err(|_| Failure(Status::InternalServerError))?
        .ok_or(Failure(Status::BadRequest))?;

    println!("Setting found");

    let mut entity = Entity::new(location, cell.id, &setting, &dna);
    entity.nickname = payload.nickname;

    let entity = conn.insert(&entity).map_err(|_| Failure(Status::InternalServerError))?;

    Ok(Json(entity.to_json()))
}
