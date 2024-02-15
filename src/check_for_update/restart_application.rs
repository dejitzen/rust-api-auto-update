use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

fn restart_application_with_new_version(new_executable_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Get the current executable path
    let current_executable = env::current_exe()?;

    // Ensure the new executable exists
    if !new_executable_path.exists() {
        return Err("New executable does not exist.".into());
    }

    // Generate paths for backup and temporary new executable names
    let old_executable_path = current_executable.with_extension("old");
    let temp_new_executable_path = current_executable.with_extension("new");

    // On Windows, it's not possible to overwrite the running executable.
    // Therefore, we first rename the current executable to a backup.
    if cfg!(target_family = "windows") {
        if old_executable_path.exists() {
            fs::remove_file(&old_executable_path)?;
        }
        fs::rename(&current_executable, &old_executable_path)?;
    }

    // Rename/move the new executable to the current executable's location
    fs::rename(new_executable_path, &temp_new_executable_path)?;

    // Attempt to replace the current executable with the new one
    fs::rename(&temp_new_executable_path, &current_executable)?;

    // Restart the application
    // Note: Adjust the command as necessary for your application, including any required arguments
    if let Err(e) = Command::new(&current_executable).spawn() {
        // If restarting fails, log or handle the error appropriately
        eprintln!("Failed to restart the application: {}", e);
        // Consider restoring the old executable if restarting the new one fails
        fs::rename(&old_executable_path, &current_executable)?;
        return Err(e.into());
    }

    // Exit the current process to complete the update process
    exit(0);

    Ok(())
}

fn main() {
    let new_executable_path = PathBuf::from("path_to_your_new_executable");

    if let Err(e) = restart_application_with_new_version(&new_executable_path) {
        eprintln!("Failed to restart application with new version: {}", e);
    }
}
