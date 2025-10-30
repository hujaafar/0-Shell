use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 2 { return Err("cp: expected SRC DEST".into()); }

    let (srcs, dest) = args.split_at(args.len() - 1);
    let dest = PathBuf::from(&dest[0]);

    if srcs.len() > 1 {
        if !dest.is_dir() { return Err("cp: last argument must be a directory when copying multiple files".into()); }
        for s in srcs {
            copy_file(Path::new(s), &dest.join(Path::new(s).file_name().unwrap()))?;
        }
        return Ok(());
    }

    let src = PathBuf::from(&srcs[0]);
    if dest.is_dir() {
        let target = dest.join(src.file_name().ok_or("cp: invalid source name")?);
        copy_file(&src, &target)
    } else {
        copy_file(&src, &dest)
    }
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    let mut fsrc = fs::File::open(src).map_err(|e| format!("cp: {}: {}", src.display(), e))?;
    let mut fdst = fs::File::create(dst).map_err(|e| format!("cp: {}: {}", dst.display(), e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fsrc.metadata() {
            let _ = fs::set_permissions(dst, fs::Permissions::from_mode(meta.permissions().mode()));
        }
    }

    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = fsrc.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        fdst.write_all(&buf[..n]).map_err(|e| e.to_string())?;
    }
    Ok(())
}
