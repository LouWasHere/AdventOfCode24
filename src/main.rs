mod day1;
mod day2;

fn main() -> std::io::Result<()> {
    let mut path = "input_data/day1.txt";
    let total = day1::day1_1(path)?;

    println!("Total absolute distance: {}", total);

    let similarity_score = day1::day1_2(path)?;

    println!("Total similarity score: {}", similarity_score);

    path = "input_data/day2.txt";
    let count = day2::count_valid_sequence(path)?;

    println!("Count of valid sequences: {}", count);

    Ok(())
}