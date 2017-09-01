use rocket;
use rocket::Request;

use rocket_jwt::JwtConfig;

use db;
use endpoints;
use endpoints::state::ParametersManager;

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
    let parameters = {
        let conn = db_pool.get().expect("Failed to get a db connection from the pool!");
        ParametersManager::from_db(&*conn)
    };

    let igniter = rocket::ignite();

    let jwt_secret = igniter.config().get_str("jwt_secret").expect("jwt_secret").to_string();
    let jwt_config = JwtConfig { secret: jwt_secret };

    igniter
        .mount("/", routes![
            endpoints::parameters::get_parameters,
            endpoints::parameters::update_parameters,
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
            endpoints::cells::recreate,
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
        ])
        .mount("/seeds", routes![
            endpoints::seeds::pickup,
            endpoints::seeds::spread
        ])
        .mount("/dev", routes![
            endpoints::dev::get_version,
            endpoints::dev::update_version
        ])
        .catch(errors![error, error_401, error_500])
        .manage(db_pool)
        .manage(parameters)
        .manage(jwt_config)
        .launch();
}
