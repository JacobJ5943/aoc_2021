use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::{io::BufRead, path::Path};

pub fn parse_input<P>(input_path: P) -> Result<Vec<usize>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .flatten()
        .map(|y| {
            y.trim()
                .parse::<usize>()
                .expect("Every line should be an integer")
        })
        .collect())
}

/// Return the number of elements that are larger than its previous element
pub fn part_one(input: &[usize]) -> usize {
    input
        .iter()
        .fold(
            (0, usize::MAX),
            |(increasing_count, previous_element), element| {
                if *element > previous_element {
                    (increasing_count + 1, *element)
                } else {
                    (increasing_count, *element)
                }
            },
        )
        .0
}

/// Return the number of times the rolling window was greater than the previous window
/// A rolling window W consists of 3 elements whose previous window contained the first 2 elements of W
/// and the next window contains the last two elements of W.
/// IMPORTANT input must contain elements who any 3 added together does not exceed the value of usize::MAX
pub fn part_two(input: &[usize]) -> Result<usize> {
    let mut prev_window = [usize::MAX; 3];
    let mut prev_sum;
    let mut curr_window = [usize::MAX; 3];
    let mut curr_sum;

    let first_four = input.iter().take(4).copied().collect::<Vec<usize>>();
    if first_four.len() < 4 {
        Ok(0)
    } else {
        prev_window[0] = first_four[0];
        prev_window[1] = first_four[1];
        prev_window[2] = first_four[2];
        prev_sum = first_four[0] + first_four[1] + first_four[2];

        curr_window[0] = first_four[1];
        curr_window[1] = first_four[2];
        curr_sum = first_four[1] + first_four[2];

        let mut current_increasing = 0;
        for x in input.iter().skip(3) {
            curr_window[2] = *x;
            curr_sum += *x;
            if prev_sum < curr_sum {
                current_increasing += 1;
            }

            // Prepare for the next loop
            prev_window.rotate_left(1);
            prev_sum -= prev_window[2];

            prev_window[2] = curr_window[2];
            prev_sum += prev_window[2];
            assert_eq!(
                prev_sum, curr_sum,
                "prev_window:{:?} and curr_window:{:?} should be the same window at this time",
                prev_window, curr_window
            ); //

            curr_sum -= curr_window[0];
            curr_window.rotate_left(1);
        }

        Ok(current_increasing)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_two};

    use super::part_one;

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]).unwrap(),
            5
        );
        assert_eq!(
            part_two(&[199, 200, 208]).unwrap(),
            0,
            "Failed to return 0 with only 3 elements"
        );
        assert_eq!(
            part_two(&[199, 200]).unwrap(),
            0,
            "Failed to return 0 with only 2 elements"
        );
        assert_eq!(
            part_two(&[199]).unwrap(),
            0,
            "Failed to return 0 with only 1 elements"
        );
        assert_eq!(
            part_two(&[]).unwrap(),
            0,
            "Failed to return 0 with only 0 elements"
        );
    }

    #[test]
    fn test_part_two_my_input() {
        assert_eq!(
            part_two(&parse_input("src/input.txt").unwrap()).unwrap(),
            1235
        );
    }
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&[]),
            0,
            "The empty vector cannot have a previous element to be greater than"
        );
        assert_eq!(part_one(&[1, 2, 3, 4]), 3, "1,2,3,4");
        assert_eq!(part_one(&[4, 3, 2, 1]), 0, "4,3,2,1");
        assert_eq!(
            part_one(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7,
            "example:[199,200,208,210,200,207,240,269,260,263]"
        );
    }

    #[test]
    fn part_one_my_input() {
        assert_eq!(part_one(&parse_input("src/input.txt").unwrap()), 1195);
    }
}
