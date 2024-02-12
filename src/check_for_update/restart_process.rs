use std::{os::unix::process::CommandExt, process::Command};

pub fn restart_process() {
    println!("Starting restart");
    let output = Command::new("/proc/self/exe").exec();
}
