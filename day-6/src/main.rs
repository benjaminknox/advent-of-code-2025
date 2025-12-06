use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;
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

    let mut equations : HashMap<usize, (String, Vec<i128>)> = HashMap::new();

    let mut result_for_part_1 : i128 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let columns : Vec<String>= line.trim().split_whitespace().map(|s| s.to_string()).collect();

        for (column_index, column_value) in columns.iter().enumerate() {

            let equation_row : &mut (String, Vec<i128>) = equations.entry(column_index).or_insert((" ".to_string(), Vec::new()));

            if column_value == "*" || column_value == "+" {
                equation_row.0 = column_value.to_string();
            } else {
                let number : i128 = column_value.parse().expect("Failed to parse to i128");

                equation_row.1.push(number);
            }
        }
    }

    for (operation, numbers) in equations.values() {
        if operation == "+" {
            result_for_part_1 += numbers.iter().sum::<i128>();
        } else {
            result_for_part_1 += numbers.iter().product::<i128>();
        }
    }

    println!("Result for part 1: {}", result_for_part_1);

}
