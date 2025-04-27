use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{config::Config, gotify, version};

#[derive(Debug, Deserialize)]
struct AurResponse {
    results: Vec<AurResult>,
}

#[derive(Debug, Deserialize)]
struct AurResult {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Version")]
    version: String,
}

pub fn check(
    client: &Client,
    config: &Config,
    packages: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let aur_url = {
        let mut aur_url = reqwest::Url::parse("https://aur.archlinux.org/rpc/?v=5&type=info")?;

        {
            let mut query_pairs = aur_url.query_pairs_mut();

            for package in packages {
                query_pairs.append_pair("arg[]", package);
            }
        }

        aur_url
    };

    let resp: AurResponse = client.get(aur_url).send()?.json()?;

    for package in packages {
        let stored_version = version::read(package).unwrap_or_else(|| "none".to_string());
        let latest_version = &resp
            .results
            .iter()
            .find(|r| &r.name == package)
            .unwrap()
            .version;
        if &stored_version == latest_version {
            println!("{package} is up to date: {latest_version}");
        } else {
            println!("{package} was updated: {latest_version}");
            gotify::push(client, config, package, latest_version)?;
            version::write(package, latest_version)?;
        }
    }

    Ok(())
}
