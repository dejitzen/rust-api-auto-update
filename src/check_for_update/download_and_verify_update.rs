use std::fs::File;
use std::io::{Read, Write};
use sha2::{Sha256, Digest};
use tempfile::tempdir;
use crate::check_for_update::UpdateMetadata;

async fn download_update(metadata: &UpdateMetadata) -> Result<String, &'static str> {
    let mut response = reqwest::get(&metadata.url).await.map_err(|_| "Failed to download update")?;
    let dir = tempdir().map_err(|_| "Failed to create temp dir")?;
    let file_path = dir.path().join("update.tar.gz");
    let mut file = File::create(&file_path).map_err(|_| "Failed to create file")?;

    while let Some(chunk) = response.chunk().await.map_err(|_| "Failed to read chunk")? {
        file.write_all(&chunk).map_err(|_| "Failed to write to file")?;
    }

    Ok(file_path.to_str().unwrap().to_string())
}

fn verify_update(file_path: &str, expected_checksum: &str) -> bool {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024]; // Buffer size can be adjusted

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(count) => hasher.update(&buffer[..count]),
            Err(_) => return false, // Handle errors or add error handling as needed
        }
    }

    let result = hasher.finalize();
    let result_str = format!("{:x}", result);
    result_str == expected_checksum
}