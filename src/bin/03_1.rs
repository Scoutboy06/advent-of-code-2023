use std::fs;

fn main() {
    let input = fs::read_to_string("src/in/3.in").expect("Input data not found (3.in)");

    let mut sum: u32 = 0;
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        let mut is_connected = false;
        let mut curr_n: Option<u32> = None;

        for col in 0..cols {
            let ch = &grid[row][col];

            match ch {
                '.' => {
                    if is_connected {
                        is_connected = false;

                        if let Some(n) = curr_n {
                            sum += n;
                        }
                    }

                    curr_n = None;
                }
                _ if ch.is_ascii_digit() => {
                    // Check if ch is adjacent to symbol
                    if !is_connected {
                        is_connected = (row > 0 && is_symbol(grid[row - 1][col])) // Up
                            || (col > 0 && is_symbol(grid[row][col - 1])) // Left
                            || (row < rows - 1 && is_symbol(grid[row + 1][col])) // Down
                            || (col < cols - 1 && is_symbol(grid[row][col + 1])) // RIght
    
                            || (row > 0 && col > 0 && is_symbol(grid[row - 1][col - 1])) // Up left
                            || (row > 0 && col < cols - 1 && is_symbol(grid[row - 1][col + 1])) // Up right
                            || (row < rows - 1 && col < cols - 1 && is_symbol(grid[row + 1][col + 1])) // Down right
                            || (row < rows - 1 && col > 0 && is_symbol(grid[row + 1][col - 1])); // Down left
                    }

                    if let Some(n) = &mut curr_n {
                        *n *= 10;
                        *n += ch.to_digit(10).unwrap();
                    } else {
                        curr_n = Some(ch.to_digit(10).unwrap());
                    }
                }
                _ => {
                    if is_connected {
                        sum += curr_n.unwrap();
                        curr_n = None;
                    }
                }
            }
        }

        // End of line
        if curr_n.is_some() && is_connected {
            sum += curr_n.unwrap();
        }
    }

    println!("Answer: {}", sum);
}

fn is_symbol(ch: char) -> bool {
    match ch {
        '.' => false,
        _ if ch.is_ascii_digit() => false,
        _ => true,
    }
}
