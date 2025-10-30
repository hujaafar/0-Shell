use std::fs;
use std::path::PathBuf;

pub fn run(args: &[String]) -> Result<(), String> {
    if args.is_empty() { return Err("rm: missing operand".into()); }

    let mut recursive = false;
    let mut paths: Vec<PathBuf> = Vec::new();

    for a in args {
        if a == "-r" || a == "-R" { recursive = true; }
        else { paths.push(PathBuf::from(a)); }
    }

    if paths.is_empty() { return Err("rm: missing operand".into()); }

    for p in paths {
        let meta = fs::symlink_metadata(&p).map_err(|e| format!("rm: {}: {}", p.display(), e))?;
        if meta.is_dir() {
            if !recursive { return Err(format!("rm: {}: is a directory (use -r)", p.display())); }
            fs::remove_dir_all(&p).map_err(|e| format!("rm: {}: {}", p.display(), e))?;
        } else {
            fs::remove_file(&p).map_err(|e| format!("rm: {}: {}", p.display(), e))?;
        }
    }
    Ok(())
}
