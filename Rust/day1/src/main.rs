use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn main() {
    if let Ok(lines) = read_lines("test.txt") {

        let values: Vec<i32> = lines.map(|line| line.unwrap().parse().unwrap()).collect();
        part1(&values);
        part2(&values);
    }
}

fn part2(values: &Vec<i32>) -> i32
{
    let mut prev_sum = 0;
    let mut total = 0;

    for (a, b, c) in values.iter().tuple_windows()
    {
        let sum = a + b + c;
        if sum > prev_sum
        {
            total += 1;
        }

        prev_sum = sum;
    }

    println!("Part 2: {}", total - 1);
    total - 1
}

fn part1(values: &Vec<i32>) -> i32
{
    let mut prev = 0;
    let mut total = 0;
    for value in values {
        if value > &prev {
            total += 1;
        }

        prev = *value;
    }

    println!("Part 1: {}", total - 1);
    total - 1
}

// fn read_lines_to_vec<P>(filename: P) -> std::io::Result<Vec<i32>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;

//     let mut data = Vec::new();
//     file.read_to_end(&mut data);

//     return Ok(data);
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}