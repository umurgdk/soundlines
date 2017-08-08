use rocket;
use rocket::Request;

use db;
use collectors;

#[error(400)]
fn error(_: &Request) -> &'static str {
    ""
}

pub fn run() {
    rocket::ignite()
        .mount("/", routes![
            collectors::wifi,
            collectors::sound,
            collectors::light,
            collectors::gps
        ])
        .catch(errors![error])
        .manage(db::init_pool())
        .launch();
}
