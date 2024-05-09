use std::io::{stdin, stdout, Write};

pub fn show_message(message: &str, wait_for_input: bool) -> String {
    let mut stdout = stdout();
    writeln!(stdout, "{}", message).unwrap();
    stdout.flush().unwrap();

    if wait_for_input {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    } else {
        String::new()
    }
}
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape sequence for clearing console
    stdout().flush().unwrap();
}