use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    let mut acc: u64 = 0;
    for line in input.lines() {
        // Here filter map consumes the None part if the character is not a digit
        let line_digits: Vec<u64> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u64)
            .collect();
        let mut left: usize = 0;
        // Find the largest digit and the second largest, put them together from left to right
        for (i, d) in line_digits[0..line_digits.len() - 1].iter().enumerate() {
            if *d > line_digits[left] {
                left = i;
            }
        }
        let mut right: usize = left + 1;
        for i in left + 1..line_digits.len() {
            if line_digits[i] > line_digits[right] {
                right = i;
            }
        }
        acc += line_digits[left] * 10 + line_digits[right];
    }
    acc
}

fn part_two(input: &str) -> u64 {
    let mut acc: u64 = 0;
    for line in input.lines() {
        // Here filter map consumes the None part if the character is not a digit
        let line_digits: Vec<u64> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u64)
            .collect();
        // for this one twelve batteries are turned on, so have to always let space at the end
        // when checking for largest number
        let mut indices = [0; 12];
        let mut small_acc = 0;
        let mut turning_on = 0;
        while turning_on < 12 {
            for i in
                indices[turning_on]..line_digits.len() - (12 - turning_on - 1)
            {
                if line_digits[i] > line_digits[indices[turning_on]] {
                    indices[turning_on] = i;
                }
            }
            // set the next starting point
            if turning_on < 11 {
                indices[turning_on + 1] = indices[turning_on] + 1;
            }
            small_acc *= 10;
            small_acc += line_digits[indices[turning_on]];
            turning_on += 1;
        }
        acc += small_acc;
    }
    acc
}

fn main() {
    match utils::read_input("day03.txt") {
        Ok(input) => {
            println!("Day 03 - Part 1: {}", part_one(input.as_str()));
            println!("Day 03 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day03.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str =
        "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 357);
    }

    #[test]
    fn test_part1_simple() {
        assert_eq!(part_one("987654321111111"), 98);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 3121910778619);
    }

    #[test]
    fn test_part2_simple() {
        assert_eq!(part_two("987654321111111"), 987654321111);
    }

    // #[test]
    // fn test_part2_simple_second() {
    //     assert_eq!(part_two("811111111111119"), 811111111119);
    // }
    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
