use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;
use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph, Undirected};

fn main() {
    println!("Hello, world!");
}

pub fn part_one<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let (graph, start, end) = create_undirected_with_goals(parse_input(input_path)?);

    let x = petgraph::algo::astar::astar(
        &graph,
        start,
        |finish| finish == end,
        |edge| {
            *graph
                .node_weight(edge.target())
                .expect("node weight exists")
        },
        |_| 0,
    );
    match x {
        Some((found_weight, _)) => Ok(found_weight),
        None => todo!(),
    }
}

pub fn part_two<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let (graph, start, end) =
        create_undirected_with_goals(tile_vector(parse_input(input_path)?, 5, 5));

    let x = petgraph::algo::astar::astar(
        &graph,
        start,
        |finish| finish == end,
        |edge| {
            *graph
                .node_weight(edge.target())
                .expect("node weight exists")
        },
        |_| 0,
    );
    match x {
        Some((found_weight, _)) => Ok(found_weight),
        None => todo!(),
    }
}
pub fn create_undirected_with_goals(
    input_vec: Vec<Vec<usize>>,
) -> (
    Graph<usize, usize, Undirected, usize>,
    NodeIndex<usize>,
    NodeIndex<usize>,
) {
    let mut working_graph: Graph<usize, usize, Undirected, usize> =
        petgraph::Graph::with_capacity(88, 88);

    // Add all of the nodes to the graph so that I don't have to worry when adding the edges
    let node_indecies_vector: Vec<Vec<NodeIndex<usize>>> = input_vec
        .iter()
        .map(|x| x.iter().map(|y| working_graph.add_node(*y)).collect())
        .collect();

    for (row_index, row_vector) in node_indecies_vector.iter().enumerate() {
        for (col_index, current_node_index) in row_vector.iter().enumerate() {
            if col_index > 0 {
                working_graph.add_edge(
                    *current_node_index,
                    *row_vector
                        .get(col_index - 1)
                        .expect("Previous element exists"),
                    0,
                );
            }
            if row_index > 0 {
                working_graph.add_edge(
                    *current_node_index,
                    *node_indecies_vector
                        .get(row_index - 1)
                        .expect("Previous row exists")
                        .get(col_index)
                        .expect("Previous element exists"),
                    0,
                );
            }
        }
    }

    (
        working_graph,
        *node_indecies_vector
            .get(0)
            .expect("start index row exists")
            .get(0)
            .expect("start index exists"),
        *node_indecies_vector
            .last()
            .expect("last row exists")
            .last()
            .expect("goal index exists"),
    )
}

/// Collect the risk map located at input_path}
pub fn parse_input<P>(input_path: P) -> Result<Vec<Vec<usize>>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let result: Vec<Vec<usize>> = reader
        .lines()
        .flatten()
        .map(|x| x.trim().split("").flat_map(str::parse::<usize>).collect())
        .collect();

    Ok(result)
}

/// row_countr and col_countr will be total counts.  That means that 1, 1 is the identity
// This feels pretty terrible.
fn tile_vector(
    input_vector: Vec<Vec<usize>>,
    row_count: usize,
    col_count: usize,
) -> Vec<Vec<usize>> {
    // preallocate so that every operation is an insertion
    let mut return_vector: Vec<Vec<usize>> =
        Vec::with_capacity(input_vector.len() * col_count * row_count);

    for up_down_modifyer in 0..(row_count) {
        // For each row I want to add all of it's repotitiosn to the end
        for row in input_vector.iter() {
            let mut working_row = Vec::new();
            for left_right_modifyer in 0..col_count {
                working_row.append(
                    &mut row
                        .iter()
                        .map(|x| (((x + left_right_modifyer + up_down_modifyer) - 1) % 9) + 1)
                        .collect::<Vec<usize>>(),
                )
            }
            return_vector.push(working_row);
        }
    }

    return_vector
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two, tile_vector};

    #[test]
    fn test_parse_input() {
        let expected_result = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let actual = parse_input("src/test_input.txt");

        assert_eq!(actual.expect("Parse input return Ok"), expected_result);
    }

    #[test]
    fn test_part_one_example() {
        let expected = 40;

        let actual = part_one("src/test_input.txt");
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => panic!("{}", actual),
        }
    }

    #[test]
    fn test_part_one_my_input() {
        let expected = 487;

        let actual = part_one("src/input.txt");
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => panic!("{}", actual),
        }
    }

    #[test]
    fn test_tile_vector() {
        let expected = parse_input("src/tiled_test_input.txt").expect("parse_input test passes");
        let actual = tile_vector(
            parse_input("src/test_input.txt").expect("parse_input test passes"),
            5,
            5,
        );
        assert_eq!(actual, expected,"")
    }

    #[test]
    fn test_part_two_example() {
        let expected = 315;

        let actual = part_two("src/test_input.txt");
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => panic!("{}", actual),
        }
    }

    #[test]
    fn test_part_two_my_input() {
        let expected = 2821;

        let actual = part_two("src/input.txt");
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => panic!("{}", actual),
        }
    }
}
