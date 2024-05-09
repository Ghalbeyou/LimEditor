use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum EditorEvent {
    Quit,
    NewProject,
}

pub fn process_key_event(event: KeyEvent) -> Option<EditorEvent> {
    match event {
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        } => Some(EditorEvent::Quit),
        KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
        } => Some(EditorEvent::NewProject),
        
        _ => None,
    }
}
