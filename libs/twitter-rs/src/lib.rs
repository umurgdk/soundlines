// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A library for interacting with Twitter.
//!
//! [Repository](https://github.com/QuietMisdreavus/twitter-rs)
//!
//! egg-mode is a Twitter library that aims to make as few assumptions about the user's codebase as
//! possible. Endpoints are exposed as bare functions where authentication details are passed in as
//! arguments, rather than as builder functions of a root "service" manager. The only exceptions to
//! this guideline are endpoints with many optional parameters, like posting a status update or
//! updating the metadata of a list.
//!
//! To use the Twitter API, there are some extra steps you need to do, both inside and outside of
//! your app code. Find the Authentication Overview and quick start guide in the [Token][]
//! documentation. The following examples already have a `token` on hand.
//!
//! [Token]: enum.Token.html
//!
//! To load the profile information of a single user:
//!
//! ```rust,no_run
//! # let token = egg_mode::Token::Access {
//! #     consumer: egg_mode::KeyPair::new("", ""),
//! #     access: egg_mode::KeyPair::new("", ""),
//! # };
//! let rustlang = egg_mode::user::show("rustlang", &token).unwrap();
//!
//! println!("{} (@{})", rustlang.name, rustlang.screen_name);
//! ```
//!
//! To post a new tweet:
//!
//! ```rust,no_run
//! # let token = egg_mode::Token::Access {
//! #     consumer: egg_mode::KeyPair::new("", ""),
//! #     access: egg_mode::KeyPair::new("", ""),
//! # };
//! use egg_mode::tweet::DraftTweet;
//!
//! let post = DraftTweet::new("Hey Twitter!").send(&token).unwrap();
//! ```
//!
//! # Types and Functions
//!
//! All of the main content of egg-mode is in submodules, but there are a few things here in the
//! crate root. To wit, it contains items related to authentication and a couple items that all the
//! submodules use.
//!
//! ## `Response<T>`
//!
//! Every method that calls Twitter and carries rate-limit information wraps its return value in a
//! [`Response`][] struct, that transmits this information to your app. From there, you can handle
//! the rate-limit information to hold off on that kind of request, or simply grab its `response`
//! field to get the output of whatever method you called. `Response` also implements `Deref`, so
//! for the most part you can access fields of the final result without having to grab the
//! `response` field directly.
//!
//! `Response` also has IntoIterator implementations and iterator creation methods that echo those
//! on `Vec<T>`, for methods that return Vecs. These methods and iterator types distribute the
//! rate-limit information across each iteration.
//!
//! There's also a type alias, [`WebResponse`], which is an alias to `Result<Response<T>, Error>`,
//! indicating a shorthand for network calls that return rate-limit metadata.
//!
//! [`Response`]: struct.Response.html
//! [`WebResponse`]: type.WebResponse.html
//!
//! ## Authentication Types/Functions
//!
//! The remaining types and methods are explained as part of the [authentication overview][Token],
//! with the exception of `verify_tokens`, which is a simple method to ensure a given token is
//! still valid.
//!
//! # Modules
//!
//! As there are many actions available in the Twitter API, egg-mode divides them roughly into
//! several modules by their shared purpose. Here's a sort of high-level overview, in rough order
//! from "most important" to "less directly used":
//!
//! ## Primary actions
//!
//! These could be considered the "core" actions within the Twitter API that egg-mode has made
//! available.
//!
//! * `tweet`: This module lets you act on tweets. Here you can find actions to load a user's
//!   timeline, post a new tweet, or like and retweet individual posts.
//! * `user`: This module lets you act on users, be it by following or unfollowing them, loading
//!   their profile information, blocking or muting them, or showing the relationship between two
//!   users.
//! * `search`: Due to the complexity of searching for tweets, it gets its own module.
//! * `direct`: Here you can work with a user's Direct Messages, either by loading DMs they've sent
//!   or received, or by sending new ones.
//! * `list`: This module lets you act on lists, from creating and deleting them, adding and
//!   removing users, or loading the posts made by their members.
//! * `text`: Text processing functions to count characters in new tweets and extract links and
//!   hashtags for highlighting and linking.
//!
//! ## Secondary actions
//!
//! These modules still contain direct actions for Twitter, but they can be considered as having
//! more of a helper role than something you might use directly.
//!
//! * `place`: Here are actions that look up physical locations that can be attached to tweets, as
//!   well at the `Place` struct that appears on tweets with locations attached.
//! * `service`: These are some miscellaneous methods that show information about the Twitter
//!   service as a whole, like loading the maximum length of t.co URLs or loading the current Terms
//!   of Service or Privacy Policy.
//!
//! ## Helper structs
//!
//! These modules contain some implementations that wrap some pattern seen in multiple "action"
//! modules.
//!
//! * `cursor`: This contains a helper trait and some helper structs that allow effective cursoring
//!   through certain collections of results from Twitter.
//! * `entities`: Whenever some text can be returned that may contain links, hashtags, media, or
//!   user mentions, its metadata is parsed into something that lives in this module.
//! * `error`: Any interaction with Twitter may result in an error condition, be it from finding a
//!   tweet or user that doesn't exist or the network connection being unavailable. All the error
//!   types are aggregated into an enum in this module.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

#[macro_use] extern crate hyper;
#[macro_use] extern crate lazy_static;
extern crate hyper_native_tls;
extern crate native_tls;
extern crate url;
extern crate rand;
extern crate ring;
extern crate rustc_serialize;
extern crate mime;
extern crate chrono;
extern crate regex;
extern crate unicode_normalization;

#[macro_use] mod common;
mod auth;
pub mod error;
pub mod user;
pub mod entities;
pub mod cursor;
pub mod tweet;
pub mod search;
pub mod place;
pub mod direct;
pub mod service;
pub mod text;
pub mod list;
mod links;

pub use auth::{KeyPair, Token, request_token, authorize_url, authenticate_url,
               access_token, verify_tokens, bearer_token, invalidate_bearer};
pub use common::{Response, ResponseIter, ResponseIterRef, ResponseIterMut, WebResponse};
