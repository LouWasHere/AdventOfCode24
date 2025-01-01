use std::fs;
use std::io::{self, BufRead};

// Function that reads the input file into a matrix of characters
fn read_file_to_matrix(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(matrix)
}

// Validates whether a position on the matrix is valid (within bounds).
fn is_valid_position(x: isize, y: isize, rows: isize, cols: isize) -> bool {
    x >= 0 && x < rows && y >= 0 && y < cols
}

// For part 1, checks possible directions for XMAS strings when an X is found. Counts instances of XMAS located
fn search_word(matrix: &[Vec<char>], word: &str) -> usize {
    let directions = [
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
        (-1, 0), // up
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1) // up-left
    ];
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;
    let word_len = word.len() as isize;
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i as usize][j as usize] == word.chars().next().unwrap() {
                for &(dx, dy) in &directions {
                    let mut found = true;
                    for k in 0..word_len {
                        let nx = i + k * dx;
                        let ny = j + k * dy;
                        if !is_valid_position(nx, ny, rows, cols) || matrix[nx as usize][ny as usize] != word.chars().nth(k as usize).unwrap() {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

// For part 2, searches for instances of A. When A is found, check the diagonally adjacent cells and see whether the diagonals both contain MAS instances
fn search_mas_pairs(matrix: &[Vec<char>]) -> usize {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i as usize][j as usize] == 'A' {
                if i > 0 && i < rows - 1 && j > 0 && j < cols - 1 {
                    let mut flipped = false;

                    // Check first diagonal (down-right and up-left)
                    let left_up = matrix[(i + 1) as usize][(j - 1) as usize];
                    match left_up {
                        'M' => {}
                        'S' => {
                            flipped = true;
                        }
                        _ => continue,
                    }
                    let right_down = matrix[(i - 1) as usize][(j + 1) as usize];
                    match right_down {
                        'M' => {
                            if !flipped {
                                continue;
                            }
                        }
                        'S' => {
                            if flipped {
                                continue;
                            }
                        }
                        _ => continue,
                    }

                    flipped = false;

                    // Check second diagonal (down-left and up-right)
                    let right_up = matrix[(i + 1) as usize][(j + 1) as usize];
                    match right_up {
                        'M' => flipped = true,
                        'S' => {}
                        _ => continue,
                    }
                    let left_down = matrix[(i - 1) as usize][(j - 1) as usize];
                    match left_down {
                        'M' => {
                            if flipped {
                                continue;
                            }
                        }
                        'S' => {
                            if !flipped {
                                continue;
                            }
                        }
                        _ => continue,
                    }
                    count += 1;
                }
            }
        }
    }
    count
}

// Provides the search string and returns instances of it
pub fn xmas_instances(path: &str) -> io::Result<i32> {
    let matrix = read_file_to_matrix(path)?;
    let word = "XMAS";
    let count = search_word(&matrix, word) as i32;
    Ok(count)
}

// Provides nothing but returns the number of X-MAS patterns found
pub fn mas_pairs(path: &str) -> io::Result<i32> {
    let matrix = read_file_to_matrix(path)?;
    let count = search_mas_pairs(&matrix) as i32;
    Ok(count)
}