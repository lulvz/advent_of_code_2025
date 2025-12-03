use advent_of_code_2025::utils; 

fn part_one(input: &str) -> u32 {
    // The file contains ranges (start-end) separated by commas, like
    // 10-20,30-540
    // We have to identify the IDs in the ranges that are 
    // "made only of some sequence of digits repeated twice" 
    // Finally simply sum all of them and return the value
    for range in input.split(",") {
        range.split_once("-")
    }
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

    }
    0
}

fn part_two(input: &str) -> u32 {
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

    }
    32
}

fn main() {
    // Use the imported function to read the specific input file
    match utils::read_input("day02.txt") {
        Ok(input) => {
            println!("Day 01 - Part 1: {}", part_one(input.as_str()));
            println!("Day 01 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day02.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const example_string: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224, \
            1698522-1698528,446443-446449,38593856-38593862,565653-565659, \
            824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_add() {
        // assert_eq!(part_one(), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
