mod download_and_verify_update;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMetadata {
    version: String,
    url: String,
    checksum: String,
}

async fn download_and_verify_update(metadata: &UpdateMetadata) -> Result<String, &'static str> {
    let download_path = download_update(metadata).await?;
    if verify_update(&download_path, &metadata.checksum) {
        Ok(download_path)
    } else {
        Err("Verification failed")
    }
}

pub async fn check_for_updates(current_version: &str, metadata_url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let response = reqwest::get(metadata_url).await?;
    let metadata: UpdateMetadata = response.json().await?;
    if metadata.version > current_version {
        println!("Update available: {}", metadata.version);
        // Directly call download_and_verify_update here
        match download_and_verify_update(&metadata).await {
            Ok(download_path) => {
                println!("Update downloaded and verified at: {}", download_path);
                Ok(Some(download_path))
            },
            Err(e) => {
                println!("Failed to download or verify update: {}", e);
                Err(e.into())
            },
        }
    } else {
        println!("No updates available or failed to compare versions.");
        Ok(None)
    }
}


#[tokio::main]
async fn main() {
    let current_version = "your_current_version";
    let metadata_url = "your_metadata_url";
    if let Err(e) = check_for_updates(current_version, metadata_url).await {
        eprintln!("Error checking for updates: {}", e);
    }
}
