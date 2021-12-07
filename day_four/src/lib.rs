use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// It's late so I'm going to assume that any boards passed in are valid.
// By valid I mean that all rows have the same number of columns
pub fn parse_input<P>(input_path: P) -> Result<(Vec<usize>, Vec<Board>)>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();
    let called_numbers = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    lines.next(); // There is a blank space between board

    let mut created_boards = Vec::new();
    let mut current_board = Vec::new();
    while let Some(next) = lines.next() {
        if next.trim().is_empty() {
            created_boards.push(Board::new(current_board));
            current_board = Vec::new();
        } else {
            current_board.push(
                next.trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        }
    }
    if current_board.len() > 0 {
        created_boards.push(Board::new(current_board));
    }

    Ok((called_numbers, created_boards))
}

pub fn part_one<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let (called_numbers, mut boards) = parse_input(input_path)?;
    for number in called_numbers {
        for board in &mut boards {
            if board.call_number(number).is_some() {
                return Ok(board.non_marked().into_iter().fold(0, |acc, x| acc + x) * number);
            }
        }
    }

    Err(anyhow!("No board had bingo by the end"))
}

pub fn part_two<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let (called_numbers, mut boards) = parse_input(input_path)?;
    let mut winning_scores = Vec::with_capacity(boards.len());
    let mut new_winning = Vec::new();
    for number in called_numbers {
        for (board_num, board) in &mut boards.iter_mut().enumerate() {
            if board.call_number(number).is_some() {
                winning_scores
                    .push(board.non_marked().into_iter().fold(0, |acc, x| acc + x) * number);
                new_winning.push(board_num);
            }
        }
        for index in new_winning.into_iter().rev() {
            boards.remove(index);
        }
        new_winning = Vec::new();
    }

    if let Some(score) = winning_scores.last() {
        Ok(*score)
    } else {
        Err(anyhow!("No board had bingo by the end"))
    }
}
#[derive(Debug)]
pub struct Board {
    internal_hits: Vec<Vec<bool>>,
    values: Vec<Vec<usize>>,
    values_index: HashMap<usize, (usize, usize)>,
}

impl Board {
    pub fn new(chosen_numbers: Vec<Vec<usize>>) -> Self {
        let mut values_index =
            HashMap::with_capacity(chosen_numbers.len() * chosen_numbers.get(0).unwrap().len());
        for (row_index, row) in chosen_numbers.iter().enumerate() {
            for (col_index, number) in row.iter().enumerate() {
                values_index.insert(*number, (row_index, col_index));
            }
        }

        Board {
            internal_hits: vec![vec![false; chosen_numbers[0].len()]; chosen_numbers.len()],
            values: chosen_numbers,
            values_index,
        }
    }
    pub fn is_bingo(&self) -> Option<Vec<usize>> {
        let mut result = self.row_check();
        if result.is_some() {
            return result;
        }

        result = self.column_check();
        if result.is_some() {
            return result;
        }
        None
    }

    fn row_check(&self) -> Option<Vec<usize>> {
        self.internal_hits
            .iter()
            .enumerate()
            .fold(None, |acc, (row_num, row_hits)| {
                if acc.is_none() {
                    if row_hits.iter().all(|x| *x) {
                        Some(self.values.get(row_num).unwrap().clone())
                    } else {
                        None
                    }
                } else {
                    acc
                }
            })
    }

    fn column_check(&self) -> Option<Vec<usize>> {
        let num_columns = self.internal_hits.get(0).unwrap().len();

        for column_num in 0..num_columns {
            let column_result = self.internal_hits.iter().fold(true, |acc, row| {
                if acc {
                    // If this number has not been called
                    if !row.get(column_num).unwrap() {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            });

            if column_result {
                return Some(self.values.iter().fold(Vec::new(), |mut acc, row| {
                    acc.push(*row.get(column_num).unwrap());
                    acc
                }));
            }
        }
        None
    }

    fn diag_check(&self) -> Option<Vec<usize>> {
        if self.internal_hits.len() != self.internal_hits.get(0).unwrap().len() {
            return None;
        }

        let mut found_bingo = true;
        let mut bingo_result: Vec<usize> = Vec::with_capacity(self.internal_hits.len());

        for (index, row) in self.internal_hits.iter().enumerate() {
            if !row.get(index).unwrap() {
                found_bingo = false;
                break;
            } else {
                bingo_result.push(*self.values.get(index).unwrap().get(index).unwrap());
            }
        }

        if found_bingo {
            return Some(bingo_result);
        }

        bingo_result.clear();
        found_bingo = true;

        for (index, row) in self.internal_hits.iter().rev().enumerate() {
            if !row.get(index).unwrap() {
                found_bingo = false;
                break;
            } else {
                bingo_result.push(
                    *self
                        .values
                        .get(self.internal_hits.len() - index - 1)
                        .unwrap()
                        .get(index)
                        .unwrap(),
                );
            }
        }

        if found_bingo {
            return Some(bingo_result);
        } else {
            None
        }
    }

    pub fn call_number(&mut self, called_number: usize) -> Option<Vec<usize>> {
        let found_number = self.values_index.get(&called_number);
        match found_number {
            Some((row_num, col_num)) => {
                self.internal_hits[*row_num][*col_num] = true;
                self.is_bingo()
            }
            None => None,
        }
    }

    pub fn non_marked(&self) -> Vec<usize> {
        let mut return_vec = Vec::new();
        for (row_num, row) in self.internal_hits.iter().enumerate() {
            for (col_num, marked) in row.iter().enumerate() {
                if !*marked {
                    return_vec.push(*self.values.get(row_num).unwrap().get(col_num).unwrap());
                }
            }
        }
        return_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, Board};
    fn create_5_by_5() -> Board {
        let board_values = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];

        Board::new(board_values)
    }
    //    #[test] Disabled as diag isn't in part one
    fn test_diag() {
        let mut board = create_5_by_5();
        assert_eq!(board.call_number(1), None);
        assert_eq!(board.call_number(7), None);
        assert_eq!(board.call_number(13), None);
        assert_eq!(board.call_number(19), None);
        assert_eq!(board.call_number(25).unwrap(), vec![1, 7, 13, 19, 25]);

        board = create_5_by_5();
        assert_eq!(board.call_number(5), None);
        assert_eq!(board.call_number(9), None);
        assert_eq!(board.call_number(13), None);
        assert_eq!(board.call_number(17), None);
        let result = board.call_number(21);
        assert_eq!(result.unwrap(), vec![21, 17, 13, 9, 5]);
    }

    #[test]
    fn test_rows() {
        let mut board = create_5_by_5();
        assert_eq!(board.call_number(1), None);
        assert_eq!(board.call_number(2), None);
        assert_eq!(board.call_number(3), None);
        assert_eq!(board.call_number(4), None);
        assert_eq!(board.call_number(5).unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_columns() {
        let mut board = create_5_by_5();
        assert_eq!(board.call_number(1), None);
        assert_eq!(board.call_number(6), None);
        assert_eq!(board.call_number(11), None);
        assert_eq!(board.call_number(16), None);
        assert_eq!(board.call_number(21).unwrap(), vec![1, 6, 11, 16, 21]);
    }

    #[test]
    fn test_get_un_marked() {
        let mut board = create_5_by_5();
        let mut expected = Vec::with_capacity(25);
        for num in 1..26 {
            expected.push(num);
        }

        assert_eq!(board.non_marked(), expected);

        expected.clear();
        for num in 7..25 {
            expected.push(num);
        }
        board.call_number(1);
        board.call_number(2);
        board.call_number(3);
        board.call_number(4);
        board.call_number(5);
        board.call_number(6);
        board.call_number(25);
        assert_eq!(board.non_marked(), expected);
    }

    #[test]
    fn part_one_example() {
        assert_eq!(part_one("src/example_input.txt").unwrap(), 4512);
    }
    #[test]
    fn part_one_my_input() {
        assert_eq!(part_one("src/input.txt").unwrap(), 71708);
    }

    #[test]
    fn part_two_my_input() {
        assert_eq!(part_two("src/input.txt").unwrap(), 34726);
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two("src/example_input.txt").unwrap(), 1924);
    }
}
