pub fn run(args: &[String]) -> Result<(), String> {
    let mut first = true;
    for a in args {
        if !first { print!(" "); }
        print!("{}", a);
        first = false;
    }
    println!();
    Ok(())
}
