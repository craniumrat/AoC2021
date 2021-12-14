use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("test.txt") {
        let values: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();

        part1(&values);
        part2(&values);

    }
}

fn part2(values: &Vec<Vec<char>>) -> i32
{
    let mut filtered = filter(values, 0, '1');
    let mut i = 0;
    println!("Filtered count: {}", filtered.len());
    // while filtered.len() > 1 {
        i += 1;
        println!("Filtered count: {}, i: {}", filtered.len(), i);
        filtered = filter(&filtered, i, '1');
        println!("Filtered count: {}, i: {}", filtered.len(), i);
    // }

    0
}

fn filter(values: &Vec<Vec<char>>, position: usize, value: char) -> Vec<Vec<char>>
{
    let mut new_values = vec!(vec!());
    println!("{:?}", values);
    let filtered = values.iter().filter(|&cs| { println!("{:?}", cs); cs[position] == value });
    for f in filtered {
        println!("{:?}", f);
        new_values.push(f.to_vec());
    }

    new_values
}

fn binary_str_to_int(s: &str) -> i32
{
    let length = s.len();
    let mut output = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            output += i32::pow(2, (length - i - 1).try_into().unwrap());
        }
    }

    output
}

fn part1(values: &Vec<Vec<char>>) -> i32
{
    let length = values[0].len();

    let mut counts: Vec<usize> = vec!(0; length);
    let values_count = values.len();

    println!("Length: {}", length);
    println!("Counts len: {}", counts.len());

    for binary in values {
        for (pos, ch) in binary.iter().enumerate() {
            if *ch == '1' {
                counts[pos] += 1;
            }
        }
    }

    println!("{:?}", counts);

    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");

    for i in 0..length {
        if counts[i] >= values_count / 2 {
            gamma_str.push('1');
            epsilon_str.push('0');
        }
        else
        {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    let gamma = binary_str_to_int(gamma_str.as_str());
    let epsilon = binary_str_to_int(epsilon_str.as_str());

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Solution: {}", gamma * epsilon);
    gamma * epsilon
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}