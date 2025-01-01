use std::collections::{HashMap};
use std::fs;
use std::io::{self, BufRead};

// Reads the input file into an array of (string, string) tuples
fn read_rules_from_file(path: &str) -> io::Result<Vec<(String, String)>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            rules.push((parts[0].to_string(), parts[1].to_string()));
        }
    }

    Ok(rules)
}

// Parses the tuples to create a HashMap to represent the rules that the sequences must follow in the format: X: {Y,...} where X must come before Y
fn build_graph(rules: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for (x, y) in rules {
        graph.entry(x.clone()).or_insert_with(Vec::new).push(y.clone());
        graph.entry(y).or_insert_with(Vec::new);
    }

    graph
}

// Reads the sequences to be tested from the input file.
fn read_sequences_from_file(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut sequences = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let sequence: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        sequences.push(sequence);
    }

    Ok(sequences)
}

// Function that queries the HashMap to validate whether a sequence is valid (just by checking if a digit is a dependant of the ones succeeding it)
fn validate_sequence(graph: &HashMap<String, Vec<String>>, sequence: &[String]) -> bool {
    let mut position = HashMap::new();

    for (index, page) in sequence.iter().enumerate() {
        position.insert(page, index);
    }

    for (x, dependents) in graph {
        if let Some(&x_pos) = position.get(x) {
            for y in dependents {
                if let Some(&y_pos) = position.get(y) {
                    if x_pos >= y_pos {
                        return false;
                    }
                }
            }
        }
    }

    true
}

// Function that takes in an invalid sequence and sorts it according to the rules in the HashMap. Uses a recursive function within this function
fn sort_incorrect_sequence(graph: &HashMap<String, Vec<String>>, sequence: &[String]) -> Vec<String> {
    let mut sequence = sequence.to_vec();
    let mut sorted_sequence = Vec::new();

    // Recursive sub-function that takes a value and checks if any values in its list appear in the sequence. If yes, call the function again on that value
    // If not, then place that value at the end of the sorted sequence (or as close to the end as possible) and dequeue it from the stack
    // Repeat until all values have been placed
    fn process_value(
        graph: &HashMap<String, Vec<String>>,
        sequence: &mut Vec<String>,
        sorted_sequence: &mut Vec<String>,
        value: String,
    ) {
        if let Some(dependents) = graph.get(&value) {
            for dependent in dependents {
                if sequence.contains(dependent) {
                    process_value(graph, sequence, sorted_sequence, dependent.clone());
                }
            }
        }
        if let Some(pos) = sequence.iter().position(|x| *x == value) {
            sorted_sequence.push(sequence.remove(pos));
        }
    }

    while !sequence.is_empty() {
        let value = sequence[0].clone();
        process_value(&graph, &mut sequence, &mut sorted_sequence, value);
    }

    sorted_sequence
}

// Function that exists to minimize code repetition in the valid_orderings() and invalid_sequence_totals() functions since the underlying result is calculated similarly
fn process_sequences<F>(path1: &str, path2: &str, validate: F) -> io::Result<i32>
where
    F: Fn(&HashMap<String, Vec<String>>, &mut Vec<String>) -> bool,
{
    let rules = read_rules_from_file(path1)?;
    let graph = build_graph(rules);
    let mut sequences = read_sequences_from_file(path2)?;
    let mut total_middle_value = 0;

    for sequence in &mut sequences {
        if validate(&graph, sequence) {
            if let Some(middle_value) = sequence.get(sequence.len() / 2) {
                if let Ok(value) = middle_value.parse::<i32>() {
                    total_middle_value += value;
                }
            }
        }
    }

    Ok(total_middle_value)
}

// Just returns the total of the middle values of valid sequences
pub fn valid_orderings(path1: &str, path2: &str) -> io::Result<i32> {
    process_sequences(path1, path2, |graph, sequence| validate_sequence(graph, sequence))
}

// Returns the total of the middle values of all invalid sequences after being made valid by sorting
pub fn invalid_sequence_totals(path1: &str, path2: &str) -> io::Result<i32> {
    process_sequences(path1, path2, |graph, sequence| {
        if !validate_sequence(graph, sequence) {
            let sorted_sequence = sort_incorrect_sequence(graph, sequence);
            *sequence = sorted_sequence;
            true
        } else {
            false
        }
    })
}