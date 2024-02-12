extern crate reqwest;
extern crate toml;

use std::error::Error;

pub async fn fetch_toml_from_github(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "reqwest")
        .send()
        .await?;

    if response.status().is_success() {
        let toml_string = response.text().await?;

        let toml_value = toml::from_str::<toml::Value>(&toml_string)?;

        // Extract the version string from the TOML value
        if let Some(package_table) = toml_value.get("package") {
            if let Some(version_value) = package_table.get("version") {
                if let toml::Value::String(version) = version_value {
                    return Ok(version.clone());
                }
            }
        }

        Err("Failed to find 'version' key in 'package' table".into())
    } else {
        Err(format!(
            "Failed to fetch TOML file. Status code: {}",
            response.status()
        )
        .into())
    }
}
