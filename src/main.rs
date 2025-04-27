#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use reqwest::blocking::Client;

mod config;

mod version;

mod gotify;

mod aur;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load()?;
    let client = Client::new();

    version::init()?;

    aur::check(&client, &config, &config.packages)?;

    Ok(())
}
