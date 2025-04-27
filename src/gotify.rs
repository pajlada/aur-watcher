use reqwest::blocking::Client;
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
struct GotifyNotification {
    title: String,
    message: String,
    priority: i32,
}

pub fn push(
    client: &Client,
    config: &Config,
    package: &str,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = GotifyNotification {
        title: format!("AUR package {package} updated"),
        message: format!("{package} was updated to {version}"),
        priority: config.gotify_priority,
    };

    client
        .post(&config.gotify_url)
        .header("X-Gotify-Key", &config.gotify_token)
        .json(&payload)
        .send()?
        .error_for_status()?;

    Ok(())
}
