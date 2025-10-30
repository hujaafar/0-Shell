use std::fs::File;
use std::io::{self, Read, Write};



/// Piping-aware: reads from `input` when args empty or "-" is present.
pub fn run_io(args: &[String], input: &mut dyn Read, output: &mut dyn Write) -> Result<(), String> {
    if args.is_empty() || args.iter().any(|a| a == "-") {
        let mut buf = [0u8; 8192];
        loop {
            let n = input.read(&mut buf).map_err(|e| e.to_string())?;
            if n == 0 { break; }
            output.write_all(&buf[..n]).map_err(|e| e.to_string())?;
        }
        return Ok(());
    }

    for p in args {
        let mut f = File::open(p).map_err(|e| format!("cat: {}: {}", p, e))?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        output.write_all(&buf).map_err(|e| e.to_string())?;
    }
    Ok(())
}
