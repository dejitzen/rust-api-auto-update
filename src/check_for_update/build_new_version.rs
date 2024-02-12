use std::process::Command;

pub fn build_new_version() {
    println!("Starting build");
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("temp/new-version")
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Print the output as a string
        println!(
            "Finished build:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        // Print the error message
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
