# 0‑Shell

A minimalist Unix‑like shell written **entirely in Rust**, designed for embedded or constrained environments.  
All commands are implemented from scratch with Rust’s standard library — **no external binaries spawned**.

---

## ✨ Features (Mandatory Scope)

- Prompted interactive shell loop (`$ `), exits gracefully on **Ctrl+D** (EOF) or `exit [code]`.
- Built‑in commands (no external processes):
  - `echo`
  - `pwd`
  - `cd` (supports `cd` to `$HOME` and `cd ~`)
  - `mkdir`
  - `ls` with flags **`-l`**, **`-a`**, **`-F`**
  - `cat`
  - `cp`
  - `rm` with **`-r`**
  - `mv` (handles _move into directory_ and _rename_; copies across devices for files)
- Parser with simple quoting/escaping:
  - Supports **single quotes** `'...'`, **double quotes** `"..."`, and **backslash escaping** outside single quotes.
- Clear, friendly error messages (e.g., `cd: <path>: <os error>`).

> **Constraints purposely not implemented** (per subject): no piping (`|`), no redirection (`>`, `<`), no globbing (`*`).

---

## 🧱 Project Structure

```
0-shell/
├── src/
│   ├── builtins.rs   # Implementations of built-in commands
│   ├── main.rs       # REPL loop and command dispatch
│   └── parser.rs     # Tokenizer with quotes/escape handling
├── Cargo.toml
└── README.md
```

---

## ⚙️ Build & Run

```bash
# Build & run optimized
cargo run --release

# You should see the prompt:
$
```

To exit:

```bash
$ exit          # or: exit 2
```

The shell also exits on **Ctrl+D** (EOF).

---

## 🧪 Quick Demo (Copy/Paste)

> Note: Output formatting (permissions, timestamps) may vary by OS/WSL — that’s normal.

```bash
$ pwd


$ ls -F
Cargo.lock*
Cargo.toml*
parent/
src/
target/

$ mkdir new_folder1
$ mkdir new_folder2
$ printf 'hello from my shell
' > new_folder1/new_doc.txt   # done in host shell (redirection is not part of 0‑Shell)

$ cp new_folder1/new_doc.txt new_folder2
$ ls -l new_folder2
-rwxrwxrwx  1       20   1762624151 new_doc.txt

$ cat new_folder1/new_doc.txt
hello from my shell

$ mv new_folder2 new_folder1
$ ls -F
Cargo.lock*
Cargo.toml*
new_folder1/
parent/
README.md*
src/
target/

$ rm -r new_folder1
$ ls -F
Cargo.lock*
Cargo.toml*
parent/
README.md*
src*
target/
```

---

