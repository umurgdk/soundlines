use rocket;
use rocket::Request;

use db;
use endpoints;
use endpoints::state::ParametersManager;

#[error(400)]
fn error(_: &Request) -> &'static str {
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

    rocket::ignite()
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
            endpoints::cells::recreate,
            endpoints::cells::cells_at,
        ])
        .mount("/users", routes![
            endpoints::users::location,
        ])
        .mount("/entities", routes![
            endpoints::entities::generate,
            endpoints::entities::index,
        ])
        .catch(errors![error])
        .manage(db_pool)
        .manage(parameters)
        .launch();
}
