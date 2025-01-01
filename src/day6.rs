use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

pub fn process_day6_file(path: &str) -> io::Result<usize> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let mut visited = HashSet::new();
    let mut direction = (-1, 0); // Start facing upwards
    let mut position = (0, 0);

    // Find the initial position of '^'
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            position = (i, j);
            break;
        }
    }

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Up, Right, Down, Left
    let mut dir_index = 0;

    loop {
        let (x, y) = position;
        visited.insert(position);

        let new_x = x as isize + direction.0;
        let new_y = y as isize + direction.1;

        if new_x < 0 || new_y < 0 || new_x >= matrix.len() as isize || new_y >= matrix[0].len() as isize {
            break; // Out of bounds
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if matrix[new_x][new_y] == '.' {
            matrix[x][y] = 'X';
            matrix[new_x][new_y] = '^';
            position = (new_x, new_y);
        } else if matrix[new_x][new_y] == '#' {
            dir_index = (dir_index + 1) % 4; // Turn right
            direction = directions[dir_index];
        } else if matrix[new_x][new_y] == 'X' {
            matrix[x][y] = 'X';
            matrix[new_x][new_y] = '^';
            position = (new_x, new_y);
        }
    }

    Ok(visited.len())
}