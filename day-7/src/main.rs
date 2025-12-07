use std::fs::File;
use std::io::{self, Write};
use std::collections::HashSet;
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

    let mut result_for_part_1 : i64 = 0;

    let start_grid : Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line
                .expect("Failed to read line")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect();

    let start_position = start_grid[0].iter().position(|&character| character == 'S').unwrap();

    let mut beams : Vec<(usize, usize)> = vec![(1, start_position)];
    let mut processed : HashSet<(usize, usize)> = HashSet::new();

    while beams.len() > 0 {
        let current_beam = beams.pop().unwrap();
        if processed.contains(&current_beam) {
            continue;
        }

        let mut next_row : usize = current_beam.0 + 1;

        while next_row < start_grid.len() {
            if start_grid[next_row][current_beam.1] == '^' {

                let beam_split_1 = (next_row, current_beam.1 - 1);
                let beam_split_2 = (next_row, current_beam.1 + 1);

                if !processed.contains(&beam_split_1) || !processed.contains(&beam_split_2) {
                    result_for_part_1 += 1;
                }

                if !processed.contains(&beam_split_1) {
                    beams.push(beam_split_1);
                }

                if !processed.contains(&beam_split_2) {
                    beams.push(beam_split_2);
                }
                break;
            }

            next_row += 1;
        }

        processed.insert(current_beam);
    }

    println!("Result for part 1: {}", result_for_part_1);
}
