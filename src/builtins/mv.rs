use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 2 { return Err("mv: expected SRC DEST".into()); }

    let (srcs, dest) = args.split_at(args.len() - 1);
    let dest = PathBuf::from(&dest[0]);

    if srcs.len() > 1 {
        if !dest.is_dir() { return Err("mv: last argument must be a directory when moving multiple files".into()); }
        for s in srcs {
            move_one(Path::new(s), &dest.join(Path::new(s).file_name().unwrap()))?;
        }
        return Ok(());
    }

    let src = PathBuf::from(&srcs[0]);
    let target = if dest.is_dir() {
        dest.join(src.file_name().ok_or("mv: bad source name")?)
    } else { dest };
    move_one(&src, &target)
}

fn move_one(src: &Path, dst: &Path) -> Result<(), String> {
    match fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(_) => {
            super::cp::run(&vec![src.to_string_lossy().into(), dst.to_string_lossy().into()])?;
            super::rm::run(&vec![src.to_string_lossy().into()])
        }
    }
}
