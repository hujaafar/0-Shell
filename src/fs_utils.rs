use std::fs::{self, Metadata};
use std::path::Path;

// unix-only perms helpers
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn mode_to_string(meta: &Metadata) -> String {
    use std::os::unix::fs::FileTypeExt;
    let ft = meta.file_type();
    let mut s = String::new();
    s.push(if ft.is_dir() {
        'd'
    } else if ft.is_symlink() {
        'l'
    } else if ft.is_char_device() {
        'c'
    } else if ft.is_block_device() {
        'b'
    } else if ft.is_fifo() {
        'p'
    } else if ft.is_socket() {
        's'
    } else {
        '-'
    });

    let mode = meta.permissions().mode();
    let rwx = [(mode >> 6) & 0o7, (mode >> 3) & 0o7, mode & 0o7];
    for m in rwx {
        s.push(if m & 0b100 != 0 { 'r' } else { '-' });
        s.push(if m & 0b010 != 0 { 'w' } else { '-' });
        s.push(if m & 0b001 != 0 { 'x' } else { '-' });
    }
    s
}

#[cfg(not(unix))]
fn mode_to_string(_meta: &Metadata) -> String {
    String::from("----------")
}

#[cfg(unix)]
pub fn is_executable(meta: &Metadata) -> bool {
    meta.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
pub fn is_executable(_meta: &Metadata) -> bool {
    false
}

pub fn classify_suffix(path: &Path, meta: &Metadata) -> char {
    let ft = meta.file_type();
    if ft.is_dir() { '/' }
    else if ft.is_symlink() { '@' }
    else if is_executable(meta) { '*' }
    else { ' ' }
}

pub fn long_entry<P: AsRef<Path>>(p: P, meta: &Metadata) -> String {
    let name = p.as_ref().file_name().and_then(|s| s.to_str()).unwrap_or("");
    let mode = mode_to_string(meta);
    let size = meta.len();
    let modified = meta.modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{} {:>8} {:>10} {}  {}", mode, 0, size, name, modified)
}

pub fn read_dir_sorted(path: &Path) -> std::io::Result<Vec<fs::DirEntry>> {
    let mut v: Vec<_> = fs::read_dir(path)?.filter_map(|e| e.ok()).collect();
    v.sort_by_key(|e| e.file_name());
    Ok(v)
}
