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

    let mut voltage_sum : i64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let battery_bank : Vec<i32> = line
            .chars()
            .map(|ch| ch.to_digit(10).expect("Not a digit") as i32)
            .collect();

        let mut max_joltage = 0;

        for tens_index in 0..battery_bank.len() - 1 {
            let tens_digit = battery_bank[tens_index];
            let ones_digit = *battery_bank[tens_index + 1..].iter().max().unwrap();
            let jolts = tens_digit * 10 + ones_digit;
            if jolts > max_joltage {
                max_joltage = jolts;
            }
        }

        voltage_sum += max_joltage as i64;
    }

    println!("Sum of all voltages for part 1: {}", voltage_sum);

}
