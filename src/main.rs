mod day1;
mod day2;
mod day3;

fn main() -> std::io::Result<()> {
    let mut path = "input_data/day1.txt";
    let total = day1::day1_1(path)?;
    println!("Total absolute distance: {}", total);

    let similarity_score = day1::day1_2(path)?;
    println!("Total similarity score: {}", similarity_score);

    path = "input_data/day2.txt";
    let count = day2::count_valid_sequence(path)?;
    println!("Count of valid sequences: {}", count);

    let new_count = day2::count_valid_or_correctable_sequences(path)?;
    println!("Count of valid or correctable sequences: {}", new_count);

    path = "input_data/day3.txt";
    let sum = day3::process_file_content(path)?;
    println!("Sum of multiplication functions: {}", sum);

    let new_sum = day3::process_file_part2(path)?;
    println!("Sum of enabled multiplication functions: {}", new_sum);

    Ok(())
}