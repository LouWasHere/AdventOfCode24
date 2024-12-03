mod day1;

fn main() -> std::io::Result<()> {
    let path = "input_data/day1.txt";
    let total = day1::day1_1(path)?;

    println!("Total absolute distance: {}", total);

    let similarity_score = day1::day1_2(path)?;

    println!("Total similarity score: {}", similarity_score);

    Ok(())
}