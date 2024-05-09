use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
};

pub fn run_editor(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut stdout = io::stdout();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut prev_cursor_x = cursor_x;
    let mut prev_cursor_y = cursor_y;

    terminal::enable_raw_mode()?;
    execute!(stdout, Hide)?;

    // Render initial text
    render_text(&mut stdout, &mut reader, cursor_x, cursor_y)?;

    execute!(stdout, MoveTo(cursor_x, cursor_y))?;
    execute!(stdout, Show)?; // Show cursor
    stdout.flush()?; // Flush stdout to ensure the cursor is shown

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('x') => break,
                KeyCode::Left => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                }
                KeyCode::Right => {
                    // Move cursor right
                    cursor_x += 1;
                }
                KeyCode::Up => {
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    }
                }
                KeyCode::Down => {
                    // Move cursor down
                    cursor_y += 1;
                }
                _ => {}
            }
            
            // Check if cursor position has changed
            if cursor_x != prev_cursor_x || cursor_y != prev_cursor_y {
                // Clear the entire screen
                execute!(stdout, Clear(ClearType::All))?;
                // Render text with updated cursor position
                render_text(&mut stdout, &mut reader, cursor_x, cursor_y)?;
                // Move the cursor to the new position
                execute!(stdout, MoveTo(cursor_x, cursor_y))?;
                // Update previous cursor position
                prev_cursor_x = cursor_x;
                prev_cursor_y = cursor_y;
            }
        }
    }

    execute!(stdout, Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn render_text(
    stdout: &mut io::Stdout,
    reader: &mut BufReader<File>,
    cursor_x: u16,
    cursor_y: u16,
) -> io::Result<()> {
    reader.seek(SeekFrom::Start(0))?; // Seek back to the beginning of the file

    let mut current_cursor_x = cursor_x;
    let mut current_cursor_y = cursor_y;

    for line in reader.lines() {
        if let Ok(line) = line {
            for c in line.chars() {
                write!(stdout, "{}", c)?;
                current_cursor_x += 1;
                execute!(stdout, MoveTo(current_cursor_x, current_cursor_y))?;
            }
            current_cursor_y += 1;
            current_cursor_x = 0; // Reset cursor_x for new line
            execute!(stdout, MoveTo(current_cursor_x, current_cursor_y))?;
        }
        write!(stdout, "\n")?; // Print newline after each line
        current_cursor_y += 1;
        current_cursor_x = 0; // Reset cursor_x for new line
        execute!(stdout, MoveTo(current_cursor_x, current_cursor_y))?;
    }

    Ok(())
}
