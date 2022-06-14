use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_input(input_lines: Vec<String>) -> Result<(Vec<char>, HashMap<(char, char), char>)> {
    let mut input_lines_iter = input_lines.into_iter();

    let line_one = input_lines_iter.next().expect("This is me being lazy");
    let return_vec = line_one.chars().collect::<Vec<char>>();

    let _empty_next = input_lines_iter.next().expect("empty line");

    let mut return_hash_map: HashMap<(char, char), char> = HashMap::new();
    for other_line in input_lines_iter {
        let mut other_line_split = other_line.split(" -> ");
        if let (Some(rules), Some(result)) = (other_line_split.next(), other_line_split.next()) {
            let mut rules = rules.chars();
            if let (Some(a), Some(b)) = (rules.next(), rules.next()) {
                return_hash_map.insert(
                    (a, b),
                    result
                        .chars()
                        .next()
                        .ok_or_else(|| anyhow!("malformed rule"))?,
                );
            } else {
                return Err(anyhow!("Some rule was malformed"));
            }
        } else {
            return Err(anyhow!("Some rule line was malformed"));
        }
    }

    Ok((return_vec, return_hash_map))
}
pub fn parse_input_file<P>(input_path: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().flatten().collect::<Vec<String>>())
}

fn part_one(input_lines: Vec<String>) -> Result<usize> {
    let (mut working_string_vec, rule_map) = parse_input(input_lines)?;

    for _step in 0..10 {
        let mut next_vec =
            working_string_vec
                .as_slice()
                .windows(2)
                .fold(vec![], |mut acc, window| {
                    let a = window[0];
                    let b = window[1];
                    acc.push(a);
                    if let Some(c) = rule_map.get(&(a, b)) {
                        acc.push(*c);
                    };
                    acc
                });
        next_vec.push(
            working_string_vec
                .pop()
                .expect("working_string_vec not enpty"),
        );
        working_string_vec = next_vec;
    }
    //dbg!(&working_string_vec);

    let count_map: HashMap<char, usize> =
        working_string_vec
            .into_iter()
            .fold(HashMap::new(), |mut acc, b| {
                acc.entry(b).and_modify(|e| *e += 1).or_insert(1);
                acc
            });

    let most_common = count_map
        .values()
        .max()
        .ok_or_else(|| anyhow!("unable to find most common"))?;
    let least_common = count_map
        .values()
        .min()
        .ok_or_else(|| anyhow!("unable to find least common"))?;
    Ok(most_common - least_common)
}

fn part_two(input_lines: Vec<String>, step_count: usize) -> Result<usize> {
    // I am going to create two hashmaps.
    // The first is the number of each pair. This is so that I don't need to walk all the pairs and apply a rule for each one.  I can do each rule all at once.
    // The second is the current number of each char.   This is because we can't determine the number of each char by the pair counts alone
    let (initial_string_as_chars, rules) = parse_input(input_lines)?;
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();

    // Get the current counts
    let mut char_counts = HashMap::new();
    for curr_char in initial_string_as_chars.iter() {
        char_counts
            .entry(curr_char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for window in initial_string_as_chars.as_slice().windows(2) {
        pair_counts
            .entry((window[0], window[1]))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for _step in 0..step_count {
        let mut next_counts: HashMap<(char, char), usize> =
            HashMap::with_capacity(pair_counts.len());
        for (pattern, result) in rules.iter() {
            if let Some(matched_count) = pair_counts.get(pattern) {
                next_counts
                    .entry((pattern.0, *result))
                    .and_modify(|e| *e += matched_count)
                    .or_insert(*matched_count);

                char_counts
                    .entry(result)
                    .and_modify(|e| *e += matched_count)
                    .or_insert(*matched_count);

                next_counts
                    .entry((*result, pattern.1))
                    .and_modify(|e| *e += matched_count)
                    .or_insert(*matched_count);
            }
        }
        pair_counts = next_counts;
    }

    Ok(*char_counts
        .values()
        .max()
        .ok_or_else(|| anyhow!("Failed to find max frequency"))?
        - *char_counts
            .values()
            .min()
            .ok_or_else(|| anyhow!("Failed to find min frequency"))?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_input, parse_input_file, part_one, part_two};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let result = parse_input(input);
        assert!(result.is_ok());

        let (actual_string, actual_map) = result.expect("already asserted");
        let mut expected_map = HashMap::new();
        expected_map.insert(('C', 'H'), 'B');
        expected_map.insert(('H', 'H'), 'N');
        expected_map.insert(('C', 'B'), 'H');
        expected_map.insert(('N', 'H'), 'C');
        expected_map.insert(('H', 'B'), 'C');
        expected_map.insert(('H', 'C'), 'B');
        expected_map.insert(('H', 'N'), 'C');
        expected_map.insert(('N', 'N'), 'C');
        expected_map.insert(('B', 'H'), 'H');
        expected_map.insert(('N', 'C'), 'B');
        expected_map.insert(('N', 'B'), 'B');
        expected_map.insert(('B', 'N'), 'B');
        expected_map.insert(('B', 'B'), 'N');
        expected_map.insert(('B', 'C'), 'B');
        expected_map.insert(('C', 'C'), 'N');
        expected_map.insert(('C', 'N'), 'C');
        assert_eq!(actual_string, vec!['N', 'N', 'C', 'B']);
        assert_eq!(actual_map, expected_map);
    }

    #[test]
    fn test_part_one_example() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let result = part_one(input);
        assert!(result.is_ok());
        assert_eq!(result.expect("already aseretd"), 1588);
    }

    #[test]
    fn part_one_my_input() {
        let lines = parse_input_file("src/my_input.txt");
        assert!(lines.is_ok());
        let lines = lines.expect("already asserted");
        let result = part_one(lines);
        dbg!(&result);
        assert!(result.is_ok());
        let result = result.expect("already asserted");

        assert_eq!(result, 2170);
    }

    #[test]
    fn part_two_my_input() {
        let lines = parse_input_file("src/my_input.txt");
        assert!(lines.is_ok(), "Failed to read my input");
        let lines = lines.expect("already asserted");
        let result = part_two(lines, 40);
        dbg!(&result);
        assert!(result.is_ok());
        let result = result.expect("already asserted");

        assert_eq!(result, 2422444761283);
    }

    #[test]
    fn test_part_two_example() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let result = part_two(input, 40);
        assert!(result.is_ok());
        assert_eq!(result.expect("already aseretd"), 2188189693529);
    }

    #[test]
    fn part_two_steps() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let result = part_two(input.clone(), 1);
        assert!(result.is_ok(), "step1");
        assert_eq!(result.expect("already aseretd"), 1, "step1");

        // NBCCNBBBCBHCB
        let result = part_two(input.clone(), 2);
        assert!(result.is_ok(), "step2");
        assert_eq!(result.expect("already aseretd"), 5, "step2");

        //NBBBCNCCNBBNBNBBCHBHHBCHB
        let result = part_two(input.clone(), 3);
        assert!(result.is_ok(), "step3");
        assert_eq!(result.expect("already aseretd"), 7, "step3");

        //NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
        let result = part_two(input, 4);
        assert!(result.is_ok(), "step4");
        assert_eq!(result.expect("already aseretd"), 18, "step4");
    }
}
