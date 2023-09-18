use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _read_line_result = stdin.read_line(&mut user_input);

    println!("Hello, World.\n{}", user_input);

    Ok(())
}