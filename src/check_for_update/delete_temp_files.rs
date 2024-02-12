use std::process::Command;

pub fn delete_temp_files() {
    let output = Command::new("rm")
        .args(["-rf", "temp/new-version"])
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Print the output as a string
        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        // Print the error message
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
