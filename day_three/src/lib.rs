use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum RatingSearch {
    Oxygen,
    CO2,
}

fn part_two_step(
    remaining_lines: Vec<Vec<bool>>,
    index: usize,
    search_type: RatingSearch,
) -> Vec<Vec<bool>> {
    let threshold = match search_type {
        RatingSearch::CO2 => f64::ceil((remaining_lines.len()) as f64 / 2.0) as usize,
        RatingSearch::Oxygen => f64::ceil(remaining_lines.len() as f64 / 2.0) as usize,
    };

    if remaining_lines.len() <= 1 {
        return remaining_lines;
    }

    let mut most_common =
        remaining_lines
            .iter()
            .fold(0, |count, line| if line[index] { count + 1 } else { count })
            >= threshold;

    if let RatingSearch::CO2 = search_type {
        most_common = !most_common;
    }

    remaining_lines
        .into_iter()
        .filter(|digit| {
            // Assuming every line has the same number of digits this should always be a valid index
            most_common == *digit.get(index).unwrap()
        })
        .collect()
}
pub fn parse_input<P>(input_path: P) -> Result<Vec<Vec<bool>>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut return_vector = Vec::new();
    for line in reader.lines().flatten() {
        return_vector.push(
            line.trim()
                .chars()
                .map(|x| match x {
                    '0' => false,
                    '1' => true,
                    _ => panic!("invalid input.  Expected 0 or 1 got {:?}", x),
                })
                .collect::<Vec<bool>>(),
        );
    }

    Ok(return_vector)
}

pub fn part_two<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let remaining_lines = parse_input(input_path)?;

    // This should be removed I'm just lazy atm
    let counts = vec![0; remaining_lines[0].len()];

    let mut remaining_o2 = remaining_lines.clone();
    for index in 0..counts.len() {
        remaining_o2 = part_two_step(remaining_o2, index, RatingSearch::Oxygen);
        if remaining_o2.len() <= 1 {
            break;
        }
    }

    let mut remaining_co2 = remaining_lines;
    for index in 0..counts.len() {
        remaining_co2 = part_two_step(remaining_co2, index, RatingSearch::CO2);
        if remaining_co2.len() <= 1 {
            break;
        }
    }
    if remaining_co2.len() > 1 || remaining_o2.len() > 1 {
        return Err(anyhow!("More than one number remaining for o2 or co2"));
    }
    if remaining_co2.is_empty() || remaining_o2.is_empty() {
        return Err(anyhow!("No numbers remaining for co2 or o2"));
    }

    Ok(bool_vec_to_usize(remaining_co2.get(0).unwrap())
        * bool_vec_to_usize(remaining_o2.get(0).unwrap()))
}

pub fn part_one<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let parsed_input = parse_input(input_path)?;

    let gamma_rate = part_one_gamma_rate(parsed_input)?;
    //let epsilon_rate = part_
    let epsilon_rate = part_one_epsilon_rate_from_gamma(&gamma_rate);

    Ok(bool_vec_to_usize(&epsilon_rate) * bool_vec_to_usize(&gamma_rate))
}

pub fn bool_vec_to_usize(input_vec: &[bool]) -> usize {
    input_vec
        .iter()
        .fold(0, |acc, x| if *x { (acc << 1) + 1 } else { acc << 1 })
}

fn true_counts(input: &[Vec<bool>]) -> Result<Vec<usize>> {
    if input.is_empty() {
        return Err(anyhow!("Input must not be empty"));
    }

    let mut counting_vec: Vec<usize> = vec![0; input[0].len()]; // Vec::with_capacity(input[0].len());

    for line in input {
        for (index, char) in line.iter().enumerate() {
            if *char {
                counting_vec[index] += 1;
            }
        }
    }
    Ok(counting_vec)
}

fn part_one_gamma_rate(input: Vec<Vec<bool>>) -> Result<Vec<bool>> {
    let max_threshold = f64::floor(input.len() as f64 / 2.0) as usize;
    let counting_vec = true_counts(&input)?;
    let mut return_vec: Vec<bool> = Vec::with_capacity(counting_vec.len());
    for digit in counting_vec {
        if digit > max_threshold {
            return_vec.push(true)
        } else {
            return_vec.push(false);
        }
    }
    Ok(return_vec)
}

fn part_one_epsilon_rate_from_input(input: Vec<Vec<bool>>) -> Result<Vec<bool>> {
    Ok(part_one_gamma_rate(input)?
        .into_iter()
        .map(|x| !x)
        .collect())
}

fn part_one_epsilon_rate_from_gamma(gamma_rate: &[bool]) -> Vec<bool> {
    gamma_rate.iter().fold(Vec::new(), |mut acc, digit| {
        acc.push(!digit);
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        bool_vec_to_usize, part_one, part_one_epsilon_rate_from_gamma, part_one_gamma_rate,
        part_two, part_two_step, RatingSearch,
    };

    #[test]
    fn test_epsilon_rate_from_gamma() {
        assert_eq!(part_one_epsilon_rate_from_gamma(&[]), vec![]);
        assert_eq!(part_one_epsilon_rate_from_gamma(&[true]), vec![false]);
        assert_eq!(
            part_one_epsilon_rate_from_gamma(&[true, true]),
            vec![false, false]
        );
        assert_eq!(
            part_one_epsilon_rate_from_gamma(&[false, false, false]),
            vec![true, true, true]
        );
        assert_eq!(
            part_one_epsilon_rate_from_gamma(&[true, false, true]),
            vec![false, true, false]
        );
    }

    #[test]
    fn test_part_one_gamma_rate() {
        let input = vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![false, false, false],
        ];

        assert_eq!(
            part_one_gamma_rate(input).unwrap(),
            vec![false, false, false]
        );
    }

    #[test]
    fn test_part_one_gamma_rate_again() {
        let input = vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![false, true, true],
        ];

        assert_eq!(part_one_gamma_rate(input).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn test_part_one_gamma_rate_again_again() {
        let input = vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![false, true, true],
            vec![true, false, false],
        ];

        assert_eq!(
            part_one_gamma_rate(input).unwrap(),
            vec![true, false, false]
        );
    }

    #[test]
    fn test_bool_vec_to_usize() {
        assert_eq!(bool_vec_to_usize(&[]), 0);
        assert_eq!(bool_vec_to_usize(&[true]), 1);
        assert_eq!(
            bool_vec_to_usize(&[true, false, false, true, true, false, true, false]),
            154
        );
        assert_eq!(
            bool_vec_to_usize(&[true, true, true, true, true, true, true, true]),
            255
        );
    }

    #[test]
    fn test_part_one_example() {
        let result = part_one("src/example_input.txt").unwrap();

        assert_eq!(result, 198);
    }

    #[test]
    fn test_part_one_my_input() {
        let result = part_one("src/input.txt").unwrap();

        assert_eq!(result, 741950);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two("src/example_input.txt").unwrap();
        assert_eq!(result, 230);
    }

    #[test]
    fn part_two_my_input() {
        let result = part_two("src/input.txt").unwrap();
        assert_eq!(result, 230);
    }

    /// if there is a tie in the count between 0 and 1's for co2 0 should be chosen.
    #[test]
    fn test_part_two_co2_rounding() {
        let input = vec![vec![false], vec![true]];
        let result = part_two_step(input, 0, RatingSearch::Oxygen);
        assert_eq!(result.len(), 1);
        assert!(*result.get(0).unwrap().get(0).unwrap());
    }

    /// if there is a tie in the count between 0 and 1's for o2 1 should be chosen.
    #[test]
    fn test_part_two_o2_rounding() {
        let input = vec![vec![false], vec![true]];
        let result = part_two_step(input, 0, RatingSearch::Oxygen);
        assert_eq!(result.len(), 1);
        assert!(*result.get(0).unwrap().get(0).unwrap());
    }
}
