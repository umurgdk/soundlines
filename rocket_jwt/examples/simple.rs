#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_jwt;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket_jwt::Jwt;
use rocket_jwt::JwtConfig;

use rocket::http::Status;
use rocket::request::Form;
 
#[derive(Deserialize, Serialize)]
pub struct AuthCredentials {
    user_id: String
}

#[derive(FromForm)]
pub struct LoginForm {
    username: String,
    password: String
}

#[post("/user-only-endpoint")]
pub fn user_only_endpoint(cred: Jwt<AuthCredentials>) -> String {
    // JWT Token validated and payload (AuthCredentials) is parsed and extracted
    format!("User id is: {}", cred.0.user_id)
}

#[post("/login", data="<login_form>")]
pub fn login(login_form: Form<LoginForm>) -> Result<Jwt<AuthCredentials>, Status> {
    let login_form = login_form.into_inner();

    // Uber secure authentication...
    match (login_form.username.as_str(), login_form.password.as_str()) {
        ("admin", "admin") => Ok(Jwt(AuthCredentials { user_id: "adminid".to_string() })),
        _                  => Err(Status::Unauthorized)
    }
}

pub fn main() {
    let jwt_config = JwtConfig { secret: "my jwt secret".to_string() };
    rocket::ignite()
        .mount("/", routes![user_only_endpoint, login])
        .manage(jwt_config)
        .launch();
}
