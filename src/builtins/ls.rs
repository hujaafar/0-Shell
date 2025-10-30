use std::path::{Path, PathBuf};
use std::fs;
use crate::fs_utils::{read_dir_sorted, long_entry, classify_suffix};

#[derive(Default)]
struct Opts { a: bool, l: bool, fclass: bool }

pub fn run(args: &[String]) -> Result<(), String> {
    let mut opts = Opts::default();
    let mut paths: Vec<PathBuf> = Vec::new();

    for a in args {
        if a.starts_with('-') {
            for ch in a.chars().skip(1) {
                match ch {
                    'a' => opts.a = true,
                    'l' => opts.l = true,
                    'F' => opts.fclass = true,
                    _ => return Err(format!("ls: unknown flag -{}", ch)),
                }
            }
        } else {
            paths.push(PathBuf::from(a));
        }
    }

    if paths.is_empty() { paths.push(PathBuf::from(".")); }

    let multi = paths.len() > 1;

    for (i, p) in paths.iter().enumerate() {
        let md = fs::symlink_metadata(p)
            .map_err(|e| format!("ls: {}: {}", p.display(), e))?;
        if md.is_dir() {
            if multi { println!("{}:", p.display()); }
            list_dir(p, &opts)?;
            if i + 1 < paths.len() { println!(); }
        } else {
            print_entry(p, &md, &opts)?;
        }
    }
    Ok(())
}

fn list_dir(dir: &Path, opts: &Opts) -> Result<(), String> {
    let entries = read_dir_sorted(dir).map_err(|e| e.to_string())?;
    for e in entries {
        let name = e.file_name();
        let name = name.to_string_lossy();
        if !opts.a && name.starts_with('.') { continue; }
        let path = e.path();
        let md = std::fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        print_entry(&path, &md, opts)?;
    }
    Ok(())
}

fn print_entry(path: &Path, md: &fs::Metadata, opts: &Opts) -> Result<(), String> {
    if opts.l {
        println!("{}", long_entry(path, md));
    } else if opts.fclass {
        let suffix = classify_suffix(path, md);
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let colored = match suffix {
            '/' => format!("\x1b[34m{}\x1b[0m/", name),
            '*' => format!("\x1b[32m{}\x1b[0m*", name),
            '@' => format!("\x1b[36m{}\x1b[0m@", name),
            _   => name.to_string(),
        };
        println!("{}", colored);
    } else {
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        println!("{}", name);
    }
    Ok(())
}
