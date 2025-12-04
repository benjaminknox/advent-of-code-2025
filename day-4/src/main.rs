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

    let mut grid : Vec<Vec<char>> = Vec::new();

    let mut total_accessible_rolls : i64 = 0;

    for line in reader.lines() {
        let row : Vec<char> = line.expect("Failed to read line").chars().collect();

        grid.push(row);
    }

    for (row, columns) in grid.iter().enumerate() {
        for (column, &space_value) in columns.iter().enumerate() {
            if space_value != '@' {
                continue;
            }

            let mut adjacent_roll_total = 0;

            if column >= 1 && row >= 1 && grid[row - 1][column - 1] == '@'{
              adjacent_roll_total += 1; // top left
            }

            if row >= 1 && grid[row - 1][column] == '@' {
              adjacent_roll_total += 1; // top
            }

            if row >= 1 && column + 1 < columns.len() && grid[row - 1][column + 1] == '@' {
              adjacent_roll_total += 1; //top right
            }

            if column >= 1 && grid[row][column - 1] == '@'{
              adjacent_roll_total += 1; // left
            }

            if row + 1 < grid.len() && column >= 1 && grid[row + 1][column - 1] == '@'{
              adjacent_roll_total += 1; // bottom left
            }

            if row + 1 < grid.len() && grid[row + 1][column] == '@'{
              adjacent_roll_total += 1; // bottom
            }

            if row + 1 < grid.len() && column + 1 < columns.len() && grid[row + 1][column + 1] == '@'{
              adjacent_roll_total += 1; // bottom right
            }

            if column + 1 < columns.len() && grid[row][column + 1] == '@'{
              adjacent_roll_total += 1; // right
            }

            if adjacent_roll_total < 4 {
                total_accessible_rolls += 1;
            }

        }
    }

    println!("Sum of all accessible rolls: {}", total_accessible_rolls);
}
