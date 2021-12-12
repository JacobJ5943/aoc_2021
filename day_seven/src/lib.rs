use anyhow::{anyhow, Result};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part_one<P>(input_path: P) -> Result<(isize, isize)>
where
    P: AsRef<Path>,
{
    let input_parsed = parse_input(input_path)?;

    let (min, max) = match (input_parsed.keys().min(), input_parsed.keys().max()) {
        (Some(min), Some(max)) => (*min, *max),
        (None, None) => return Err(anyhow!("Failed to find a min and max for the given input")),
        (_, _) => {
            panic!("If there was a min there should also be a max.  So this case should not exist")
        }
    };

    let mut costs = (min..=max).map(|x| (calculate_cost_part_one(&input_parsed, &x), x));

    if let Some((mut current_min_cost, mut current_min_location)) = costs.next() {
        for (next_min_cost, next_min_location) in costs {
            if next_min_cost < current_min_cost {
                current_min_cost = next_min_cost;
                current_min_location = next_min_location;
            }
        }
        Ok((current_min_cost, current_min_location))
    } else {
        Err(anyhow!("Failed to find a min cost"))
    }
}

pub fn part_two<P>(input_path: P) -> Result<(isize, isize)>
where
    P: AsRef<Path>,
{
    let input_parsed = parse_input(input_path)?;

    let (min, max) = match (input_parsed.keys().min(), input_parsed.keys().max()) {
        (Some(min), Some(max)) => (*min, *max),
        (None, None) => return Err(anyhow!("Failed to find a min and max for the given input")),
        (_, _) => {
            panic!("If there was a min there should also be a max.  So this case should not exist")
        }
    };

    let mut costs = (min..=max).map(|x| (calculate_cost_part_two(&input_parsed, &x), x));

    if let Some((mut current_min_cost, mut current_min_location)) = costs.next() {
        for (next_min_cost, next_min_location) in costs {
            if next_min_cost < current_min_cost {
                current_min_cost = next_min_cost;
                current_min_location = next_min_location;
            }
        }
        Ok((current_min_cost, current_min_location))
    } else {
        Err(anyhow!("Failed to find a min cost"))
    }
}

fn calculate_cost_part_one(
    crab_locations: &HashMap<isize, isize>,
    target_location: &isize,
) -> isize {
    crab_locations
        .iter()
        .fold(0, |acc, (crab_location, cost_per_index)| {
            let distance = (crab_location - target_location).abs();
            acc + distance * cost_per_index
        })
}

fn calculate_cost_part_two(
    crab_locations: &HashMap<isize, isize>,
    target_location: &isize,
) -> isize {
    crab_locations
        .iter()
        .fold(0, |acc, (crab_location, crabs_at_location)| {
            let distance = (crab_location - target_location).abs();

            let cost_per_crab = distance * (distance + 1) / 2; // (N * (N + 1) )/ 2

            acc + cost_per_crab * crabs_at_location
        })
}

pub fn parse_input<P>(input_path: P) -> Result<HashMap<isize, isize>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    match reader.lines().flatten().next() {
        Some(line) => {
            let crab_locations: Vec<isize> = line
                .trim()
                .split(',')
                .map(|crab_location| {
                    crab_location
                        .parse::<isize>()
                        .expect("Only integer locations are allowed")
                })
                .collect();
            let mut crab_costs_by_location = HashMap::new();
            for crab_location in crab_locations {
                match crab_costs_by_location.entry(crab_location) {
                    Entry::Occupied(occupied) => *occupied.into_mut() += 1,
                    Entry::Vacant(vacant) => {
                        vacant.insert(1);
                    }
                };
            }

            Ok(crab_costs_by_location)
        }
        None => panic!("Input was empty"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_cost_part_one, parse_input, part_one, part_two};
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_parse_input() {
        let example_parsed = parse_input("src/example_input.txt").unwrap();

        let mut expected_parsed = HashMap::new();
        expected_parsed.insert(16, 1);
        expected_parsed.insert(0, 1);
        expected_parsed.insert(1, 2);
        expected_parsed.insert(2, 3);
        expected_parsed.insert(4, 1);
        expected_parsed.insert(7, 1);
        expected_parsed.insert(14, 1);

        assert_eq!(example_parsed, expected_parsed);
    }

    #[test]
    fn test_calculate_costs_target_before_crab() {
        let mut ex_map = HashMap::new();

        ex_map.insert(5, 2);

        assert_eq!(calculate_cost_part_one(&ex_map, &0), 10);
        assert_eq!(calculate_cost_part_one(&ex_map, &1), 8);
        assert_eq!(calculate_cost_part_one(&ex_map, &2), 6);
        assert_eq!(calculate_cost_part_one(&ex_map, &3), 4);
        assert_eq!(calculate_cost_part_one(&ex_map, &4), 2);
        assert_eq!(calculate_cost_part_one(&ex_map, &5), 0);
    }

    #[test]
    fn test_calculate_costs_target_after_crab() {
        let mut ex_map = HashMap::new();
        ex_map.insert(5, 2);
        assert_eq!(calculate_cost_part_one(&ex_map, &6), 2);
        assert_eq!(calculate_cost_part_one(&ex_map, &7), 4);
        assert_eq!(calculate_cost_part_one(&ex_map, &8), 6);
        assert_eq!(calculate_cost_part_one(&ex_map, &9), 8);
        assert_eq!(calculate_cost_part_one(&ex_map, &10), 10);
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one("src/example_input.txt").unwrap(), (37, 2))
    }

    #[test]
    fn test_part_one_my_input() {
        assert_eq!(part_one("src/input.txt").unwrap(), (337833, 331))
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two("src/example_input.txt").unwrap(), (168, 5))
    }

    #[test]
    fn test_part_two_my_input() {
        assert_eq!(part_two("src/input.txt").unwrap(), (96678050, 461))
    }
}
