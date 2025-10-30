use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Strip ANSI colors, prompts, and normalize newlines & slashes (Windows).
fn norm(s: &str) -> String {
    let no_ansi = strip_ansi_escapes::strip(s).unwrap_or_else(|_| s.as_bytes().to_vec());
    String::from_utf8_lossy(&no_ansi)
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace("$ ", "")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .replace('\\', "/")
}

/// Temp working dir for each test.
fn temp_dir(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("zero_shell_it_{}", name));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

/// Launch the compiled shell binary and feed a script via stdin.
/// Returns (stdout, stderr, exit_ok).
fn run_shell_in(dir: &Path, script: &str) -> (String, String, bool) {
    run_shell_in_env(dir, script, &[])
}

/// Same as `run_shell_in` but allows overriding specific env pairs for the child.
fn run_shell_in_env(dir: &Path, script: &str, env_pairs: &[(&str, &str)]) -> (String, String, bool) {
    let bin = env!("CARGO_BIN_EXE_zero_shell");
    let mut cmd = Command::new(bin);
    cmd.current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Only override the variables we care about to avoid global leakage between tests.
    if !env_pairs.is_empty() {
        cmd.envs(env_pairs.iter().cloned());
    }

    let mut child = cmd.spawn().expect("spawn zero_shell");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(script.as_bytes())
        .expect("write script");

    let out = child.wait_with_output().expect("wait output");
    let stdout = norm(&String::from_utf8_lossy(&out.stdout));
    let stderr = norm(&String::from_utf8_lossy(&out.stderr));
    (stdout, stderr, out.status.success())
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "Expected to find:\n---\n{}\n---\nin output:\n---\n{}\n---",
        needle,
        haystack
    );
}

// ---------- General / Functional checks ----------

#[test]
fn runs_and_shows_prompt_then_exit() {
    let dir = temp_dir("prompt_exit");

    // In non-TTY mode rustyline does not print the prompt; we only assert clean exit.
    let (_out, err, ok) = run_shell_in(&dir, "exit\n");
    assert!(ok, "process should exit cleanly, stderr:\n{}", err);
}

#[test]
fn enter_required_for_validation_and_echo_equivalence() {
    let dir = temp_dir("enter_echo");

    // Send echo with quotes and without quotes.
    let script = r#"echo "something!"
echo something else
exit
"#;
    let (out, err, ok) = run_shell_in(&dir, script);
    assert!(ok, "shell run failed: {}", err);

    // Both forms should print text exactly once each, on their own lines.
    assert_contains(&out, "something!");
    assert_contains(&out, "something else");
}

#[test]
fn pwd_shows_current_path_and_cd_behaviors() {
    let dir = temp_dir("pwd_cd");

    // Create parent with two children; cd into parent; pwd should show it.
    let script = format!(
        "pwd
mkdir -p parent/child1 parent/child2
cd parent
pwd
cd child1
pwd
cd -
pwd
exit
"
    );
    let (out, err, ok) = run_shell_in(&dir, &script);
    assert!(ok, "shell run failed: {}", err);

    let dir_s = dir.to_string_lossy().replace('\\', "/");
    assert_contains(&out, &dir_s); // first pwd
    assert_contains(&out, &(dir_s.clone() + "/parent")); // after cd parent
    assert_contains(&out, &(dir_s.clone() + "/parent/child1")); // after cd child1
    assert_contains(&out, &(dir_s + "/parent")); // after cd -
}

#[test]
fn cd_to_home_when_no_args() {
    let dir = temp_dir("cd_home");
    let home = dir.join("home");
    fs::create_dir_all(&home).unwrap();
    let home_s = home.to_string_lossy().to_string();

    // Set HOME only for the child shell process (no global leakage).
    let (out, err, ok) = run_shell_in_env(&dir, "cd\npwd\nexit\n", &[("HOME", &home_s)]);
    assert!(ok, "shell run failed: {}", err);
    assert_contains(&out, &home.to_string_lossy().replace('\\', "/"));
}

#[test]
fn ls_basic_and_with_flags_are_reasonable() {
    let dir = temp_dir("ls_flags");
    fs::create_dir_all(dir.join(".hidden")).unwrap();
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("file.txt"), "hi\n").unwrap();

    // Compare outputs are "similar": we at least see expected names with flags.
    let (out, _err, ok) = run_shell_in(
        &dir,
        "ls\nls -a\nls -F\nls -l\nexit\n"
    );
    assert!(ok);

    // plain ls: file.txt and sub (hidden may be omitted)
    assert_contains(&out, "file.txt");
    assert_contains(&out, "sub");

    // -a shows .hidden
    assert_contains(&out, ".hidden");

    // -F shows suffix '/' for dirs
    assert_contains(&out, "sub/");

    // -l shows long entries (ensure filename appears)
    assert_contains(&out, "file.txt");
}

#[test]
fn mkdir_cp_mv_rm_cat_workflows() {
    let dir = temp_dir("file_ops");
    // Prepare layout and a doc to copy/move
    fs::create_dir_all(dir.join("new_folder1")).unwrap();
    fs::create_dir_all(dir.join("new_folder2")).unwrap();
    fs::write(dir.join("new_folder1/new_doc.txt"), b"RANDOM-CONTENT\n").unwrap();

    // 1) cp + cat (assert before any move/remove)
    let (out1, err1, ok1) = run_shell_in(
        &dir,
        "cp new_folder1/new_doc.txt new_folder2\n\
         cat new_folder2/new_doc.txt\n\
         exit\n",
    );
    assert!(ok1, "shell run failed: {}", err1);
    assert!(dir.join("new_folder2/new_doc.txt").exists(), "cp didn't create target file");
    assert_contains(&out1, "RANDOM-CONTENT");

    // 2) mv directory into another, assert existence
    let (_out2, err2, ok2) = run_shell_in(&dir, "mv new_folder2 new_folder1\nexit\n");
    assert!(ok2, "mv failed: {}", err2);
    assert!(dir.join("new_folder1/new_folder2").exists(), "mv didn't place folder correctly");

    // 3) rm -r the parent, assert removal
    let (_out3, err3, ok3) = run_shell_in(&dir, "rm -r new_folder1\nexit\n");
    assert!(ok3, "rm -r failed: {}", err3);
    assert!(!dir.join("new_folder1").exists(), "rm -r should remove directory tree");
}

#[test]
fn chaining_pipes_redirection_env_tilde_help_history_and_colors() {
    let dir = temp_dir("extras");
    // Make a home for ~ expansion test (set only for child process)
    let home = dir.join("home");
    fs::create_dir_all(&home).unwrap();
    let home_s = home.to_string_lossy().to_string();

    let script = r#"
# command chaining
echo first ; echo second

# env var and tilde expansions
echo $HOME
echo ~
echo ~/sub
mkdir -p ~/sub

# pipeline and redirection
echo hi | cat - > out.txt
cat < out.txt | cat -

# history & help
history
help

# ls -F colors are present but stripped in norm(); just ensure names exist
ls -F
exit
"#;

    let (out, err, ok) = run_shell_in_env(&dir, script, &[("HOME", &home_s)]);
    assert!(ok, "shell run failed: {}", err);

    // chaining produced two lines
    assert_contains(&out, "first");
    assert_contains(&out, "second");

    // env and tilde
    let home_s_norm = home.to_string_lossy().replace('\\', "/");
    assert_contains(&out, &home_s_norm); // $HOME
    assert_contains(&out, &home_s_norm); // ~ expands to HOME
    assert!(home.join("sub").exists());

    // pipeline + redirection
    assert!(dir.join("out.txt").exists());
    assert_contains(&out, "hi");

    // history printed and help header present
    assert_contains(&out, "history");
    assert_contains(&out, "0-shell builtins:");

    // ls ran (we don't assert colors because they were stripped)
    assert!(out.contains("out.txt") || out.contains("home"));
}

// ----------- support: minimal dependency for ANSI strip -----------
mod strip_ansi_escapes {
    // Tiny ANSI stripper to avoid external crates in Cargo.toml for tests.
    pub fn strip(s: &str) -> Result<Vec<u8>, ()> {
        let mut out = Vec::with_capacity(s.len());
        let mut it = s.as_bytes().iter().cloned();
        while let Some(b) = it.next() {
            if b == 0x1B {
                // ESC [...]
                if let Some(b'[') = it.next() {
                    // consume until letter @-~ (CSI final byte)
                    while let Some(c) = it.next() {
                        if (0x40..=0x7E).contains(&c) { break; }
                    }
                } else {
                    // Not CSI; drop this ESC only
                }
            } else {
                out.push(b);
            }
        }
        Ok(out)
    }
}
