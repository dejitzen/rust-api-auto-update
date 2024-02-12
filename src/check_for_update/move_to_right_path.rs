use std::process::Command;

pub fn move_to_right_path() {
    println!("Starting moving");
    let output = Command::new("mv")
        .args(["-T", "temp/new-version/target/release", "target/release"])
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Print the output as a string
        println!(
            "Finished moving:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        // Print the error message
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
