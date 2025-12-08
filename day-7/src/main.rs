use std::fs::File;
use std::io::{self, Write};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
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

    let mut beams_for_part_1 : Vec<(usize, usize)> = vec![(1, start_position)];
    let mut processed_in_part_1 : HashSet<(usize, usize)> = HashSet::new();

    while beams_for_part_1.len() > 0 {
        let current_beam = beams_for_part_1.pop().unwrap();
        if processed_in_part_1.contains(&current_beam) {
            continue;
        }

        let mut next_row : usize = current_beam.0 + 1;

        while next_row < start_grid.len() {
            if start_grid[next_row][current_beam.1] == '^' {
                let beam_split_1 = (next_row, current_beam.1 - 1);
                let beam_split_2 = (next_row, current_beam.1 + 1);

                if !processed_in_part_1.contains(&beam_split_1) || !processed_in_part_1.contains(&beam_split_2) {
                    result_for_part_1 += 1;
                }

                if !processed_in_part_1.contains(&beam_split_1) {
                    beams_for_part_1.push(beam_split_1);
                }

                if !processed_in_part_1.contains(&beam_split_2) {
                    beams_for_part_1.push(beam_split_2);
                }
                break;
            }

            next_row += 1;
        }

        processed_in_part_1.insert(current_beam);
    }

    let mut result_for_part_2 : i128 = 0;

    let mut beams_for_part_2 : HashMap<(usize, usize), i128> = HashMap::new();
    let mut queue : VecDeque<(usize,usize)> = VecDeque::new();
    let start_coordinates : (usize,usize) = (0, start_position);

    beams_for_part_2.insert(start_coordinates, 1);
    queue.push_back(start_coordinates);

    while let Some((mut row, col)) = queue.pop_front() {
        let count = beams_for_part_2.get(&(row, col)).cloned().unwrap_or(0);
        if count == 0 {
            continue;
        }

        beams_for_part_2.remove(&(row, col));

        loop {
            row += 1;

            if row >= start_grid.len() {
                result_for_part_2 += count;
                break;
            }

            let tile = start_grid[row][col];

            if tile == '^' {
                if col > 0 {
                    let left = (row, col - 1);
                    let prev = beams_for_part_2.get(&left).cloned().unwrap_or(0);
                    let new = prev + count;
                    beams_for_part_2.insert(left, new);
                    if new > prev {
                        queue.push_back(left);
                    }
                } else {
                    result_for_part_2 += count;
                }

                if col + 1 < start_grid[0].len() {
                    let right = (row, col + 1);
                    let prev = beams_for_part_2.get(&right).cloned().unwrap_or(0);
                    let new = prev + count;
                    beams_for_part_2.insert(right, new);
                    if new > prev {
                        queue.push_back(right);
                    }
                } else {
                    result_for_part_2 += count;
                }

                break;
            }
        }
    }

    println!("Result for part 1: {}", result_for_part_1);
    println!("Result for part 2: {}", result_for_part_2);
}
