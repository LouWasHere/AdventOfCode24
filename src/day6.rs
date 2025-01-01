use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

fn read_file_to_matrix(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    Ok(matrix)
}

fn populate_visited(matrix: &mut Vec<Vec<char>>, visited_with_dir: &mut HashSet<(usize, usize, usize)>) -> HashSet<(usize, usize)> {
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
        visited_with_dir.insert((x, y, dir_index));

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

    visited
}

pub fn process_day6_file(path: &str) -> io::Result<usize> {
    let mut matrix = read_file_to_matrix(path)?;
    let mut visited_with_dir = HashSet::new();
    let visited = populate_visited(&mut matrix, &mut visited_with_dir);
    Ok(visited.len())
}

fn check_for_loop(new_matrix: &mut Vec<Vec<char>>) -> bool {
    let mut visited_with_dir = HashSet::new();
    let mut direction = (-1, 0); // Start facing upwards
    let mut position = (0, 0);

    // Find the initial position of '^'
    for (i, row) in new_matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            position = (i, j);
            break;
        }
    }

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Up, Right, Down, Left
    let mut dir_index = 0;

    loop {
        let (x, y) = position;
        if visited_with_dir.contains(&(x, y, dir_index)) {
            return true; // Loop detected
        }
        visited_with_dir.insert((x, y, dir_index));

        let new_x = x as isize + direction.0;
        let new_y = y as isize + direction.1;

        if new_x < 0 || new_y < 0 || new_x >= new_matrix.len() as isize || new_y >= new_matrix[0].len() as isize {
            break; // Out of bounds
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        match new_matrix[new_x][new_y] {
            '.' | 'X' => {
                new_matrix[x][y] = 'X';
                new_matrix[new_x][new_y] = '^';
                position = (new_x, new_y);
            }
            '#' => {
                dir_index = (dir_index + 1) % 4; // Turn right
                direction = directions[dir_index];
            }
            _ => {}
        }
    }

    false // No loop detected
}

pub fn process_day6_file_part2(path: &str) -> io::Result<usize> {
    let mut matrix = read_file_to_matrix(path)?;
    let mut visited_with_dir = HashSet::new();
    let visited = populate_visited(&mut matrix, &mut visited_with_dir);

    let mut count = 0;

    for (x, y) in &visited {
        let mut new_matrix = read_file_to_matrix(path)?;
        new_matrix[*x][*y] = '#';
        //println!("Testing {}, {}", x, y);

        if check_for_loop(&mut new_matrix) {
            count += 1;
        }
    }

    Ok(count)
}