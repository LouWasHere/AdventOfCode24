use std::fs;
use regex::Regex;

pub fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn process_file_content(path: &str) -> std::io::Result<i32> {
    let content = read_file(path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(&content) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    Ok(sum)
}