use std::io::Write;

pub fn run(args: &[String], output: &mut dyn Write) -> Result<(), String> {
    let mut first = true;
    for a in args {
        if !first {
            write!(output, " ").map_err(|e| e.to_string())?;
        }
        write!(output, "{}", a).map_err(|e| e.to_string())?;
        first = false;
    }
    writeln!(output).map_err(|e| e.to_string())?;
    Ok(())
}
