use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Operation {
    FoldY(usize),
    FoldX(usize),
}

pub fn parse_input<P>(input_path: P) -> Result<(HashSet<(isize, isize)>, Vec<Operation>)>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().flatten();
    let mut location_set = HashSet::new();
    let mut operation_vector = Vec::new();

    // Process the coordinates
    for line in &mut lines_iter {
        if line.trim().is_empty() {
            break;
        }

        let coordinate: Vec<isize> = line
            .trim()
            .split(',')
            .map(|x| {
                x.parse::<isize>()
                    .expect("coordinate line must be in format isize,isize")
            })
            .collect();
        if coordinate.len() != 2 {
            return Err(anyhow!(
                "Coordinate should be in  format isize,isize.  Got {:?}",
                line
            ));
        }
        location_set.insert((coordinate[0], coordinate[1]));
    }

    // Process the operations

    for line in lines_iter {
        let line_split: Vec<&str> = line.split('=').collect();
        match line_split.len() {
            2 => {
                let value: usize = line_split.get(1).unwrap().parse()?;
                match line_split.get(0).unwrap().chars().last().unwrap() {
                    'y' => operation_vector.push(Operation::FoldY(value)),
                    'x' => operation_vector.push(Operation::FoldX(value)),
                    _ => {
                        return Err(anyhow!(
                            "Failed to determind fold x or y from line {:?}",
                            line
                        ))
                    }
                };
            }
            _ => {
                return Err(anyhow!("Failed to pull operation from line {:?}", line));
            }
        };
    }

    Ok((location_set, operation_vector))
}

fn fold(input_set: HashSet<(isize, isize)>, fold_operation: Operation) -> HashSet<(isize, isize)> {
    input_set
        .into_iter()
        .map(|(x, y)| match fold_operation {
            Operation::FoldX(fold_line) => {
                let fold_line = fold_line as isize;
                match x.cmp(&fold_line) {
                    Ordering::Greater => (fold_line - (x - fold_line), y),
                    Ordering::Less => (x, y),
                    Ordering::Equal => panic!("No dot should appear on a fold_line"),
                }
            }
            Operation::FoldY(fold_line) => {
                let fold_line = fold_line as isize;
                match y.cmp(&fold_line) {
                    Ordering::Greater => (x, fold_line - (y - fold_line)),
                    Ordering::Less => (x, y),
                    Ordering::Equal => panic!("No dot should appear on a fold_line"),
                }
            }
        })
        .collect()
}
pub fn part_one(input_set: HashSet<(isize, isize)>, operations: Vec<Operation>) -> usize {
    fold(
        input_set,
        operations
            .into_iter()
            .next()
            .expect("operations cannot be empty"),
    )
    .len()
}

pub fn part_two(mut input_set: HashSet<(isize, isize)>, operations: Vec<Operation>) -> String {
    for operation in operations {
        input_set = fold(input_set, operation);
    }

    let max_y = *input_set
        .iter()
        .map(|(_x, y)| y)
        .max()
        .or(Some(&0))
        .unwrap()
        + 1;
    let max_x = *input_set
        .iter()
        .map(|(x, _y)| x)
        .max()
        .or(Some(&0))
        .unwrap()
        + 1;

    // what I should probably do is shift all of them so that they are all positive, but that's another day
    assert!(max_y > 0);
    assert!(max_x > 0);
    let mut final_vec = vec![vec!["."; max_y as usize]; max_x as usize];
    dbg!((max_x, max_y));
    dbg!(&input_set);

    for (x, y) in input_set
        .into_iter()
        .map(|(x, y)| (x as usize, y as usize))
    {
        *final_vec.get_mut(x).unwrap().get_mut(y).unwrap() = "#";
    }

    final_vec
        .into_iter()
        .fold(String::new(), |final_string, line| {
            final_string + &line.into_iter().fold(String::new(), |acc, char| acc + char) + "\n"
        })
}
#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::{parse_input, part_one, part_two, Operation};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_input_example() {
        let example_parsed =
            parse_input("src/example_input.txt").expect("Expected src/example_input.txt to exist");
        let expected_coord = vec![
            (4, 11),
            (9, 0),
            (0, 13),
            (0, 3),
            (3, 0),
            (0, 14),
            (6, 0),
            (6, 10),
            (6, 12),
            (9, 10),
            (2, 14),
            (10, 4),
            (3, 4),
            (10, 12),
            (8, 4),
            (8, 10),
            (4, 1),
            (1, 10),
        ];
        let mut expected_set = HashSet::with_capacity(18);

        for ec in expected_coord {
            expected_set.insert(ec);
        }
        assert_eq!(example_parsed.0, expected_set);
        assert_eq!(
            example_parsed.1,
            vec![Operation::FoldY(7), Operation::FoldX(5)]
        );
    }

    #[test]
    fn test_part_one_example() {
        let example =
            parse_input("src/example_input.txt").expect("Expected src/example_input.txt to exist");

        let actual_result = part_one(example.0, example.1);
        assert_eq!(actual_result, 17);
    }

    #[test]
    fn test_part_one_my_input() {
        let my_input = parse_input("src/input.txt").expect("Expected src/input.txt to exist");

        let actual_result = part_one(my_input.0, my_input.1);
        assert_eq!(actual_result, 790);
    }

    #[test]
    fn part_two_example() {
        let example =
            parse_input("src/example_input.txt").expect("Expected src/example_input.txt to exist");

        let actual_result = part_two(example.0, example.1);
        assert_eq!(actual_result, "#####\n#...#\n#...#\n#...#\n#####\n");
    }

    #[test]
    fn part_two_my_input() {
        let my_input = parse_input("src/input.txt").expect("Expected src/input.txt to exist");

        let actual_result = part_two(my_input.0, my_input.1);
        // Even splitting the lines this is hard to read backwards
        assert_eq!(actual_result, "######\n#..#..\n#..#..\n.##...\n......\n.####.\n#....#\n#..#.#\n.#.###\n......\n######\n..#...\n..#...\n######\n......\n#...##\n#..#.#\n#.#..#\n##...#\n......\n######\n#.#..#\n#.#..#\n.#.##.\n......\n######\n#.#...\n#.#...\n#.....\n......\n....#.\n.....#\n#....#\n#####.\n......\n.####.\n#....#\n#....#\n.#..#.\n");
    }
}
