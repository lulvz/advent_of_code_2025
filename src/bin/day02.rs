use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    // The file contains ranges (start-end) separated by commas, like
    // 10-20,30-540
    // We have to identify the IDs in the ranges that are
    // "made only of some sequence of digits repeated twice"
    // Finally simply sum all of them and return the value
    let mut acc: u64 = 0;
    for range in input.split(",") {
        // just do an unwrap since every range is complete
        let range_vector = range.split_once("-").unwrap();
        let start: u64 = range_vector.0.parse().expect("Failed to parse start value");
        println!("{:?}", start);
        let end: u64 = range_vector.1.parse().expect("Failed to parse end value");
        println!("{:?}", end);
        for i in start..=end {
            let i_str = i.to_string();
            if i_str[0..i_str.len() / 2] == i_str[i_str.len() / 2..i_str.len()] {
                acc += i;
            }
        }
    }
    acc
}

fn part_two(input: &str) -> u64 {
    // Now an ID is invalid if the digits repeat at least twice
    let mut acc: u64 = 0;
    for range in input.split(",") {
        // just do an unwrap since every range is complete
        let range_vector = range.split_once("-").unwrap();
        let start: u64 = range_vector.0.parse().expect("Failed to parse start value");
        // println!("{:?}", start);
        let end: u64 = range_vector.1.parse().expect("Failed to parse end value");
        // println!("{:?}", end);
        for i in start..=end {
            let mut repeats = false;
            let i_str = i.to_string();
            // a pattern can be at max half the size of the number, maybe check every possibility
            // inside that half against the rest of the number, starting at pos 0 length 1
            for pl in 1..=i_str.len() {
                if i_str.len() % pl != 0 {
                    break;
                }
                let pattern = &i_str[0..pl];
                for p in 1..i_str.len() / pl {
                    // if pl > 1 {
                    // println!("p={:?}, pl={:?}", p, pl);
                    // println!("p*pl={:?}, p*pl+pl={:?}", p * pl, p * pl + pl);
                    // }
                    if &i_str[pl * p..pl * p + pl] != pattern {
                        break;
                    }
                    if pl * p + pl == i_str.len() {
                        repeats = true;
                    }
                }
            }
            // println!("i: {:?}", i);
            if repeats {
                acc += i;
            }
        }
    }
    acc
}

fn main() {
    match utils::read_input("day02.txt") {
        Ok(input) => {
            println!("Day 02 - Part 1: {}", part_one(input.as_str()));
            println!("Day 02 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day02.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 4174379265);
    }

    #[test]
    fn test_part2_simple() {
        assert_eq!(part_two("2121212118-2121212124"), 2121212121);
    }
    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
