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

use serde_json::Value;
use std::collections::BTreeMap;
use ::error::{Error, Result};

#[macro_escape]
macro_rules! field {
    ($map:expr, float, $key:expr) => {
        remove(&mut $map, $key).ok().and_then(|v| v.as_f64())
    };

    ($map:expr, R, float, $key:expr) => {
        req!(try!(remove(&mut $map, $key)).as_f64())
    };

    ($map:expr, int, $key:expr) => {
        remove(&mut $map, $key).ok().and_then(|v| v.as_u64())
    };

    ($map:expr, R, int, $key:expr) => {
        req!(try!(remove(&mut $map, $key)).as_u64())
    };

    ($map:expr, O, $key:expr, $decode:path) => {
        try!(opt(&mut $map, $key, $decode))
    };

    ($map:expr, R, $key:expr, $decode:path) => {
        try!(remove(&mut $map, $key).and_then($decode))
    };
}

#[macro_escape]
macro_rules! req {
    ($opt:expr) => {
        try!($opt.ok_or(Error::Decode(concat!("Type mismatch in model:",
                                              line!(),
                                              ": ",
                                              stringify!($opt)), Value::Null)))
    }
}

#[macro_escape]
macro_rules! map_names {
    ($typ:ident; $($entry:ident, $value:expr;)*) => {
        impl $typ {
            pub fn name(&self) -> &str {
                match *self {
                    $($typ::$entry => $value,)*
                }
            }

            #[doc(hidden)]
            pub fn from_str(name: &str) -> Option<Self> {
                match name {
                    $($value => Some($typ::$entry),)*
                    _ => None,
                }
            }

            #[doc(hiddden)]
            pub fn decode(value: Value) -> Result<Self> {
                let name = try!(into_string(value));
                Self::from_str(&name).ok_or(Error::Decode(
                    concat!("Expected valid ", stringify!($typ)),
                    Value::String(name)
                ))
            }
        }
    }
}

pub fn into_map(value: Value) -> Result<BTreeMap<String, Value>> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

pub fn decode_array<T, F: Fn(Value) -> Result<T>>(value: Value,
                                                  f: F)
                                                  -> Result<Vec<T>> {
    into_array(value)
        .and_then(|x| x.into_iter().map(f).collect())
}

pub fn into_array(value: Value) -> Result<Vec<Value>> {
    match value {
        Value::Array(v) => Ok(v),
        value => Err(Error::Decode("Expected array", value)),
    }
}

pub fn into_string(value: Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s),
        value => Err(Error::Decode("Expected string", value)),
    }
}

pub fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value> {
    map.remove(key)
        .ok_or(Error::Decode("Unexpected absent key",
                             Value::String(key.into())))
}

pub fn opt<T, F: FnOnce(Value) -> Result<T>>(map: &mut BTreeMap<String, Value>,
                                            key: &str, f: F)
                                            -> Result<Option<T>> {
    match map.remove(key) {
        None | Some(Value::Null) => Ok(None),
        Some(val) => f(val).map(Some),
    }
}
