use std::io::{self, Write};

mod day3;

fn main() {
    let mut input_text = String::new();
    print!("Enter Day: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            println!("Running Day {}...", i);
            day3::run::run();
        },
        Err(..) => println!("Invalid input: {}", trimmed),
    };
}
