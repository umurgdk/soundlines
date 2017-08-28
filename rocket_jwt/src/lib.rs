//! JWT Token guard & state for rocket web server library
//!
//! # Example
//!
//! ```
//! #![feature(plugin)]
//! #![feature(custom_derive)]
//! #![plugin(rocket_codegen)]
//! extern crate rocket;
//! extern crate rocket_jwt;
//!
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use rocket_jwt::Jwt;
//! use rocket_jwt::JwtConfig;
//!
//! use rocket::http::Status;
//! use rocket::request::Form;
//! 
//! #[derive(Deserialize, Serialize)]
//! pub struct AuthCredentials {
//!     user_id: String
//! }
//!
//! #[derive(FromForm)]
//! pub struct LoginForm {
//!     username: String,
//!     password: String
//! }
//!
//! #[post("/user-only-endpoint")]
//! pub fn user_only_endpoint(credentials: Jwt<AuthCredentials>) -> String {
//!     let credentials = credentials.into_inner();
//!
//!     // JWT Token validated and payload (AuthCredentials) is parsed and extracted
//!     format!("User id is: {}", cred.user_id)
//! }
//!
//! #[post("/login", data="<login_form>")]
//! pub fn login(login_form: Form<LoginForm>) -> Result<Jwt<AuthCredentials>, Status> {
//!     let login_form = login_form.into_inner();
//!
//!     // Uber secure authentication...
//!     match (login_form.username, login_form.password) {
//!         ("admin", "admin") => Jwt(AuthCredentials { user_id: "adminid" }),
//!         _                  => Status::Unauthorized
//!     }
//! }
//!
//! pub fn main() {
//!     let jwt_config = JwtConfig { secret: "my jwt secret" };
//!     rocket::ignite()
//!         .mount("/", routes![user_only_endpoint, login])
//!         .manage(jwt_config)
//!         .launch();
//! }
//! ```
extern crate rocket;
extern crate jsonwebtoken as jwt;
extern crate serde;

use std::io::Cursor;

use rocket::State;
use rocket::Request;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request;
use rocket::request::FromRequest;
use rocket::response;
use rocket::response::Response;
use rocket::response::Responder;

use jwt::decode;
use jwt::encode;
use jwt::Header;
use jwt::Validation;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct JwtConfig {
    pub secret: String
}

pub struct Jwt<T: Serialize + DeserializeOwned>(pub T);

impl<T: Serialize + DeserializeOwned> Jwt<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn encode(&self, jwt_config: &JwtConfig) -> Result<String, Status> {
        encode::<T>(&Header::default(), &self.0, jwt_config.secret.as_bytes())
            .map_err(|_| Status::InternalServerError)
    }
}

impl<'a, 'r, T: Serialize + DeserializeOwned> FromRequest<'a, 'r> for Jwt<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Get rocket state for accessing jwt secret
        let jwt_config = request.guard::<State<JwtConfig>>().expect("JwtConfig state");

        // If authorization header is not set return failure
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        // Extract token from Authorization header
        let auth_parts: Vec<_> = keys[0].split_whitespace().collect();
        if auth_parts.len() != 2 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        // Decode jwt token
        let key = auth_parts[1];
        let token = decode::<T>(&key, jwt_config.secret.as_bytes(), &Validation::default());

        match token {
            Ok(token) => Outcome::Success(Jwt(token.claims)),
            _         => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

impl<'r, T: Serialize + DeserializeOwned> Responder<'r> for Jwt<T> {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        // Get rocket state for accessing jwt secret
        let jwt_config = request.guard::<State<JwtConfig>>().expect("JwtConfig state not set!");

        // Encode the payload into JWT
        let token = encode::<T>(&Header::default(), &self.0, jwt_config.secret.as_bytes())
            .map_err(|_| Status::InternalServerError)?;

        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(token))
            .ok()
    }
}
