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

    let mut parsing_ranges : bool = true;
    let mut total_fresh_ingredients_for_part_1 : i64 = 0;
    let mut total_fresh_ingredients_for_part_2 : i64 = 0;
    let mut ingredient_id_ranges : Vec<(i64,i64)> = Vec::new();
    let mut merged_ingredient_id_ranges : Vec<(i64,i64)> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.is_empty() {
            parsing_ranges = false;
            ingredient_id_ranges.sort_by_key(|range| range.0);

            for &(range_start, range_end) in ingredient_id_ranges.iter() {
                if merged_ingredient_id_ranges.len() == 0 {
                    merged_ingredient_id_ranges.push((range_start, range_end))
                } else {
                    let last_range = merged_ingredient_id_ranges.last_mut().unwrap();
                    
                    if range_start <= last_range.1 {
                        last_range.1 = last_range.1.max(range_end);
                    } else {
                        merged_ingredient_id_ranges.push((range_start, range_end))
                    }
                }
            }

            total_fresh_ingredients_for_part_2 = merged_ingredient_id_ranges.iter().map(|(range_start, range_end)| range_end - range_start + 1).sum();

        }

        if parsing_ranges {
            let numbers_in_range: Vec<i64> = line
                .split('-')
                .map(|s| s.parse().expect("Failed to parse range part to i64"))
                .collect();

            ingredient_id_ranges.push((numbers_in_range[0], numbers_in_range[1]));
        } else if line.is_empty() == false {
            let ingredient_id : i64= line.parse().expect("Failed to parse ingredient id");

            for &(range_start, range_end) in ingredient_id_ranges.iter() {
                if range_start <= ingredient_id && ingredient_id <= range_end {
                    total_fresh_ingredients_for_part_1 += 1;
                    break;
                }
            }

        }
    }

    println!("Sum of all total fresh ingredients for part 1: {}", total_fresh_ingredients_for_part_1);
    println!("Sum of all total fresh ingredients for part 2: {}", total_fresh_ingredients_for_part_2);
}
