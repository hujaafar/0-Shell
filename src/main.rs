mod shell;
mod lexer;
mod fs_utils;
mod builtins;

use rustyline::{DefaultEditor, error::ReadlineError};
use shell::Shell;

fn main() {
    let mut sh = Shell::new();

    let mut rl = DefaultEditor::new().expect("init line editor");
    let hist_path = std::env::temp_dir().join("zero_shell_history.txt");
    let _ = rl.load_history(&hist_path);

    loop {
        let prompt = sh.prompt();
        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim_end();
                if !trimmed.is_empty() {
                    let _ = rl.add_history_entry(trimmed);
                    sh.push_history(trimmed);
                }
                if let Err(e) = sh.handle_line(trimmed) {
                    eprintln!("\x1b[31m{}\x1b[0m", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!(); // Ctrl+C: newline and keep going
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!(); // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(&hist_path);
}
