use std::path::PathBuf;
use crate::shell::Shell;

pub fn run(sh: &mut Shell, args: &[String]) -> Result<(), String> {
    let target = if args.is_empty() {
        std::env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else if args[0] == "-" {
        if let Some(prev) = sh.prev_dir() { prev.display().to_string() } else { return Ok(()); }
    } else {
        args[0].clone()
    };

    sh.chdir(PathBuf::from(target))
}
