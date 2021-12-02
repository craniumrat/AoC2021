extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[grammar = "directions.pest"] // relative to src
struct Directions;

fn main() {

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines {
            if let Ok(value) = line {
                println!("{}", value);
                let instruction = Directions::parse(Rule::instruction, &value).expect("Unexpected parse").next().unwrap();
                let mut amount = 0;
                let mut direction = "";
                for entry in instruction.into_inner()
                {
                    match entry.as_rule() {
                        Rule::direction => {
                            println!("Record value: {}", entry.as_str());
                            direction = entry.as_str();
                        },
                        Rule::value => {
                            println!("Record value: {}", entry.as_str());
                            amount = entry.as_str().parse::<i32>().unwrap();
                        },
                        Rule::WHITESPACE => (),
                        Rule::instruction => (),
                   }

                   update_position_part1(&mut x, &mut y, direction, amount);
                   println!("x: {}, y: {}", x, y);
                }
            }
        }
    }

    println!("{}", x * y);
}

fn update_position_part1(x: &mut i32, y: &mut i32, direction: &str, amount: i32)
{
    println!("x: {}, y: {}, direction: {}, amount: {}", x, y, direction, amount);
    match direction {
        "forward" => {
            *x += amount;
        },
        "backward" => {
            *x -= amount;
        },
        "up" => {
            *y -= amount;
        },
        "down" => {
            *y += amount;
        },
        _ => { unreachable!(); }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}