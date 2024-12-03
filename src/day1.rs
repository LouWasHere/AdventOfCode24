use std::fs::File;
use std::io::{self, BufRead};

pub fn day1_1(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_numbers.push(left);
                right_numbers.push(right);
            }
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut total = 0;
    for i in 0..left_numbers.len() {
        total += (left_numbers[i] - right_numbers[i]).abs();
    }

    Ok(total)
}