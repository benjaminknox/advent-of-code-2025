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
    let mut part_one_password : i32 = 0;
    let mut part_two_password : i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let first_char = line.chars().next().unwrap();
        let turns : i32 = line[1..].parse().expect("Expected to find a number");

        if first_char == 'R' {
            for step in 1..=turns {
                if (dial + step).rem_euclid(100) == 0 {
                    part_two_password += 1;
                }
            }
            dial += turns;
        } else {
            for step in 1..=turns {
                if (dial - step).rem_euclid(100) == 0 {
                    part_two_password += 1;
                }
            }
            dial -= turns;
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            part_one_password += 1;
        }

    }

    println!("The password for part one is: {}", part_one_password);
    println!("The password for part two is: {}", part_two_password);
}
