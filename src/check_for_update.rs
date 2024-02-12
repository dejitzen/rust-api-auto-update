mod fetch_toml_from_github;
use std::env;
use version_compare::{compare, Cmp};

pub async fn check_for_update() {
    const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("STARTING");

    let url = "https://raw.githubusercontent.com/dejitzen/rust-api-auto-update/main/Cargo.toml";
    if let Ok(remote_version) = fetch_toml_from_github::fetch_toml_from_github(url).await {
        match compare(CURRENT_VERSION, &remote_version) {
            Ok(cmp) => match cmp {
                Cmp::Lt => println!("A newer version is available."),
                Cmp::Eq => println!("Your application is up to date."),
                Cmp::Gt => println!("You are using a newer version than available."),
                _ => println!("Unexpected comparison result."),
            },
            Err(_err) => {
                println!("Failed to compare versions");
            }
        }
    } else {
        println!("Failed to fetch remote version.");
    }
}
