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

fn make_graph(grid: &Vec<Vec<u32>>) -> DiGraph<u32, u32>
{
    let size = grid.len();

    //Need to create a tuple of (source, target, weight) for all
    //nodes. We call source & target as (i*size + j) u32's and weight as grid[i][j] of target
    let mut edge_weights = vec!();   
 
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

    let size_u32: u32 = grid.len().try_into().unwrap();

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
