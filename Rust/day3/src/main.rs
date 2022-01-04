use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn main() {
    if let Ok(lines) = read_lines("test.txt") {
        let values: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();

        //part1(&values);
        part2(&values);

    }
}

fn part2(values: &Vec<Vec<char>>) -> i32
{
    let length = values[0].len();

    let (ones, zeroes) = partition(values, 0, '1');
    let (mut oxygen_start, mut co2_start) = if ones.len() >= zeroes.len() { (ones, zeroes) } else { (zeroes, ones) };

    for position in 1..length {
        if oxygen_start.len() == 1 {
            break;
        }

        let(ones, zeroes) = partition(&oxygen_start, position, '1');
        oxygen_start = if ones.len() >= zeroes.len() { ones } else { zeroes } ;
    }

    for position in 1..length {
        if co2_start.len() == 1 {
            break;
        }

        let(ones, zeroes) = partition(&co2_start, position, '1');
        co2_start = if ones.len() >= zeroes.len() { zeroes } else { ones } ;
    }

    let o2 = binary_str_to_int(oxygen_start[0].iter().collect::<String>().as_str());
    let co2 = binary_str_to_int(co2_start[0].iter().collect::<String>().as_str());
    
    dbg!(o2);
    dbg!(co2);

    0
}

fn partition(values: &Vec<Vec<char>>, position: usize, value: char) -> (Vec<Vec<char>>, Vec<Vec<char>>)
{
    let mut trues: Vec<Vec<char>> = vec!();
    let mut falses: Vec<Vec<char>> = vec!();
    
    // println!("{:?}", values);
    // let filtered = values.iter().filter(|&cs| { println!("{:?}", cs); cs[position] == value });
    for v in values {
        if v[position] == value {
            trues.push(v.to_vec());
        } else {
            falses.push(v.to_vec());
        }
    }

    (trues, falses)
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