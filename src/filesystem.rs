use std::fs;
use std::io;
use std::path::PathBuf;
use crate::state::AppState; // Adjust the path as per your project structure

pub fn read_files(app_state: &mut AppState) -> Result<Vec<PathBuf>, io::Error> {

    let mut file_paths = Vec::new();

    // Check if current_directory is set
    if let Some(current_dir) = &app_state.current_directory {
        // Check if current_directory exists and is a directory
        if current_dir.is_dir() {
            // Read directory entries
            if let Ok(entries) = fs::read_dir(current_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Get the path of the entry
                        let path = entry.path();
                        // Check if it's a file
                        if path.is_file() {
                            file_paths.push(path);
                        }
                    }
                }
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to read directory entries"));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Current directory is not a directory"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Current directory is not set"));
    }

    Ok(file_paths)
}
