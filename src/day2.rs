use std::fs::File;
use std::io::{self, BufRead};

fn read_test(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut all_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        all_numbers.push(numbers);
    }

    Ok(all_numbers)
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let mut increasing = None;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if increasing.is_none() {
            increasing = Some(diff > 0);
        } else if increasing.unwrap() != (diff > 0) {
            return false;
        }
    }

    true
}

pub(crate) fn count_valid_sequence(path: &str) -> io::Result<usize> {
    let all_numbers = read_test(path)?;
    let count = all_numbers.iter().filter(|&numbers| is_valid_sequence(numbers)).count();
    Ok(count)
}