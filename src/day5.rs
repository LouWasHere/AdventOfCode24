use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

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

fn build_graph(rules: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for (x, y) in rules {
        graph.entry(x.clone()).or_insert_with(Vec::new).push(y.clone());
        graph.entry(y).or_insert_with(Vec::new);
    }

    graph
}

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

pub fn valid_orderings(path1: &str, path2: &str) -> io::Result<i32> {
    let rules = read_rules_from_file(path1)?;
    let graph = build_graph(rules);
    let sequences = read_sequences_from_file(path2)?;
    let mut total_middle_value = 0;

    for sequence in sequences {
        if validate_sequence(&graph, &sequence) {
            if let Some(middle_value) = sequence.get(sequence.len() / 2) {
                if let Ok(value) = middle_value.parse::<i32>() {
                    total_middle_value += value;
                }
            }
        }
    }

    Ok(total_middle_value)
}