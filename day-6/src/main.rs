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

    let mut equations_for_part_1 : HashMap<usize, (String, Vec<i128>)> = HashMap::new();
    let mut character_matrix : Vec<Vec<char>> = Vec::new();

    let mut result_for_part_1 : i128 = 0;
    let mut result_for_part_2 : i128 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        character_matrix.push(line.chars().collect());

        let columns : Vec<String>= line.trim().split_whitespace().map(|s| s.to_string()).collect();

        for (column_index, column_value) in columns.iter().enumerate() {

            let equation_row : &mut (String, Vec<i128>) = equations_for_part_1.entry(column_index).or_insert((" ".to_string(), Vec::new()));

            if column_value == "*" || column_value == "+" {
                equation_row.0 = column_value.to_string();
            } else {
                let number : i128 = column_value.parse().expect("Failed to parse to i128");

                equation_row.1.push(number);
            }
        }
    }

    for (operation, numbers) in equations_for_part_1.values() {
        if operation == "+" {
            result_for_part_1 += numbers.iter().sum::<i128>();
        } else {
            result_for_part_1 += numbers.iter().product::<i128>();
        }
    }

    // Rotate 90 degrees
    let character_row_count = character_matrix.len();
    let character_column_count = character_matrix[0].len();
    let mut rotated_character_matrix : Vec<Vec<char>> = (0..character_column_count)
        .rev()
        .map(|column_index| {
            (0..character_row_count) 
                .map(|row_index| {
                    character_matrix[row_index][column_index]
                }).collect()
        }).collect();

    let mut equations_for_part_2 : Vec<i128> = Vec::new();

    for equation in rotated_character_matrix {
        let line : String = equation
            .iter()
            .filter(|character| character.is_digit(10))
            .collect::<String>()
            .trim()
            .to_string();

        if line.is_empty() {
            continue;
        }

        let number : i128 = line
            .parse()
            .expect("Could not convert line to i128 ");

        equations_for_part_2.push(number);

        let operation = *equation.last().unwrap();

        if operation == '*' || operation == '+' {
            if operation == '+' {
                result_for_part_2 += equations_for_part_2.iter().sum::<i128>();
            } else {
                result_for_part_2 += equations_for_part_2.iter().product::<i128>();
            }

            equations_for_part_2.clear();
        }
    }

    println!("Result for part 1: {}", result_for_part_1);
    println!("Result for part 2: {}", result_for_part_2);

}
