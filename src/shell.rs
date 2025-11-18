use crate::lexer::{split_words, expand_vars_and_tilde};
use crate::builtins;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Default)]
pub struct Shell {
    prev_dir: Option<PathBuf>,
    history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prev_dir: None,
            history: Vec::new(),
        }
    }

    pub fn push_history(&mut self, line: &str) {
        self.history.push(line.to_string());
    }

    pub fn prompt(&self) -> String {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
        let home = env::var("HOME").unwrap_or_default();
        let mut disp = cwd.display().to_string();

        if !home.is_empty() && disp.starts_with(&home) {
            disp = disp.replacen(&home, "~", 1);
        }

        format!("\x1b[36m{}\x1b[0m $ ", disp)
    }

    pub fn handle_line(&mut self, raw: &str) -> Result<(), String> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Ok(());
        }

        // Support chaining: cmd1 ; cmd2 ; cmd3
        for segment in raw.split(';') {
            let segment = segment.trim();
            if !segment.is_empty() {
                self.exec_one(segment)?;
            }
        }
        Ok(())
    }

    fn exec_one(&mut self, raw: &str) -> Result<(), String> {
        let mut cmd_part = raw.to_string();
        let mut infile: Option<String> = None;
        let mut outfile: Option<String> = None;

        // ---- OUTPUT REDIRECT ----
        if let Some(pos) = cmd_part.rfind('>') {
            let t = cmd_part[pos + 1..].trim().to_string();
            if t.is_empty() {
                return Err("syntax error: expected filename after '>'".into());
            }
            outfile = Some(t);
            cmd_part.truncate(pos);
            cmd_part = cmd_part.trim_end().to_string();
        }

        // ---- INPUT REDIRECT ----
        if let Some(pos) = cmd_part.rfind('<') {
            let t = cmd_part[pos + 1..].trim().to_string();
            if t.is_empty() {
                return Err("syntax error: expected filename after '<'".into());
            }
            infile = Some(t);
            cmd_part.truncate(pos);
            cmd_part = cmd_part.trim_end().to_string();
        }

        // ---- PIPE STAGES ----
        let stages: Vec<&str> = cmd_part
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if stages.is_empty() {
            return Ok(());
        }

        // ---- FIRST INPUT ----
        let mut current_in: Box<dyn Read> = if let Some(f) = infile {
            Box::new(File::open(&f).map_err(|e| format!("{}: {}", f, e))?)
        } else {
            Box::new(io::empty())
        };

        let mut last_output: Vec<u8> = Vec::new();

        // ==== PIPE LOOP ====
        for (i, stage) in stages.iter().enumerate() {
            let mut argv = split_words(stage).map_err(|e| e.to_string())?;
            argv = expand_vars_and_tilde(argv);
            if argv.is_empty() {
                continue;
            }

            last_output.clear();
            {
                let mut out: Box<dyn Write> = Box::new(&mut last_output);
                self.dispatch(&argv, &mut current_in, &mut out)?;
            }

            current_in = Box::new(io::Cursor::new(last_output.clone()));

            if i + 1 == stages.len() {
                break;
            }
        }

        // ==== FINAL OUTPUT ====
        if let Some(of) = outfile {
            let mut f = File::create(&of).map_err(|e| format!("{}: {}", of, e))?;
            f.write_all(&last_output).map_err(|e| e.to_string())?;
        } else {
            io::stdout()
                .write_all(&last_output)
                .map_err(|e| e.to_string())?;
            // ❗ NO println! → prevents extra blank line
        }

        Ok(())
    }

    // ================= DISPATCH BUILTINS ==================
    fn dispatch(
        &mut self,
        argv: &[String],
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> Result<(), String> {
        match argv[0].as_str() {
            "exit" => std::process::exit(0),
            "echo" => builtins::echo::run(&argv[1..], output),
            "pwd" => builtins::pwd::run(),
            "cd" => builtins::cd::run(self, &argv[1..]),
            "ls" => builtins::ls::run(&argv[1..]),
            "cat" => builtins::cat::run_io(&argv[1..], input, output),
            "cp" => builtins::cp::run(&argv[1..]),
            "rm" => builtins::rm::run(&argv[1..]),
            "mv" => builtins::mv::run(&argv[1..]),
            "mkdir" => builtins::mkdir::run(&argv[1..]),
            "help" => builtins::help::run(),
            "history" => {
                for (i, h) in self.history.iter().enumerate() {
                    writeln!(output, "{}  {}", i + 1, h).map_err(|e| e.to_string())?;
                }
                Ok(())
            }
            other => Err(format!("Command '{}' not found", other)),
        }
    }

    // ================== DIRECTORY TRACKING ==================
    pub fn chdir(&mut self, to: PathBuf) -> Result<(), String> {
        let cur = env::current_dir().map_err(|e| e.to_string())?;
        env::set_current_dir(&to).map_err(|e| e.to_string())?;
        self.prev_dir = Some(cur);
        Ok(())
    }

    pub fn prev_dir(&self) -> Option<PathBuf> {
        self.prev_dir.clone()
    }
}
