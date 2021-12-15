use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part_one(heightmap: &[Vec<usize>]) -> usize {
    let low_points = find_low_points(heightmap);

    low_points.into_iter().fold(0, |acc, (row, col)| {
        acc + heightmap.get(row).unwrap().get(col).unwrap() + 1
    })
}

type Basin = HashMap<(usize, usize), ()>;
pub fn part_two(heightmap: &[Vec<usize>]) -> usize {
    let low_points = find_low_points(heightmap);
    if low_points.is_empty() {
        return 0;
    }

    let mut basins: Vec<Basin> = Vec::new();

    // create basins for each low_point
    for low_point in &low_points {
        let mut low_point_basin = HashMap::new();
        low_point_basin.insert((low_point.0, low_point.1), ());
        basins.push(low_point_basin);
    }

    let mut new_nodes: Vec<(usize, (usize, usize))> = low_points.into_iter().enumerate().collect();

    while !new_nodes.is_empty() {
        let mut next_nodes: Vec<(usize, (usize, usize))> = Vec::with_capacity(new_nodes.len());

        for (basin, (row, col)) in new_nodes {
            let neighbors = get_neighbors(row, col, heightmap);
            for neighbor in neighbors {
                if heightmap.get(neighbor.0).unwrap().get(neighbor.1).unwrap() != &9
                    && !basins[basin].contains_key(&neighbor)
                {
                    basins
                        .get_mut(basin)
                        .unwrap()
                        .insert((neighbor.0, neighbor.1), ());
                    next_nodes.push((basin, (neighbor.0, neighbor.1)))
                }
            }
        }

        new_nodes = next_nodes;
    }

    let mut max_three: Vec<usize> = basins.iter().take(3).map(|x| x.keys().len()).collect();
    let mut max_three_min: usize = *max_three.iter().min().unwrap();
    if max_three.len() == 3 {
        for basin in basins.iter().skip(3) {
            if basin.keys().len() > max_three_min {
                max_three.rotate_left(1);
                max_three.pop();
                max_three.push(basin.keys().len());
                max_three.sort_unstable();
                max_three_min = *max_three.iter().min().unwrap();
            }
        }
    }

    dbg!(max_three).into_iter().product()
}

fn get_neighbors(
    row_index: usize,
    column_index: usize,
    heightmap: &[Vec<usize>],
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);

    let max_row_index = match heightmap.len() {
        0 => 0,
        row_length => row_length - 1,
    };

    let max_col_index = match heightmap.get(0) {
        Some(column_vector) => match column_vector.len() {
            0 => 0,
            column_length => column_length - 1,
        },
        None => 0,
    };

    if row_index > 0 {
        neighbors.push((row_index - 1, column_index));
    }

    // Get the one to the left of the current node
    if column_index > 0 {
        neighbors.push((row_index, column_index - 1));
    }

    // Get the node below
    if row_index < max_row_index {
        neighbors.push((row_index + 1, column_index));
    }

    // get below to the right
    if column_index < max_col_index {
        neighbors.push((row_index, column_index + 1));
    }

    neighbors
}
pub fn parse_input<P>(input_path: P) -> Result<Vec<Vec<usize>>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .flatten()
        .map(|line| {
            line.trim()
                .chars()
                .map(|x| match x {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    other => panic!("Only integers are accepted.  Got char {:?}", other),
                })
                .collect()
        })
        .collect())
}

fn find_low_points(heightmap: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();

    for (row_index, row) in heightmap.iter().enumerate() {
        for (column_index, height) in row.iter().enumerate() {
            let neighbors: Vec<usize> = get_neighbors(row_index, column_index, heightmap)
                .into_iter()
                .map(|(row, col)| *heightmap.get(row).unwrap().get(col).unwrap())
                .collect();

            let mut low_point = true;
            for neighbor in neighbors {
                if height >= &neighbor {
                    low_point = false;
                    break;
                }
            }

            if low_point {
                low_points.push((row_index, column_index));
            }
        }
    }

    low_points
}

#[cfg(test)]
mod tests {
    use crate::{find_low_points, parse_input, part_one, part_two};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_low_points_example() {
        let example_input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let actual_result = find_low_points(&example_input);
        let expected_low_points = vec![(0, 1), (0, 9), (2, 2), (4, 6)];
        assert_eq!(
            actual_result.len(),
            expected_low_points.len(),
            "Expected length to be {}, got {}",
            expected_low_points.len(),
            actual_result.len()
        );
        for low_point in actual_result {
            assert!(
                expected_low_points.contains(&low_point),
                "Unexpected low point {:?}",
                low_point
            );
        }
    }

    #[test]
    fn test_part_one_example() {
        let example_input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let actual_result = part_one(&example_input);
        let expected_result = 15;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_part_one_my_input() {
        let my_input = parse_input("src/input.txt").unwrap();
        let actual_result = part_one(&my_input);
        let expected_result = 475;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_part_two_example() {
        let example_input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let actual_result = part_two(&example_input);
        let expected_result = 1134;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_part_two_my_input() {
        let my_input = parse_input("src/input.txt").unwrap();
        let actual_result = part_two(&my_input);
        let expected_result = 475;

        assert_eq!(actual_result, expected_result);
    }
}
