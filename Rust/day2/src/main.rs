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
                let instruction = Directions::parse(Rule::instruction, &value).expect("Unexpected parse").next().unwrap();
                for record in instruction.into_inner()
                {
                    let mut amount = 0;
                    let mut direction = "";
                    match record.as_rule() {
                        Rule::direction => {
                            direction = record.as_str();
                        },
                        Rule::value => {
                            amount = record.as_str().parse::<i32>().unwrap();
                        },
                        _ => { unreachable!(); }
                   }

                   update_position(&mut x, &mut y, direction, amount);
                   println!("x: {}, y: {}", x, y);
                }
            }
        }
    }
}

fn update_position(x: &mut i32, y: &mut i32, direction: &str, amount: i32)
{
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