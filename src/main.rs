use std::io::{self, Write};

mod day3;
mod day4;
mod day5;
mod day6;
mod day8;
mod day9;
mod day10;
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
            match i {
                3 => day3::run::run(),
                4 => day4::run::run(),
                5 => day5::run::run(),
                6 => day6::run::run(),
                8 => day8::run::run(),
                9 => day9::run::run(),
                10 => day10::run::run(),
                _ => println!("Day {} not implemented", i),
            }
        },
        Err(..) => println!("Invalid input: {}", trimmed),
    };
}
