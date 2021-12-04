use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<(&str, i32)> for Command {
    type Error = anyhow::Error;
    fn try_from(value: (&str, i32)) -> Result<Self> {
        let (command_str, command_strength) = value;

        match command_str {
            "forward" => Ok(Command::Forward(command_strength)),
            "down" => Ok(Command::Down(command_strength)),
            "up" => Ok(Command::Up(command_strength)),
            _ => Err(anyhow!(
                "command \"{:?}\" did not match the valid options of forward, down, up",
                command_str
            )),
        }
    }
}

pub fn parse_input<P>(input_path: P) -> Result<Vec<Command>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|input_line| {
            let mut input_line_split = input_line.trim().split(' ');
            let command_str = input_line_split.next();
            let command_strength = input_line_split.next();

            match (command_str, command_strength) {
                (Some(command_str), Some(command_strength)) => {
                    match command_strength.parse::<i32>() {
                        Ok(command_strength) => Command::try_from((command_str, command_strength)),
                        Err(_) => Err(anyhow!(
                            "command_strength \"{:?}\" was not a valid i32",
                            command_strength
                        )),
                    }
                }
                _ => Err(anyhow!(
                    "Input line \"{:?}\" contained a number of commands not equal to two",
                    input_line
                )),
            }
        })
        .collect()
}

/// Return the final depth * final horizontal position using the rules given by part one
///
///
/// - forward X increases the horizontal position by X units.
/// - down X increases the depth by X units.
/// - up X decreases the depth by X units.
pub fn part_one<I>(input: I) -> Result<i32>
where
    I: IntoIterator<Item = Command>,
{
    let (final_position, final_depth) = input.into_iter().fold(
        (0, 0),
        |(horizontal_delta, vertical_delta), current_command| match current_command {
            Command::Forward(strength) => (horizontal_delta + strength, vertical_delta),
            Command::Down(strength) => (horizontal_delta, vertical_delta + strength),
            Command::Up(strength) => (horizontal_delta, vertical_delta - strength),
        },
    );

    Ok(final_position * final_depth)
}
/// Return the final depth * final horizontal position using the rules given by part one
///
/// - down X increases your aim by X units.
/// - up X decreases your aim by X units.
/// - forward X does two things:
///     1. It increases your horizontal position by X units.
///     2. It increases your depth by your aim multiplied by X
pub fn part_two<I>(input: I) -> Result<i32>
where
    I: IntoIterator<Item = Command>,
{
    let (final_position, final_depth, _aim) = input.into_iter().fold(
        (0, 0, 0),
        |(horizontal_delta, vertical_delta, aim), current_command| match current_command {
            Command::Forward(strength) => (
                horizontal_delta + strength,
                vertical_delta + (aim * strength),
                aim,
            ),
            Command::Down(strength) => (horizontal_delta, vertical_delta, aim + strength),
            Command::Up(strength) => (horizontal_delta, vertical_delta, aim - strength),
        },
    );

    Ok(final_position * final_depth)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two, Command};

    #[test]
    fn test_part_one_no_commands() {
        let result = part_one([]);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_part_one_forward_once() {
        let result = part_one([Command::Forward(5)]);
        assert_eq!(
            result.unwrap(),
            0,
            "Since the depth was 0 we expect the result to be 0"
        );
    }

    #[test]
    fn test_part_one_up_once_forward_once() {
        let result = part_one([Command::Up(5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), -25);
    }

    #[test]
    fn test_part_one_down_once_forward_once() {
        let result = part_one([Command::Down(5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), 25);
    }

    #[test]
    fn test_part_one_forward_negative() {
        let result = part_one([Command::Down(5), Command::Forward(-5)]);
        assert_eq!(result.unwrap(), -25);
    }

    #[test]
    fn test_part_one_down_negative() {
        let result = part_one([Command::Down(-5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), -25);
    }

    #[test]
    fn test_part_one_up_negative() {
        let result = part_one([Command::Up(-5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), 25);
    }

    #[test]
    fn test_part_one_example_part_one() {
        let result = part_one([
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ]);

        assert_eq!(result.unwrap(), 150);
    }

    #[test]
    fn test_part_one_my_input() {
        let my_input_parsed = parse_input("src/input.txt").unwrap();
        let result = part_one(my_input_parsed);

        assert_eq!(result.unwrap(), 2091984);
    }

    #[test]
    fn test_part_two_down_forward() {
        let result = part_two([Command::Down(5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), 125);
    }

    #[test]
    fn test_part_two_up_forward() {
        let result = part_two([Command::Up(5), Command::Forward(5)]);
        assert_eq!(result.unwrap(), -125);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two([
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ]);

        assert_eq!(result.unwrap(), 900);
    }

    #[test]
    fn test_part_two_my_input() {
        let my_input_parsed = parse_input("src/input.txt").unwrap();
        let result = part_two(my_input_parsed);

        assert_eq!(result.unwrap(), 2086261056);
    }
}
