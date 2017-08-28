use rocket::request;
use rocket::Request;
use rocket::http::Status;
use rocket::data;
use rocket::Data;
use rocket::data::FromData;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_contrib::Json;

use serde::de::DeserializeOwned;
use serde_json::from_value;
use serde_json::Value as JValue;

use rocket_jwt::Jwt;

use soundlines_core::db::models::User;

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    action: String
}

pub struct Auth(User);

impl Auth {
    pub fn into_user(self) -> User {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.guard::<Jwt<User>>()?;
        Outcome::Success(Auth(token.into_inner()))
    }
}

pub struct JsonUserIdCheck<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> FromData for JsonUserIdCheck<T> {
    type Error = ();
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let token = match request.guard::<Jwt<User>>() {
            Outcome::Success(token) => token,
            _                       => return Outcome::Failure((Status::Unauthorized, ()))
        };

        let user = token.into_inner();

        let json: JValue = Json::<JValue>::from_data(request, data).map_failure(|_| (Status::BadRequest, ()))?.into_inner();

        if json["user_id"] != user.id {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        match from_value(json) {
            Ok(val) => Outcome::Success(JsonUserIdCheck(val)),
            _       => Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
