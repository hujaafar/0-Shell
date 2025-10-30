pub fn run() -> Result<(), String> {
    println!("0-shell builtins:
    echo [args...]             - print arguments
    cd [dir] | cd -            - change directory (to HOME if no dir)
    pwd                        - print current directory
    ls [-l] [-a] [-F] [path]   - list files
    cat [files...] | cat -     - print files/stdin
    cp SRC... DEST             - copy files
    mv SRC... DEST             - move/rename files
    rm [-r] PATH...            - remove files or directories
    mkdir [-p] PATH...         - make directories
    history                    - show command history
    help                       - this help

Features: env vars ($HOME), ~ expansion, command chaining (;), pipelines (|),
redirection (<, >), Ctrl+C friendly, colored ls -F, prompt shows current directory.");
    Ok(())
}
