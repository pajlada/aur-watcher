use serde::Deserialize;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub gotify_url: String,
    pub gotify_token: String,
    pub gotify_priority: i32,
    pub packages: Vec<String>,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(CONFIG_FILE)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
