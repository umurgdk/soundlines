// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

extern crate hyper;
extern crate serde_json;
extern crate hyper_native_tls;

#[macro_use]
mod utils;

mod error;
mod models;

pub use error::{Error, Result};
pub use models::*;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use std::collections::HashMap;
use utils::into_string;

static API_URL: &'static str = "https://api.darksky.net";

/// A block is a name of a `Datablock` returned from the API. This can be used
/// to exclude datablocks from being returned from the API, to reduce bandwidth.
pub enum Block {
    Currently,
    Daily,
    Flags,
    Hourly,
    Minutely,
}

map_names! { Block;
    Currently, "currently";
    Daily, "daily";
    Flags, "flags";
    Hourly, "hourly";
    Minutely, "minutely";
}

/// The language to return from the API for the "summary" field.
///
/// The language is automatically English, so specifying English is not needed.
pub enum Language {
    /// Arabic
    Ar,
    /// Azerbaijani
    Az,
    /// Belarusian
    Be,
    /// Bosnian
    Bs,
    /// Czech
    Cs,
    /// German
    De,
    /// Greek
    El,
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// Croatian
    Hr,
    /// Hungarian
    Hu,
    /// Indonesian
    Id,
    /// Italian
    It,
    /// Icelandic
    Is,
    /// Cornish
    Kw,
    /// Norwegian Bokmål
    Nb,
    /// Dutch
    Nl,
    /// Polish
    Pl,
    /// Portuguese
    Pt,
    /// Russian
    Ru,
    /// Slovak
    Sk,
    /// Serbian
    Sr,
    /// Swedish
    Sv,
    /// Tetum
    Tet,
    /// Turkish
    Tr,
    /// Ukrainian
    Uk,
    /// Igpay Atinlay
    XPigLatin,
    /// Simplified Chinese
    Zh,
    /// Traditional Chinese
    ZhTw,
}

map_names! { Language;
    Ar, "ar";
    Az, "az";
    Be, "be";
    Bs, "bs";
    Cs, "cs";
    De, "de";
    El, "el";
    En, "en";
    Es, "es";
    Fr, "fr";
    Hr, "hr";
    Hu, "hu";
    Id, "id";
    It, "it";
    Is, "is";
    Kw, "kw";
    Nb, "nb";
    Nl, "nl";
    Pl, "pl";
    Pt, "pt";
    Ru, "ru";
    Sk, "sk";
    Sr, "sr";
    Sv, "sv";
    Tet, "tet";
    Tr, "tr";
    Uk, "uk";
    XPigLatin, "x-pig-latin";
    Zh, "zh";
    ZhTw, "zh-tw";
}

/// The type of units that the API should send back. `Auto` is the default
/// value, and does not need to be specified in that case.
///
/// The values are explained under "Options" and then "units=[setting]":
///
/// <https://developer.forecast.io/docs/v2>
pub enum Unit {
    Auto,
    Ca,
    Si,
    Uk2,
    Us,
}

map_names! { Unit;
    Auto, "auto";
    Ca, "ca";
    Si, "si";
    Uk2, "uk2";
    Us, "us";
}

/// Build a list of options to send in the request, including the type of units
/// that the API should send back, the blocks to exclude, whether to extend the
/// hourly forecast, and the language for the summary.
pub struct Options(HashMap<String, String>);

impl Options {
    /// Set the list of datablocks to exclude. For a full list of potential
    /// datablocks to exclude, refer to `Block`.
    pub fn exclude(mut self, blocks: Vec<Block>) -> Self {
        let block_names: Vec<&str> = blocks.iter()
            .map(|block| block.name())
            .collect();

        let list = block_names.join(",");

        self.0.insert("exclude".to_owned(), list.to_owned());

        self
    }

    /// Extends the hourly forecast to the full 7 days ahead, rather than only
    /// the first 2 days.
    pub fn extend_hourly(mut self) -> Self {
        self.0.insert("extend".to_owned(), "hourly".to_owned());

        self
    }

    /// Set the language of the summary provided.
    pub fn language(mut self, language: Language) -> Self {
        self.0.insert("lang".to_owned(), language.name().to_owned());

        self
    }

    /// Sets the unit type returned from the API. Refer to the forecast.io
    /// documentation for more info:
    ///
    /// <https://developer.forecast.io/docs/v2>
    pub fn unit(mut self, unit: Unit) -> Self {
        self.0.insert("units".to_owned(), unit.name().to_owned());

        self
    }
}

fn get_client() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}

pub fn get_forecast<S: Into<String>>(token: S,
                                     latitude: f64,
                                     longitude: f64)
                                     -> Result<Forecast> {
    let response =
        get_client()
        .get(&format!("{}/forecast/{}/{},{}?units=auto",
                      API_URL,
                      token.into(),
                      latitude,
                      longitude))
        .send()?;

    Forecast::decode(try!(serde_json::from_reader(response)))
}

pub fn get_forecast_with_options<S, F>(token: S,
                                       latitude: f64,
                                       longitude: f64,
                                       options: F)
                                       -> Result<Forecast>
                                       where F: FnOnce(Options) -> Options,
                                             S: Into<String> {
    let items: Vec<String> = options(Options(HashMap::new()))
        .0
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    let built = items.join("&");
    let response =
        get_client()
        .get(&format!("{}/forecast/{}/{},{}?{}",
                      API_URL,
                      token.into(),
                      latitude,
                      longitude,
                      built))
        .send()?;

    Forecast::decode(serde_json::from_reader(response)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_forecast() {
        let token = ::std::env::var("FORECAST_TOKEN").expect("forecast token");

        if let Err(why) = ::get_forecast(&token[..],
                                         37.8267,
                                         -122.423) {
            panic!("{:?}", why);
        }

        if let Err(why) = ::get_forecast(&token[..],
                                         39.9042,
                                         116.4074) {
            panic!("{:?}", why);
        }

        if let Err(why) = ::get_forecast(&token[..],
                                         19.2465,
                                         -99.1013) {
            panic!("{:?}", why);
        }
    }

    #[test]
    fn get_forecast_with_options() {
        let token = ::std::env::var("FORECAST_TOKEN").expect("forecast token");

        match ::get_forecast_with_options(
            &token[..],
            19.2465,
            -99.1013,
            |opt| opt
                .exclude(vec![::Block::Currently, ::Block::Daily])
                .extend_hourly()
                .language(::Language::Es)
                .unit(::Unit::Si)) {
            Ok(forecast) => {
                assert!(forecast.currently.is_none());
                assert!(forecast.daily.is_none());
                assert!(forecast.flags.is_some());
            },
            Err(why) => {
                panic!("{:?}", why);
            },
        }
    }
}
