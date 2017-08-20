use std::ops::Deref;

use chrono::ParseError;
use chrono::prelude::*;

use rocket::http::RawStr;
use rocket::request::FromParam;

pub struct DateTimeUtc(pub DateTime<Utc>);

impl Deref for DateTimeUtc {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromParam<'a> for DateTimeUtc {
    type Error = ParseError;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        param.as_str().parse::<DateTime<Utc>>().map(DateTimeUtc)
    }
}
