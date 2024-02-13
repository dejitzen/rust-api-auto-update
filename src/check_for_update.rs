mod build_new_version;
mod clone_project;
mod delete_temp_files;
mod fetch_toml_from_github;
mod move_to_right_path;
mod restart_process;
use std::env;
use version_compare::{compare, Cmp};

fn download_and_install_new_version() {
    clone_project::clone_project();
    build_new_version::build_new_version();
    move_to_right_path::move_to_right_path();
    delete_temp_files::delete_temp_files();
    restart_process::restart_process();
}

pub async fn check_for_update() {
    const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

    let url = "https://raw.githubusercontent.com/dejitzen/rust-api-auto-update/main/Cargo.toml";
    if let Ok(remote_version) = fetch_toml_from_github::fetch_toml_from_github(url).await {
        println!("Remote version: {}", remote_version);
        match compare(CURRENT_VERSION, &remote_version) {
            Ok(cmp) => match cmp {
                Cmp::Lt => download_and_install_new_version(),
                Cmp::Eq => println!("You are using the latest version."),
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
