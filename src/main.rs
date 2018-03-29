extern crate egg_mode;

use egg_mode::{KeyPair};
use std::env;

fn main() {
    // Todo: use envy and/or dotenv.
    let consumer_key = match env::var("UBLOG_KEY") {
        Ok(key) => key,
        Err(_e) => {
            println!("You need to set $UBLOG_KEY, see https://apps.twitter.com/.");
            std::process::exit(1);
        }
    };
    let consumer_secret = match env::var("UBLOG_SECRET") {
        Ok(key) => key,
        Err(_) => {
            println!("You need to set $UBLOG_SECRET, see https://apps.twitter.com/.");
            std::process::exit(1);
        }
    };

    let con_token = KeyPair::new(consumer_key, consumer_secret);
    let token = egg_mode::bearer_token(&con_token).unwrap();

    let mut timeline =
        egg_mode::tweet::user_timeline("_wilfredh", false, true, &token).with_page_size(5);

    for tweet in timeline.start().unwrap() {
        println!("----------\n{}\n", tweet.text);
    }
}
