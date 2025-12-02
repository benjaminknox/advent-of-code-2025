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

    let code_list = reader.lines()
        .next()
        .expect("File is empty")
        .expect("Failed to read line");

    let mut result_for_part_1 : i64 = 0;
    let mut result_for_part_2 : i64 = 0;

    for code_range_string in code_list.split(',') {
        let code_range_nums: Vec<&str> = code_range_string.split('-').collect();    
        let start: i64 = code_range_nums[0].parse().expect("Failed to parse start");
        let end: i64 = code_range_nums[1].parse().expect("Failed to parse end");

        for code in start..=end {
            let code_str : String = code.to_string();

            let code_length = code_str.len();
            
            if code_length % 2 == 0 {
                let half_code_length : usize = code_length / 2;
                let first_half : &str = &code_str[..half_code_length];
                let second_half : &str = &code_str[half_code_length..];

                if first_half == second_half {
                    result_for_part_1 += code;
                }
            }

            let mut is_invalid_part2 = false;
            for substring_length in 1..=code_length / 2 {
                if code_length % substring_length == 0 {
                    let pattern = &code_str[..substring_length];
                    if pattern.repeat(code_length / substring_length) == code_str {
                        is_invalid_part2 = true;
                        break;
                    }
                }
            }
            if is_invalid_part2 {
                result_for_part_2 += code;
            }
        }
    }

    println!("Sum of all invalid ids for part 1: {}", result_for_part_1);
    println!("Sum of all invalid ids for part 2: {}", result_for_part_2);

}
