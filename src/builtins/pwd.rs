pub fn run() -> Result<(), String> {
    match std::env::current_dir() {
        Ok(p) => { println!("{}", p.display()); Ok(()) }
        Err(e) => Err(e.to_string())
    }
}
