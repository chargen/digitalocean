// Demonstrates `DigitalOcean::execute(...)`

extern crate digitalocean;
extern crate dotenv;
extern crate env_logger;

use digitalocean::DigitalOcean;
use digitalocean::api::SshKey;
use std::env;

// cargo run --example keys
fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    let req = SshKey::list();
    let result = client.execute(req).unwrap();

    println!("{:#?}", result);
}
