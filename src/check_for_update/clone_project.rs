use std::process::Command;

pub fn clone_project() {
    println!("Starting clone");
    let output = Command::new("git")
        .args([
            "clone",
            "https://github.com/dejitzen/rust-api-auto-update",
            "temp/new-version",
        ])
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Print the output as a string
        println!(
            "Finished clone:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        // Print the error message
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
