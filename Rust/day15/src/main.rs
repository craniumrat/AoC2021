use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use petgraph::graph::DiGraph;
use petgraph::algo::dijkstra;

fn main() {
    let grid = make_grid();
    let graph = make_graph(&grid);

    let size = grid.len();

    let start_node = graph.node_indices().find(|n| graph[*n] == 0).unwrap();
    let end_node = graph.node_indices().find(|n| n == &petgraph::prelude::NodeIndex::new(size * size - 1)).unwrap();

    let solution = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());
    println!("Part 1: {:?}", solution.get(&end_node));

    let large_grid = make_large_grid(&grid);
    let large_graph = make_graph(&large_grid);
    let large_size = large_grid.len();

    let large_start_node = large_graph.node_indices().find(|n| graph[*n] == 0).unwrap();
    let large_end_node = large_graph.node_indices().find(|n| n == &petgraph::prelude::NodeIndex::new(large_size * large_size - 1)).unwrap();

    let large_solution = dijkstra(&large_graph, large_start_node, Some(large_end_node), |e| *e.weight());
    print!("Part 2: {:?}", large_solution.get(&large_end_node));
}

fn make_grid() -> Vec<Vec<u32>>
{
    let mut grid = vec!();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(string) = line {
                let mut row = vec!();
                for c in string.chars() {
                    row.push(c.to_digit(10).unwrap());
                }

                // println!("{:?}", row);
                grid.push(row)
            }
        }
    }

    grid
}

fn make_large_grid(base: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let base_size = base.len();
    let mut output = Vec::with_capacity(base_size * 5 * 5);

    //First row. i = 0..5, j = 0, grid[i] == 0..grid.len(),  

    for i in 0..5 {        
        for row in 0..base.len() {
            let mut output_row = vec!();
            for j in 0..5 {
                // println!("({}, {}): {}", i, j, i + j);
                let risk = i + j;
    
                for col in 0..base.len() {
                    let mut value: u32 = base[row][col] + risk as u32;
                    while value > 9 {
                        // println!("Value: Before: {}, After: {}", value, value - 9);
                        value -= 9
                    }
                    output_row.push(value);
                }
            }

            output.push(output_row);
        }
    }

    output
}

/** NOTE: Turns out, changing the node weights from (u32, u32) to u32 was not necessary. I should have made a 
 * DiGraph with the incoming weights on both directions to get the correct result. Lesson learnt. 
 */
fn make_graph(grid: &Vec<Vec<u32>>) -> DiGraph<u32, u32>
{
    let size = grid.len();

    //Need to create a tuple of (source, target, weight) for all
    //nodes. We call source & target as (i*size + j) u32's and weight as grid[i][j] of target
    let mut edge_weights = vec!();

    println!("Grid size: {}", size);
 
    for i in 0..size {
        for j in 0..size {
            if i < size - 1 {
                edge_weights.push((TryInto::<u32>::try_into(i * size + j).unwrap(), TryInto::<u32>::try_into((i + 1) * size + j).unwrap(), TryInto::<u32>::try_into(grid[i + 1][j]).unwrap()));
                edge_weights.push((TryInto::<u32>::try_into((i + 1) * size + j).unwrap(), TryInto::<u32>::try_into(i * size + j).unwrap(), TryInto::<u32>::try_into(grid[i][j]).unwrap()));
            }

            if j < size - 1 {
                edge_weights.push((TryInto::<u32>::try_into(i * size + j).unwrap(), TryInto::<u32>::try_into(i * size + j + 1).unwrap(), TryInto::<u32>::try_into(grid[i][j + 1]).unwrap()));
                edge_weights.push((TryInto::<u32>::try_into(i * size + j + 1).unwrap(), TryInto::<u32>::try_into(i * size + j).unwrap(), TryInto::<u32>::try_into(grid[i][j]).unwrap()));
            }
        }
    }

    // println!("{:?}", edge_weights);

    let graph = DiGraph::from_edges(&edge_weights);
    graph
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_read_grid() {
    let grid = make_grid();

    assert_eq!(grid[0][0], 1);
}

#[test]
fn test_make_graph() {
    let grid = make_grid();
    let graph = make_graph(&grid);

    assert_eq!(graph.node_count(), grid.len() * grid.len());

    // for v in graph.node_indices() {
    //     println!("{:?}", v);
    // }

    // for v in graph.node_indices() {
    //     for e in graph.edges(v) {
    //         println!("{:?}", e);
    //     }
    // }

    assert_eq!(1, 0);
}

#[test]
fn test_make_large_grid() {
    let base = make_grid();
    let grid = make_large_grid(&base);

    println!("{:?}", grid);

    for i in 0..grid.len() {
        let line: String = grid[i].iter().map(|d| char::from_digit(*d, 10).unwrap()).collect();
        println!("{}", line);
    }

    
    assert_eq!(0, 1);
}
