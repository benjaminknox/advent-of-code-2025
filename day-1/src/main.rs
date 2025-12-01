use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

fn main() {
    let default_value = "my-input.txt";

    print!("Enter input file: (default: {}): ", default_value);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input = input.trim().to_string();

    if input.is_empty() {
        input = default_value.to_string();
    }

    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dial : i32 = 50;
    let mut password : i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let first_char = line.chars().next().unwrap();
        let turns : &i32 = &line[1..].parse().expect("Expected to find a number");

        if first_char == 'R' {
            dial += turns;
        } else {
            dial -= turns;
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            password += 1;
        }

    }

    println!("The password is: {}", password);
}
