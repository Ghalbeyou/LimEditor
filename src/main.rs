mod input;
mod message;
mod state;
mod filesystem;
mod editor;

use std::{io::{stdout, Write}, path::PathBuf};
use crossterm::{
    execute, terminal,
    event::Event,
};
use state::AppState;
use dialoguer::{Select, theme::ColorfulTheme};


// Creates a new project

fn new_project(app_state: &mut AppState) {
    message::clear_console();
    if app_state.project_opened {
        let _ = message::show_message("Already opened a project.", false);
        return;
    }

    let _ = message::show_message("Waiting for input ...", false);

    let project_directory = message::show_message("Enter the project directory please:", true);

    message::clear_console();

    // Convert the String to PathBuf
    let project_path_buf: PathBuf = project_directory.into();

    app_state.current_directory = Some(project_path_buf.clone());

    
    let files = match filesystem::read_files(app_state) {
        Ok(files) => files,
        Err(err) => {
            let _ = message::show_message(&format!("Error reading files: {}", err), false);
            return;
        }
    };
    app_state.project_opened = true;
    
    // Convert file paths to display strings
    let file_names: Vec<_> = files.iter().map(|file| file.display().to_string()).collect();
    
    // Show file selection prompt
    let selected_file_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a file:")
        .items(&file_names)
        .interact_opt()
        .unwrap_or(None);
    
    match selected_file_index {
        Some(index) => {
            if let Some(selected_file) = files.get(index) {
                let _ = message::show_message(&format!("You selected: {} , Reading it ...", selected_file.display()), false);
                message::clear_console();
                let _ = editor::run_editor(selected_file.display().to_string().as_str());
                app_state.project_opened = false;
            } else {
                let _ = message::show_message("Invalid selection", false);
            }
        }
        None => {
            let _ = message::show_message("No file selected", false);
        }
    }
    
    
}



fn main() {
    // Initialize terminal
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    let mut app_state = state::AppState::new();
    app_state.project_opened = false;

    message::clear_console();

    // Main loop
    loop {
        // Handle events
        if let Ok(event) = crossterm::event::read() {
            if let Event::Key(key_event) = event {
                if let Some(editor_event) = input::process_key_event(key_event) {
                    match editor_event {
                        input::EditorEvent::Quit => break,
                        input::EditorEvent::NewProject => new_project(&mut app_state),
                    }
                }
            }
        }

        // Render UI
        stdout.flush().unwrap();
    }

    // Cleanup terminal
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}
