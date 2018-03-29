extern crate egg_mode;

use egg_mode::{authorize_url, request_token, KeyPair};
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

    let pin = match env::var("UBLOG_PIN") {
        Ok(pin) => Some(pin),
        Err(_) => None,
    };

    let con_token = KeyPair::new(consumer_key, consumer_secret);
    let request_token = request_token(&con_token, "oob").unwrap();

    let pin = match pin {
        Some(pin) => pin,
        None => {
            let auth_url = authorize_url(&request_token);
            println!("Please sign in and set UBLOG_PIN: {}", auth_url);
            std::process::exit(1);
        }
    };

    println!("pin: {:?}", pin);
    let (token, user_id, screen_name) =
        egg_mode::access_token(con_token, &request_token, pin).unwrap();

    println!("result: {:?} {:?} {:?}", token, user_id, screen_name);
}
