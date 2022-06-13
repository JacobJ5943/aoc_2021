use anyhow::{anyhow, Result};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

fn part_one(input_string: Vec<String>) -> Result<usize> {
    let initial_hash_set = RefCell::new(HashSet::new());
    initial_hash_set.borrow_mut().insert("start".to_string());
    depth_first_search_all_paths_part_one(
        &create_cave_from_input_lines(input_string)?,
        &initial_hash_set,
        &"start".to_string(),
    )
}

fn part_two(input_string: Vec<String>) -> Result<usize> {
    let initial_hash_set = RefCell::new(HashSet::new());
    initial_hash_set.borrow_mut().insert("start".to_string());
    depth_first_search_all_paths_part_two(
        &create_cave_from_input_lines(input_string)?,
        &initial_hash_set,
        &"start".to_string(),
        false,
    )
}

fn create_cave_from_input_lines(
    input_lines: Vec<String>,
) -> Result<HashMap<String, HashSet<String>>> {
    let mut cave: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input_lines {
        let mut line_split = line.split('-').take(2);
        match (line_split.next(), line_split.next()) {
            (Some(key), Some(value)) => {
                match cave.entry(key.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_set) => {
                        occupied_set.get_mut().insert(value.to_string());
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_location) => {
                        let mut insert_hashset = HashSet::new();
                        insert_hashset.insert(value.to_string());
                        vacant_location.insert(insert_hashset);
                    }
                }

                match cave.entry(value.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_set) => {
                        occupied_set.get_mut().insert(key.to_string());
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_location) => {
                        let mut insert_hashset = HashSet::new();
                        insert_hashset.insert(key.to_string());
                        vacant_location.insert(insert_hashset);
                    }
                }
            }
            _ => return Err(anyhow!("Failed to get left and right of entry")),
        };
    }
    Ok(cave)
}

// This is assuming that there are no ways that there can be a loop.
// That is A -> B -> C -> A should never exist
// If that's the case this will recure until a limit is hit
fn depth_first_search_all_paths_part_two(
    cave: &HashMap<String, HashSet<String>>,
    already_visited_nodes: &RefCell<HashSet<String>>,
    current_location: &String,
    already_second: bool,
) -> Result<usize> {
    if current_location.eq("end") {
        return Ok(1);
    }

    let mut current_count = 0;
    //for key in cave.keys() {
    if let Some(possible_next_paths) = cave.get(current_location) {
        for possible_next_location in possible_next_paths.iter() {
            let next_location_is_lower_case =
                possible_next_location.chars().all(char::is_lowercase);

            // If it's a big cave go for it
            if !next_location_is_lower_case {
                current_count += depth_first_search_all_paths_part_two(
                    cave,
                    already_visited_nodes,
                    possible_next_location,
                    already_second,
                )?;
            } else {
                // This is a small cave now
                let already_traversed = already_visited_nodes
                    .borrow_mut()
                    .contains(possible_next_location);

                if !already_traversed {
                    already_visited_nodes
                        .borrow_mut()
                        .insert(possible_next_location.clone());

                    current_count += depth_first_search_all_paths_part_two(
                        cave,
                        already_visited_nodes,
                        possible_next_location,
                        already_second,
                    )?;

                    already_visited_nodes
                        .borrow_mut()
                        .remove(possible_next_location);
                } else if !already_second && possible_next_location != "start" {
                    // annoyed I have this start bandaid in here
                    current_count += depth_first_search_all_paths_part_two(
                        cave,
                        already_visited_nodes,
                        possible_next_location,
                        true,
                    )?;
                }
            }

            // I don't know how to make this not have the duplicate code.
            // I want if let ... = ... && (second_small_name == &next_location), but that's not a thing
            // Normal case
            if next_location_is_lower_case {}

            // end normal case
        }
    }
    //}

    Ok(current_count)
}

// This is assuming that there are no ways that there can be a loop.
// That is A -> B -> C -> A should never exist
// If that's the case this will recure until a limit is hit
fn depth_first_search_all_paths_part_one(
    cave: &HashMap<String, HashSet<String>>,
    already_visited_nodes: &RefCell<HashSet<String>>,
    current_location: &String,
) -> Result<usize> {
    if current_location.eq("end") {
        return Ok(1);
    }

    let mut current_count = 0;
    //for key in cave.keys() {
    if let Some(possible_next_paths) = cave.get(current_location) {
        for next_location in possible_next_paths
            .iter()
            .filter(|a| !already_visited_nodes.borrow_mut().contains(*a))
        {
            if next_location.chars().all(char::is_lowercase) {
                // This means that we need ot add it to already_visited_nodes
                already_visited_nodes
                    .borrow_mut()
                    .insert(next_location.clone());
            }
            current_count +=
                depth_first_search_all_paths_part_one(cave, already_visited_nodes, next_location)?;

            if next_location.chars().all(char::is_lowercase) {
                already_visited_nodes.borrow_mut().remove(next_location);
            }
        }
    }
    //}

    Ok(current_count)
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{part_one, part_two};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_create_cave() {
        let actual_cave = crate::create_cave_from_input_lines(vec![
            "A-B".to_string(),
            "A-C".to_string(),
            "B-C".to_string(),
        ]);

        assert!(actual_cave.is_ok());
        let actual_cave = actual_cave.unwrap();

        assert!(actual_cave.contains_key("A"));
        assert!(actual_cave.contains_key("B"));
        assert!(actual_cave.contains_key("C"));

        let mut expected_a = HashSet::new();
        expected_a.insert("B".to_string());
        expected_a.insert("C".to_string());
        assert_eq!(
            &expected_a,
            actual_cave.get("A").expect("already asserted A exists")
        )
    }

    #[test]
    fn test_part_one() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end"
            .split("\n")
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let result = part_one(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 10);

        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
                .to_string()
                .split("\n")
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
        let result = part_one(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 19);

        let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW".to_string().split("\n").map(|a|a.to_string()).collect::<Vec<String>>();
        let result = part_one(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 226);
    }

    #[test]
    fn test_part_one_my_input() {
        let input = "CV-mk\ngm-IK\nsk-gm\nca-sk\nsx-mk\ngm-start\nsx-ca\nkt-sk\nca-VS\nkt-ml\nkt-ca\nmk-IK\nend-sx\nend-sk\ngy-sx\nend-ca\nca-ml\ngm-CV\nsx-kt\nstart-CV\nIK-start\nCV-kt\nml-mk\nml-CV\nml-gm\nml-IK".to_string().split("\n").map(|a|a.to_string()).collect::<Vec<String>>();

        let result = part_one(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 4186);
    }

    #[test]
    fn test_part_two() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end"
            .split("\n")
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let result = part_two(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 36);

        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
                .to_string()
                .split("\n")
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
        let result = part_two(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 103);

        let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW".to_string().split("\n").map(|a|a.to_string()).collect::<Vec<String>>();
        let result = part_two(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 3509);
    }

    #[test]
    fn part_two_my_input() {
        let input = "CV-mk\ngm-IK\nsk-gm\nca-sk\nsx-mk\ngm-start\nsx-ca\nkt-sk\nca-VS\nkt-ml\nkt-ca\nmk-IK\nend-sx\nend-sk\ngy-sx\nend-ca\nca-ml\ngm-CV\nsx-kt\nstart-CV\nIK-start\nCV-kt\nml-mk\nml-CV\nml-gm\nml-IK".to_string().split("\n").map(|a|a.to_string()).collect::<Vec<String>>();

        let result = part_two(input);

        assert!(result.is_ok(), "Result {:?} failed", result);
        assert_eq!(result.unwrap(), 92111);
    }
}
