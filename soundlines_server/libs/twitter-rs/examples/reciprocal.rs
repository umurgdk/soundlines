// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate egg_mode;

mod common;

use std::collections::HashSet;
use egg_mode::user;

//IMPORTANT: see common.rs for instructions on making sure this properly authenticates with
//Twitter.
fn main() {
    let config = common::Config::load();

    println!("");
    let friends =
        user::friends_ids(config.user_id, &config.token)
              .map(|r| r.unwrap().response)
              .collect::<HashSet<u64>>();
    let followers =
        user::followers_ids(config.user_id, &config.token)
              .map(|r| r.unwrap().response)
              .collect::<HashSet<u64>>();

    let reciprocals = friends.intersection(&followers).cloned().collect::<Vec<_>>();

    println!("{} accounts that you follow follow you back.", reciprocals.len());

    for user in user::lookup(&reciprocals, &config.token).unwrap() {
        println!("{} (@{})", user.name, user.screen_name);
    }
}
