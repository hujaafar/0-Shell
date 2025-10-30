use std::fs;

pub fn run(args: &[String]) -> Result<(), String> {
    if args.is_empty() { return Err("mkdir: missing operand".into()); }

    let mut parents = false;
    let mut names: Vec<&str> = Vec::new();

    for a in args {
        if a == "-p" { parents = true; } else { names.push(a); }
    }

    for n in names {
        if parents {
            std::fs::create_dir_all(n).map_err(|e| format!("mkdir: {}: {}", n, e))?;
        } else {
            fs::create_dir(n).map_err(|e| format!("mkdir: {}: {}", n, e))?;
        }
    }
    Ok(())
}
