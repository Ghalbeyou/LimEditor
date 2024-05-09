use std::path::PathBuf;

pub struct AppState {
    pub current_file: Option<PathBuf>,
    pub current_line: usize,
    pub current_directory: Option<PathBuf>,
    pub project_opened: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            current_file: None,
            current_line: 0,
            current_directory: None,
            project_opened: false,
        }
    }
    
}
