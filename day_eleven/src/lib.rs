use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

#[derive(Debug)]
enum Octopus {
    Flashed,
    Charging(usize),
}

pub fn parse_input<P>(input_path: P) -> Result<Vec<Vec<usize>>, Error>
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
pub fn part_two(input: Vec<Vec<usize>>) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut oc_vec = Vec::with_capacity(input.len());

    if input.get(0).unwrap().is_empty() {
        return 0;
    }

    for column in input {
        let mut oc_col = Vec::with_capacity(column.len());
        for charge in column {
            oc_col.push(Octopus::Charging(charge));
        }
        oc_vec.push(oc_col);
    }

    let mut local_flash_count = 0;
    let goal_flash_count = oc_vec.len() * oc_vec.get(0).unwrap().len();
    let mut current_step = 0;
    while local_flash_count != goal_flash_count {
        current_step += 1;

        let (flash_count, next_oc_vec) = part_one_step(oc_vec);
        local_flash_count = flash_count;
        oc_vec = next_oc_vec;
    }

    current_step
}
pub fn part_one(input: Vec<Vec<usize>>) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut oc_vec = Vec::with_capacity(input.len());

    if input.get(0).unwrap().is_empty() {
        return 0;
    }

    for column in input {
        let mut oc_col = Vec::with_capacity(column.len());
        for charge in column {
            oc_col.push(Octopus::Charging(charge));
        }
        oc_vec.push(oc_col);
    }
    let mut total_flash_count = 0;
    for _step in 0..100 {
        let (step_count, new_oc_vec) = part_one_step(oc_vec);

        oc_vec = new_oc_vec;
        total_flash_count += step_count;
    }

    total_flash_count
}

fn part_one_step(mut current_state: Vec<Vec<Octopus>>) -> (usize, Vec<Vec<Octopus>>) {
    let mut continue_flash: Vec<(usize, usize)> = Vec::new();

    let max_row_index = match current_state.len() {
        0 => 0,
        row_length => row_length - 1,
    };

    let max_col_index = match current_state.get(0) {
        Some(column_vector) => match column_vector.len() {
            0 => 0,
            column_length => column_length - 1,
        },
        None => 0,
    };

    let mut total_flashed = 0;

    // Scan the graph first to get the initial flashes
    for (row_index, columns) in current_state.iter_mut().enumerate() {
        for (col_index, octopus) in columns.iter_mut().enumerate() {
            match octopus {
                Octopus::Flashed => (),
                Octopus::Charging(charge_level) => {
                    if *charge_level >= 9 {
                        *octopus = Octopus::Flashed;
                        total_flashed += 1;
                        // Add neighbors to continue flash
                        let octopus_neighbors =
                            get_neighbors(row_index, col_index, max_row_index, max_col_index);

                        for neighbor in octopus_neighbors {
                            continue_flash.push(neighbor);
                        }
                    } else {
                        *charge_level += 1;
                    }
                }
            }
        }
    }

    while !continue_flash.is_empty() {
        let mut next_to_continue = Vec::new();

        for (row, col) in continue_flash {
            match current_state.get_mut(row).unwrap().get_mut(col).unwrap() {
                Octopus::Charging(level) => {
                    if *level >= 9 {
                        *current_state.get_mut(row).unwrap().get_mut(col).unwrap() =
                            Octopus::Flashed;

                        total_flashed += 1;

                        let neighbors = get_neighbors(row, col, max_row_index, max_col_index);
                        //.into_iter()
                        //.filter(|(row, col)| {
                        //match current_state.get(*row).unwrap().get(*col).unwrap() {
                        //Octopus::Flashed => false,
                        //Octopus::Charging(_) => true,
                        //}
                        //});

                        for neighbor in neighbors {
                            next_to_continue.push(neighbor);
                        }
                    } else {
                        *level += 1;
                    }
                }
                Octopus::Flashed => (),
            }
        }
        continue_flash = next_to_continue;
    }

    // Now clear the flashed
    for columns in current_state.iter_mut() {
        for octopus in columns.iter_mut() {
            match octopus {
                Octopus::Flashed => *octopus = Octopus::Charging(0),
                Octopus::Charging(_charge_level) => (),
            }
        }
    }

    (total_flashed, current_state)
}

fn get_neighbors(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let row = row as isize;
    let col = col as isize;
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .into_iter()
    .filter(|(row, col)| {
        *row >= 0 && *row as usize <= max_row && *col >= 0 && *col as usize <= max_col
    })
    .map(|(row, col)| (row as usize, col as usize))
    .collect()
}

#[cfg(test)]
mod tests {

    use crate::{parse_input, part_one, part_two};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn part_one_example() {
        let example_input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(part_one(example_input), 1656);
    }

    #[test]
    fn part_one_my_input() {
        let my_input = parse_input("src/input.txt").unwrap();
        assert_eq!(part_one(my_input), 1719);
    }

    #[test]
    fn test_part_one_small() {
        let example_input = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];

        assert_eq!(part_one(example_input), 259);
    }

    #[test]
    fn test_part_two_example(){
        let example_input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(part_two(example_input), 195);
    }

#[test]
    fn test_part_two_my_input(){
        let my_input = parse_input("src/input.txt").unwrap();
        assert_eq!(part_two(my_input), 232);
    }
}
