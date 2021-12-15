use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TEST_START: &str = "NNCB";
const START: &str = "OHFNNCKCVOBHSSHONBNF";

fn main() {
    let graph = make_graph();
    let mut output_str = String::from(START);

    for _ in 0..10 {
        output_str = step(output_str.as_str(), &graph);
    }

    let output = output_value(output_str.as_str());
    println!("Part 1: {}", output);

    // So naive!
    // for i in 0..40 {
    //     println!("{}", i);
    //     output_str = step(output_str.as_str(), &graph);
    // }

    // let output = output_value(output_str.as_str());
    // println!("Part 2: {}", output);

    let mut counts_map = str_to_counts_hash(START);
    for i in 0..40 {
        println!("{}", i);
        counts_map = step_hash(&counts_map, &graph);
    }

    let output2 = output_value_hash(&counts_map, START);
    println!("Part 2: {}", output2);
}

fn make_graph() -> HashMap<String, char> {
    let mut graph = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(pair_string) = line {
                let key = String::from(&pair_string[0..2]);
                let value = pair_string.chars().last().unwrap();
                graph.insert(key, value);
            }
        }
    }

    graph
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn str_to_counts_hash(input: &str) -> HashMap<String, u64> {
    let mut output = HashMap::new();

    for (a, b) in input.chars().tuple_windows() {
        let mut key = String::new();
        key.push(a);
        key.push(b);

        *output.entry(key).or_insert(0) += 1;
    }

    output
}

fn step_hash(input: &HashMap<String, u64>, graph: &HashMap<String, char>) -> HashMap<String, u64> {
    let mut output = HashMap::new();

    for (pair, count) in input.iter() {
        let first = pair.chars().nth(0).unwrap();
        let second = pair.chars().nth(1).unwrap();

        let value = *graph.get(pair.as_str()).unwrap();

        let mut first_pair = String::new();
        first_pair.push(first);
        first_pair.push(value);

        let mut second_pair = String::new();
        second_pair.push(value);
        second_pair.push(second);

        *output.entry(first_pair).or_insert(0) += count;
        *output.entry(second_pair).or_insert(0) += count;
    }
    
    output
}

fn step(input: &str, graph: &HashMap<String, char>) -> String {
    let mut output = String::new();

    for (a, b) in input.chars().tuple_windows() {
        let mut key = String::new();
        key.push(a);
        key.push(b);

        let value = *graph.get(key.as_str()).unwrap();
        output.push(a);
        output.push(value);
    }

    output.push(input.chars().last().unwrap());
    output
}

fn output_value(input: &str) -> u64 {
    let mut counts = HashMap::new();

    for (a, b) in input.chars().tuple_windows() {
        let mut k = String::new();
        k.push(a);
        k.push(b);

        *counts.entry(k).or_insert(0) += 1;
    }

    output_value_hash(&counts, input)
}

fn output_value_hash(input: &HashMap<String, u64>, start_string: &str) -> u64 {
    let mut minimum = u64::MAX;
    let mut maximum = 0;

    let mut character_counts = HashMap::new();

    for (key, value) in input.iter() {
        let first_char = key.chars().nth(0).unwrap();
        let second_char = key.chars().nth(1).unwrap();

        *character_counts.entry(first_char).or_insert(0) += value;
        *character_counts.entry(second_char).or_insert(0) += value;
    }

    let first_char = start_string.chars().nth(0).unwrap();
    let last_char = start_string.chars().last().unwrap();

    *character_counts.entry(first_char).or_insert(0) += 1;
    *character_counts.entry(last_char).or_insert(0) += 1;

    for (_, value) in character_counts.iter() {
        if *value > maximum {
            maximum = *value;
        }

        if *value < minimum {
            minimum = *value;
        }
    }

    (maximum - minimum) / 2
}

#[test]
fn test_step() {
    let mut graph = HashMap::new();
    graph.insert(String::from("NN"), 'C');
    graph.insert(String::from("NC"), 'B');
    graph.insert(String::from("CB"), 'H');
    graph.insert(String::from("CN"), 'C');
    graph.insert(String::from("NB"), 'B');
    graph.insert(String::from("BC"), 'B');
    graph.insert(String::from("CH"), 'B');
    graph.insert(String::from("HB"), 'C');

    let mut output = step(TEST_START, &graph);
    assert_eq!(output, "NCNBCHB");

    output = step(output.as_str(), &graph);
    assert_eq!(output, "NBCCNBBBCBHCB");
}

#[test]
fn test_make_graph() {
    let graph = make_graph();
    assert_eq!(graph.get("SV"), Some(&'O'));
}

#[test]
fn test_steps() {
    let mut output = String::from(TEST_START);

    let mut graph = HashMap::new();
    graph.insert(String::from("NN"), 'C');
    graph.insert(String::from("NC"), 'B');
    graph.insert(String::from("CB"), 'H');
    graph.insert(String::from("CN"), 'C');
    graph.insert(String::from("NB"), 'B');
    graph.insert(String::from("BC"), 'B');
    graph.insert(String::from("CH"), 'B');
    graph.insert(String::from("HB"), 'C');

    for _ in 0..2 {
        output = step(output.as_str(), &graph);
        println!("{:?}", output);
    }

    assert_eq!(output, "NBCCNBBBCBHCB");
}

#[test]
fn test_output_value() {
    let output = output_value("AAABB");
    assert_eq!(output, 1);
}

#[test]
fn test_example() {

    let mut output = String::from(TEST_START);

    let mut graph = HashMap::new();
    graph.insert(String::from("CH"), 'B');
    graph.insert(String::from("HH"), 'N');
    graph.insert(String::from("CB"), 'H');
    graph.insert(String::from("NH"), 'C');
    graph.insert(String::from("HB"), 'C');
    graph.insert(String::from("HC"), 'B');
    graph.insert(String::from("HN"), 'C');
    graph.insert(String::from("NN"), 'C');
    graph.insert(String::from("BH"), 'H');
    graph.insert(String::from("NC"), 'B');
    graph.insert(String::from("NB"), 'B');
    graph.insert(String::from("BN"), 'B');
    graph.insert(String::from("BB"), 'N');
    graph.insert(String::from("BC"), 'B');
    graph.insert(String::from("CC"), 'N');
    graph.insert(String::from("CN"), 'C');

    for _ in 0..10 {
        output = step(output.as_str(), &graph);
        println!("{:?}", output);
    }

    assert_eq!(output.len(), 3073);
    let output_val = output_value(output.as_str());

    assert_eq!(output_val, 1588);
}

#[test]
fn test_str_to_counts_map() {
    let counts_map = str_to_counts_hash(TEST_START);
    assert_eq!(counts_map.get("NN"), Some(&1));
    assert_eq!(counts_map.get("NC"), Some(&1));
    assert_eq!(counts_map.get("CB"), Some(&1));
}

#[test]
fn test_step_hash() {
    let mut counts_map = str_to_counts_hash(TEST_START);

    let mut graph = HashMap::new();
    graph.insert(String::from("CH"), 'B');
    graph.insert(String::from("HH"), 'N');
    graph.insert(String::from("CB"), 'H');
    graph.insert(String::from("NH"), 'C');
    graph.insert(String::from("HB"), 'C');
    graph.insert(String::from("HC"), 'B');
    graph.insert(String::from("HN"), 'C');
    graph.insert(String::from("NN"), 'C');
    graph.insert(String::from("BH"), 'H');
    graph.insert(String::from("NC"), 'B');
    graph.insert(String::from("NB"), 'B');
    graph.insert(String::from("BN"), 'B');
    graph.insert(String::from("BB"), 'N');
    graph.insert(String::from("BC"), 'B');
    graph.insert(String::from("CC"), 'N');
    graph.insert(String::from("CN"), 'C');

    counts_map = step_hash(&counts_map, &graph);

    //NCNBCHB
    assert_eq!(counts_map.get("NC"), Some(&1));
    assert_eq!(counts_map.get("CN"), Some(&1));
    assert_eq!(counts_map.get("NB"), Some(&1));
    assert_eq!(counts_map.get("BC"), Some(&1));
    assert_eq!(counts_map.get("CH"), Some(&1));
    assert_eq!(counts_map.get("HB"), Some(&1));
}

#[test]
fn test_output_value_hash() {
    let mut input = HashMap::new();

    //NBCCNBBBCBHCB -- N: 2, B: 6, C: 4, H: 1
    input.insert(String::from("NB"), 2);
    input.insert(String::from("BC"), 2);
    input.insert(String::from("CC"), 1);
    input.insert(String::from("CN"), 1);
    input.insert(String::from("BB"), 2);
    input.insert(String::from("CB"), 2);
    input.insert(String::from("BH"), 1);

    let output = output_value_hash(&input, "NNCB");
    assert_eq!(output, 5); // B: 6 - H: 1 = 5
}

#[test]
fn test_step_hash_iteration() {
    let mut counts_map = str_to_counts_hash(TEST_START);

    let mut graph = HashMap::new();
    graph.insert(String::from("CH"), 'B');
    graph.insert(String::from("HH"), 'N');
    graph.insert(String::from("CB"), 'H');
    graph.insert(String::from("NH"), 'C');
    graph.insert(String::from("HB"), 'C');
    graph.insert(String::from("HC"), 'B');
    graph.insert(String::from("HN"), 'C');
    graph.insert(String::from("NN"), 'C');
    graph.insert(String::from("BH"), 'H');
    graph.insert(String::from("NC"), 'B');
    graph.insert(String::from("NB"), 'B');
    graph.insert(String::from("BN"), 'B');
    graph.insert(String::from("BB"), 'N');
    graph.insert(String::from("BC"), 'B');
    graph.insert(String::from("CC"), 'N');
    graph.insert(String::from("CN"), 'C');

    for _ in 0..10 {
        counts_map = step_hash(&counts_map, &graph);
    }

    let output_val = output_value_hash(&counts_map, TEST_START);

    assert_eq!(output_val, 1588);
}