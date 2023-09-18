use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    for _i in 0..3 {
        let mut input = String::new();
        let read_line_result = stdin.read_line(&mut input);
        lines.push(match read_line_result {
            Ok(_line) => input,
            Err(error) => panic!("Problem reading line: {:?}", error),
        });
    }

    let int: i32 = match lines.get(0) {
        Some(x) => match x.trim().parse() {
            Ok(x) => x,
            Err(error) => panic!("Problem parsing int: {:?}", error),
        },
        None => 0,
    };

    let double: f64 = match lines.get(1) {
        Some(x) => match x.trim().parse() {
            Ok(x) => x,
            Err(error) => panic!("Problem parsing double: {:?}", error),
        },
        None => 0.0,
    };
    
    let user_input = lines.get(2).unwrap();

    println!(
        "{0}\n{1:.1}\n{2}{3}",
        int + 4,
        double + 4.0,
        "Hacker Rank ",
        user_input
    );

    Ok(())
}
