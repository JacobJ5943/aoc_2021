/// Every fish is the same so all we need to know is how many fish are on each day.
fn part_two(input: Vec<usize>, num_days: usize) -> usize {
    let mut lantern_fish = [0; 9];


    for input_fish in input {
        lantern_fish[input_fish] += 1;
    }

    for _day in 0..num_days {
        lantern_fish.rotate_left(1);
        lantern_fish[6] += lantern_fish[8];
    }

    lantern_fish.iter().sum()
}


/// The naive approach to keep track of every fish individually and update them all individually.
fn part_one(mut input: Vec<usize>) -> usize {
    let mut new_fish = Vec::new();
    for _ in 0..80 {
        for fish in input.iter_mut() {
            if *fish == 0 {
                new_fish.push(8);
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        for fish in new_fish {
            input.push(fish);
        }
        new_fish = Vec::new();
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_part_one() {
        let example_input = vec![3, 4, 3, 1, 2];
        assert_eq!(part_one(example_input), 5934);
    }

    #[test]
    fn test_my_input_part_one() {
        let example_input = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2".split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        assert_eq!(part_one(example_input), 394994);
    }

    #[test]
    fn test_my_input_part_two() {
        let example_input = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2".split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        assert_eq!(part_two(example_input, 256), 1765974267455);
    }

    #[test]
    fn test_example_part_two() {
        let example_input = vec![3, 4, 3, 1, 2];
        assert_eq!(part_two(example_input, 256), 26984457539);
    }

    #[test]
    fn test_example_part_two_with_80() {
        let example_input = vec![3, 4, 3, 1, 2];
        assert_eq!(part_two(example_input, 80), 5934);
    }

    #[test]
    fn test_example_part_two_with_18() {
        let example_input = vec![3, 4, 3, 1, 2];
        assert_eq!(part_two(example_input, 18), 26);
    }
}
