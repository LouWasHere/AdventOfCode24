use std::fs;
use regex::Regex;

// Simply reads the file into a string
fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

// Applies a RegEx formula to capture valid uses of a "mul()" function in the string and then total the multiplications contained within
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

// Applies a slightly more complicated RegEx formula to capture "mul()"s, "do()"s and "don't()"s, then modifies the behavior of the sum depending on what is captured
pub fn process_file_part2(path: &str) -> std::io::Result<i32> {
    let content = read_file(path)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\)|don't\(\))").unwrap();
    let mut part_2_score = 0;
    let mut multiplications_enabled = true;

    for line in content.lines() {
        for cap in re.captures_iter(line) {
            match &cap[0] {
                "do()" => multiplications_enabled = true,
                "don't()" => multiplications_enabled = false,
                _ => {
                    if let Some(inner_cap) = cap.get(1).zip(cap.get(2)) {
                        let x: i32 = inner_cap.0.as_str().parse().unwrap();
                        let y: i32 = inner_cap.1.as_str().parse().unwrap();
                        if multiplications_enabled {
                            part_2_score += x * y;
                        }
                    }
                }
            }
        }
    }

    Ok(part_2_score)
}