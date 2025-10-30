use std::env;

/// Small lexer with quotes/escapes
pub fn split_words(input: &str) -> Result<Vec<String>, &'static str> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => { in_single = !in_single; }
            '"'  if !in_single => { in_double = !in_double; }
            '\\' if !in_single => {
                if let Some(n) = chars.next() { buf.push(n); }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !buf.is_empty() { out.push(std::mem::take(&mut buf)); }
            }
            _ => buf.push(c),
        }
    }

    if in_single || in_double { return Err("unclosed quotes"); }
    if !buf.is_empty() { out.push(buf); }

    Ok(out)
}

/// Expand $VAR and leading ~ (no ${} support)
pub fn expand_vars_and_tilde(mut argv: Vec<String>) -> Vec<String> {
    for t in &mut argv {
        if t.starts_with('~') {
            if let Ok(home) = env::var("HOME") {
                if t == "~" { *t = home; }
                else if t.starts_with("~/") { *t = format!("{}/{}", home, &t[2..]); }
            }
        }
        let mut out = String::new();
        let mut it = t.chars().peekable();
        while let Some(c) = it.next() {
            if c == '$' {
                let mut name = String::new();
                while let Some(&nc) = it.peek() {
                    if nc.is_ascii_alphanumeric() || nc == '_' { name.push(nc); it.next(); } else { break; }
                }
                if name.is_empty() { out.push('$'); }
                else { out.push_str(&env::var(&name).unwrap_or_default()); }
            } else {
                out.push(c);
            }
        }
        *t = out;
    }
    argv
}
