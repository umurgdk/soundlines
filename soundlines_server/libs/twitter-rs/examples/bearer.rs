// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate egg_mode;

mod common;

fn main() {
    let con_key = include_str!("common/consumer_key").trim();
    let con_secret = include_str!("common/consumer_secret").trim();

    let con_token = egg_mode::KeyPair::new(con_key, con_secret);

    println!("Pulling up the bearer token...");
    let token = egg_mode::bearer_token(&con_token).unwrap();

    println!("Pulling up a user timeline...");
    let mut timeline = egg_mode::tweet::user_timeline("rustlang", false, true, &token).with_page_size(5);

    for tweet in timeline.start().unwrap() {
        println!("");
        common::print_tweet(&tweet);
    }
}
