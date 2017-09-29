use rocket;
use rocket::Request;
use rocket_jwt::JwtConfig;
use soundlines_core::db;

use endpoints;

#[error(400)]
fn error(_: &Request) -> &'static str {
    ""
}

#[error(401)]
fn error_401(_: &Request) -> &'static str {
    ""
}

#[error(500)]
fn error_500(_: &Request) -> &'static str {
    ""
}

pub fn run() {
    let db_pool = db::init_pool();

    let igniter = rocket::ignite();

    let jwt_secret = igniter.config().get_str("jwt_secret").expect("jwt_secret").to_string();
    let jwt_config = JwtConfig { secret: jwt_secret };

    igniter
	    .mount("/", routes![
	        endpoints::weather::get
	    ])
        .mount("/data", routes![
            endpoints::collectors::wifi,
            endpoints::collectors::sound,
            endpoints::collectors::light,
            endpoints::collectors::gps,
        ])
        .mount("/cells", routes![
            endpoints::cells::index,
            endpoints::cells::show,
            endpoints::cells::cells_at,
        ])
        .mount("/users", routes![
            endpoints::users::register,
            endpoints::users::location,
            endpoints::users::location_range,
            endpoints::users::location_times
        ])
        .mount("/entities", routes![
            endpoints::entities::generate,
            endpoints::entities::index,
            endpoints::entities::delete
        ])
        .mount("/seeds", routes![
            endpoints::seeds::pickup,
            endpoints::seeds::spread,
            endpoints::seeds::get,
            endpoints::seeds::deploy
        ])
        .mount("/dev", routes![
            endpoints::dev::get_version,
            endpoints::dev::update_version,
            endpoints::dev::get_settings,
            endpoints::dev::update_setting,
            endpoints::dev::get_snapshot
        ])
        .catch(errors![error, error_401, error_500])
        .manage(db_pool)
        .manage(jwt_config)
        .launch();
}
