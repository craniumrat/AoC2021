use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;

fn main() {
    let grid = make_grid();
    let graph = make_graph(&grid);

    let size = grid.len();

    let start_node = graph.node_indices().find(|n| graph[*n] == (0, 0)).unwrap();
    let end_node = graph.node_indices().find(|n| graph[*n] == (TryInto::<u32>::try_into(size).unwrap() - 1, TryInto::<u32>::try_into(size).unwrap() - 1)).unwrap();

    let solution = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());
    println!("{:?}", solution.get(&end_node));
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

                println!("{:?}", row);
                grid.push(row)
            }
        }
    }

    grid
}

fn make_graph(grid: &Vec<Vec<u32>>) -> UnGraph<(u32, u32), u32>
{
    let size = grid.len();
    let mut graph = UnGraph::<(u32, u32), u32>::with_capacity(size, size * size);

    let mut node_indices = vec!();

    for i in 0..size {
        let mut node_indices_row = vec!();
        for j in 0..size {
            let n = graph.add_node((i.try_into().unwrap(), j.try_into().unwrap()));
            node_indices_row.push(n);
        }

        node_indices.push(node_indices_row);
    }

    for i in 0..size {
        for j in 0..size {
            if i < size - 1 {
                let start = node_indices[i][j];
                let end = node_indices[i + 1][j];
                graph.update_edge(start, end, grid[i + 1][j]);
            }

            if j < size - 1 {
                let start = node_indices[i][j];
                let end = node_indices[i][j + 1];
                graph.update_edge(start, end, grid[i][j + 1]);
            }
        }
    }

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

    let first_node = graph.node_indices().find(|i| graph[*i] == (0, 1)).unwrap();
    for edge in graph.edges(first_node) {
        println!("{}", edge.weight());
    }

    assert_eq!(1, 1);
}
