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
        for (i, d) in line_digits[left + 1..line.len()].iter().enumerate() {
            if *d > line_digits[right] {
                right = i;
            }
        }
        acc += line_digits[left] * 10 + line_digits[right];
    }
    acc
}

fn part_two(_input: &str) -> u64 {
    32
}

fn main() {
    match utils::read_input("day02.txt") {
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

    // #[test]
    // fn test_part2_example() {
    //     assert_eq!(part_two(EXAMPLE_STRING), 4174379265);
    // }
    //
    // #[test]
    // fn test_part2_simple() {
    //     assert_eq!(part_two("2121212118-2121212124"), 2121212121);
    // }
    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
