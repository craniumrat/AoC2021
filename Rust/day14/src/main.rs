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
    println!("{}", output);
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

fn output_value(input: &str) -> i32 {
    let mut counts = HashMap::new();

    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut minimum = i32::MAX;
    let mut maximum = 0;

    for (_, value) in counts.iter() {
        if *value > maximum {
            maximum = *value;
        }

        if *value < minimum {
            minimum = *value;
        }
    }

    maximum - minimum
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