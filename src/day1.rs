use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn read_numbers(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
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

    Ok((left_numbers, right_numbers))
}

pub fn day1_1(path: &str) -> io::Result<i32> {
    let (mut left_numbers, mut right_numbers) = read_numbers(path)?;

    left_numbers.sort();
    right_numbers.sort();

    let mut total = 0;
    for i in 0..left_numbers.len() {
        total += (left_numbers[i] - right_numbers[i]).abs();
    }

    Ok(total)
}

pub fn day1_2(path: &str) -> io::Result<i32> {
    let (left_numbers, right_numbers) = read_numbers(path)?;

    let mut right_counts = HashMap::new();
    for &num in &right_numbers {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let mut total_similarity_score = 0;
    for &num in &left_numbers {
        if let Some(&count) = right_counts.get(&num) {
            total_similarity_score += num * count;
        }
    }

    Ok(total_similarity_score)
}