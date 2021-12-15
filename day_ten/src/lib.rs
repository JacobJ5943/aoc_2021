fn validate_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();
    for current_char in line.trim().chars() {
        match current_char {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '<' => stack.push('>'),
            _ => {
                if !stack.is_empty() && stack.last().unwrap() == &current_char {
                    stack.pop();
                } else {
                    return Err(current_char);
                }
            }
        }
    }
    Ok(stack.into_iter().rev().collect())
}

pub fn part_one(input: &[String]) -> usize {
    input.iter().fold(0, |total_score, line| {
        total_score
            + match validate_line(line) {
                Ok(_) => 0,
                Err(failed_char) => match failed_char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Unexpected character {:?}", failed_char),
                },
            }
    })
}

pub fn part_two(input: &[String]) -> usize {
    let incomplete_lines: Vec<Vec<char>> = input
        .iter()
        .map(|line| validate_line(line))
        .filter(|result| result.is_ok())
        .map(|x| x.unwrap())
        .collect();

    let mut scores = Vec::with_capacity(incomplete_lines.len());

    for line in incomplete_lines {
        // Needs to be rev because my stack is in reverse order
        scores.push(
            line.iter()
                .fold(0, |total_score, current_char| match current_char {
                    ')' => total_score * 5 + 1,
                    ']' => total_score * 5 + 2,
                    '}' => total_score * 5 + 3,
                    '>' => total_score * 5 + 4,
                    _ => panic!("unexpected_char {:?}", current_char),
                }),
        )
    }

    scores.sort_unstable();
    *scores
        // floor is used instead of ceil becuase vectors start at index 0
        .get(f64::floor(scores.len() as f64 / 2.0) as usize)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, validate_line};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_example() {
        let example_input:Vec<String> = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]".split('\n').map(|x|x.to_string()).collect();
        assert_eq!(part_one(&example_input), 26397);
    }

    #[test]
    fn test_part_one_my_input() {
        let file = File::open("src/input.txt").unwrap();
        let reader = BufReader::new(file);
        let my_input: Vec<String> = reader.lines().flatten().collect();
        assert_eq!(part_one(&my_input), 316851);
    }


    #[test]
    fn test_part_two_my_input() {
        let file = File::open("src/input.txt").unwrap();
        let reader = BufReader::new(file);
        let my_input: Vec<String> = reader.lines().flatten().collect();
        assert_eq!(part_two(&my_input), 316851);
    }

    #[test]
    fn test_part_two_example() {
        let example_input:Vec<String> = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]".split('\n').map(|x|x.to_string()).collect();
        assert_eq!(part_two(&example_input), 288957);
    }

    #[test]
    fn test_validate_line() {
        let input = "[({(<(())[]>[[{[]{<()<>>";
        let expected_result = vec!['}', '}', ']', ']', ')', '}', ')', ']'];
        assert_eq!(validate_line(input).unwrap(), expected_result);

        let input = "[(()[<>])]({[<{<<[]>>(";
        let expected_result = vec![')', '}', '>', ']', '}', ')'];
        assert_eq!(validate_line(input).unwrap(), expected_result);

        let input = "(((({<>}<{<{<>}{[]{[]{}";
        let expected_result = vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'];
        assert_eq!(validate_line(input).unwrap(), expected_result);

        let input = "{<[[]]>}<{[{[{[]{()[[[]";
        let expected_result = vec![']', ']', '}', '}', ']', '}', ']', '}', '>'];
        assert_eq!(validate_line(input).unwrap(), expected_result);

        let input = "<{([{{}}[<[[[<>{}]]]>[]]";
        let expected_result = vec![']', ')', '}', '>'];
        assert_eq!(validate_line(input).unwrap(), expected_result);
    }
}
