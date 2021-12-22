use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//const TEST_HEX_STRING: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
const HEX_STRING: &str = "########...#.###.##..#.....#####....#.#...#..##..##.###.....####...#.#..#.###...#.#.##...#..##.###..##......##....##....#.#..#...##.###..#.#.#####..##.##....#.#####.#.######...#.####....#.#.####..#.####.#.#..##......#....#.##...#...#...#......#...#...#.#.....###..#.##..#...####.##....#.#.#...##...####.#####.####.###.##.###.##...#.#.#...###.######....#.#.#.#..#.#....#####.#.##...##...##...##...#.#.#####.#...#.#######..#...##..#..#...#..####....###..#..##.##..#.#.##...#####......#.#.###.####...#.#.###.#.##.#.";

fn main() {
    let mut image = get_image("./input.txt");
    let map = to_hash_map(HEX_STRING);

    // println!("{}", count_lit_pixels(&image));

    /* NOTE: We messed up because the 0th value in HEX_STRING is '#'. 
    * That means for every element in the infinite page outside our input, all values
    * map to '[[000][000][000]]', the corresponding value in HEX_STRING will replace
    * all values to 1. So the entire infinite page will be all #'s. (We cannot get a non-infinite count)
    * In the next enhance, all elements outside input will be [[111][111][111]], which will 
    * correspond to the 511th value in HEX_STRING which is '.'. So all those infinte '#'s will
    * revert back to 0.
    *
    * We can solve this by passing a default value for the enhance function to indicate what values 
    * to fill in by default.
    */
    let mut default = false;

    for _ in 0..2 {
        image = enhance(&image, &map, &mut default);
    }    

    println!("Part 1: {}", count_lit_pixels(&image));

    for _ in 0..48 {
        image = enhance(&image, &map, &mut default);
    }

    println!("Part 2: {}", count_lit_pixels(&image));
}

fn count_lit_pixels(image: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    
    for row in image {
        count += row.iter().fold(0i32, |mut sum, val| if *val { sum += 1; sum} else { sum });
    } 

    count
}

fn enhance(input: &Vec<Vec<bool>>, map: &HashMap<usize, bool>, default: &mut bool) -> Vec<Vec<bool>> {
    //Since the original image is infinite going in all directions, 
    //pixels two to the left and top and pixels two to the right and bottom will
    //be affected by the enchance function. We should consider those also.
    let  mut output = Vec::with_capacity((input.len() + 2) * (input.len() + 2));

    for i in 0..input.len() + 2 {
        let mut output_row = vec!();

        for j in 0..input.len() + 2 {
            let mut array = vec!();

            // println!("For ({}, {})", i, j);

            //Get (i - 1, j - 1)
            if i > 1 && j > 1 {
                array.push(input[i - 2][j - 2]);
            } else {
                array.push(*default);
            }

            //(i - 1, j)
            if i > 1 && j > 0 && j < input.len() + 1 {
                array.push(input[i - 2][j - 1]);
            } else {
                array.push(*default);
            }

            //(i - 1, j + 1)
            if i > 1 && j < input.len() {
                // println!("For (i - 1, j + 1): ({}, {})", i, j);
                array.push(input[i - 2][j]);
            } else {
                array.push(*default);
            }

            //(i, j - 1)
            if i > 0 && i < input.len() + 1 && j > 1 {
                // println!("For (i, j - 1): ({}, {})", i, j);
                array.push(input[i - 1][j - 2]);
            } else {
                array.push(*default);
            }

            //(i, j)
            if i > 0 && i < input.len() + 1 && j > 0 && j < input.len() + 1 {
                array.push(input[i - 1][j - 1]);
            } else {
                array.push(*default);
            }

            //(i, j + 1)
            if i > 0 && i < input.len() + 1 && j < input.len() {
                // println!("For (i, j + 1): ({}, {})", i, j);
                array.push(input[i - 1][j]);
            } else {
                array.push(*default);
            }

            //(i + 1, j - 1)
            if i < input.len() && j > 1 {
                array.push(input[i][j - 2]);
            } else {
                array.push(*default);
            }

            //(i + 1, j)
            if i < input.len() && j > 0 && j < input.len() + 1 {
                array.push(input[i][j - 1]);
            } else {
                array.push(*default);
            }

            //(i + 1, j + 1)
            if i < input.len() && j < input.len() {
                array.push(input[i][j]);
            } else {
                array.push(*default);
            }

            let value = convert(&array, &map);
            output_row.push(value);
        }

        output.push(output_row);
    }

    let current_default = *default;
    let default_vector = vec!(current_default; 9);

    *default = convert(&default_vector, &map);

    output
}

fn convert(input: &Vec<bool>, map: &HashMap<usize, bool>) -> bool {
    if input.len() != 9 {
        println!("{:?}", input);
        panic!();
    }

    let mut offset: usize = 0;
    for (i, value) in input.iter().enumerate() {
        if *value {
            offset += usize::pow(2, (8 - i).try_into().unwrap());
        }
    }

    // println!("{:?}, Offset: {}", input, offset);

    *map.get(&offset).unwrap()
}

fn get_image(filename: &str) -> Vec<Vec<bool>> {
    let mut image = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(row) = line {
                let image_row: Vec<bool> = row.chars().map(|c| if c == '#' { true } else { false }).collect();
                image.push(image_row);  
            }
        }
    }

    image
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn to_hash_map(input: &str) -> HashMap<usize, bool> {
    let mut output = HashMap::new();

    for (i, c) in input.chars().enumerate() {
        output.insert(i, if c == '#' { true } else { false });
    }

    output
}

fn to_string(input: &Vec<Vec<bool>>) -> String {
    let mut output = String::new();

    for row in input.iter() {
        for cell in row.iter() {
            if *cell {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        output.push('\n');
    }

    output
}

#[test]
fn test_to_hash_map() {
    let hash_map = to_hash_map(HEX_STRING);
    assert_eq!(hash_map.get(&34), Some(&true));
    assert_eq!(hash_map.get(&3), Some(&false));
}

#[test]
fn test_to_image() {
    let image = get_image(".\\input.txt");
    assert_eq!(image[1][1], false);
    assert_eq!(image[2][1], true);
}

#[test]
fn test_to_string() {
    let image = get_image(".\\input.txt");
    let output = to_string(&image);

    println!("{}", output);

    let first_line = &output[0..5];
    assert_eq!(first_line, "#..#.");
}

