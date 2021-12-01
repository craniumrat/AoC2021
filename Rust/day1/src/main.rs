use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("test.txt") {
        let mut prev = 0;
        let mut total = 0;
        for line in lines {
            if let Ok(value) = line {
                let num: i32 = value.parse().unwrap();
                if num > prev {
                    total += 1;
                }

                prev = num;
            }
        }

        println!("{}", total - 1);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}