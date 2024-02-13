use std::{os::unix::process::CommandExt, process::Command};

pub fn restart_process() {
    println!("Starting restart");
    let output = Command::new("./target/release/rust-api-auto-update").exec();
}
