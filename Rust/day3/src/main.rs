use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("test.txt") {
        let values: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
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

        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..length {
            println!("Counts[i]: {}, i: {}, output: {}", counts[i], i, gamma);
            if counts[i] >= values_count / 2 {
                println!("2 ^ {}", (length - i - 1));
                gamma += i32::pow(2, (length - i - 1).try_into().unwrap());
            }
            else
            {
                println!("2 ^ {}", (length - i - 1));
                epsilon += i32::pow(2, (length - i - 1).try_into().unwrap());
            }
        }

        println!("Gamma: {}", gamma);
        println!("Epsilon: {}", epsilon);
        println!("Solution: {}", gamma * epsilon);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}