use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Replaces the old version of the executable with the new one.
///
/// # Arguments
///
/// * `current_executable_path` - Path to the current executable.
/// * `new_executable_path` - Path to the new executable that will replace the current one.
///
/// # Returns
///
/// This function returns `Ok(())` if the replacement was successful, or an `io::Error` if it failed.
pub fn replace_old_version(current_executable_path: &Path, new_executable_path: &Path) -> io::Result<()> {
    #[cfg(target_family = "unix")]
    {
        // On Unix-like systems, we can often directly overwrite the existing executable.
        // However, if the executable is currently running, this might not work as expected on all systems.
        // It's safer to rename the current executable and then move the new one into its place.
        let backup_path = current_executable_path.with_extension("old");
        fs::rename(current_executable_path, &backup_path)?;
        fs::rename(new_executable_path, current_executable_path)?;
    }

    #[cfg(target_family = "windows")]
    {
        // On Windows, a running executable cannot be overwritten.
        // Therefore, we first rename the current executable.
        let backup_path = current_executable_path.with_extension("old");
        fs::rename(current_executable_path, &backup_path)?;

        // Then, try to rename the new executable to the original name.
        match fs::rename(new_executable_path, current_executable_path) {
            Ok(_) => (),
            Err(e) => {
                // If the rename fails, attempt to restore the original executable.
                let _ = fs::rename(&backup_path, current_executable_path);
                return Err(e);
            },
        }
    }

    // Optionally, you can delete the backup (old version) here if you're confident the new version works well.
    // fs::remove_file(backup_path)?;

    Ok(())
}

