use advent_of_code_2025::utils::{self, HasX, HasY, Vector};
use std::array;

fn read_red_positions(input: &str) -> Vec<Vector<u64, 2>> {
    let mut red_positions: Vec<Vector<u64, 2>> = Vec::new();
    for l in input.lines() {
        let mut parts = l.split(',').map(|s| s.trim().parse::<u64>().unwrap());
        let red_position: Vector<u64, 2> = array::from_fn(|_| parts.next().unwrap()).into();
        red_positions.push(red_position);
    }
    red_positions
}

fn part_one(input: &str) -> u64 {
    let red_positions: Vec<Vector<u64, 2>> = read_red_positions(input);

    let mut max_area = 0;
    for i in 0..red_positions.len() {
        for j in (i + 1)..red_positions.len() {
            let sides: Vector<u64, 2> = [
                red_positions[i].x().abs_diff(*red_positions[j].x()) + 1,
                red_positions[i].y().abs_diff(*red_positions[j].y()) + 1,
            ]
            .into();
            let area = sides.x() * sides.y();
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn part_two(_input: &str) -> u64 {
    32
}

fn main() {
    match utils::read_input("day08.txt") {
        Ok(input) => {
            println!("Day 08 - Part 1: {}", part_one(input.as_str()));
            println!("Day 08 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day08.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 50);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }
    // #[test]
    // fn test_part2_example() {
    //     assert_eq!(part_two(EXAMPLE_STRING), 25272);
    // }
    //
    // #[test]
    // fn test_part2_alt_example() {
    //     assert_eq!(part_two_alt(EXAMPLE_STRING), 3263827);
    // }
    //
    // #[test]
    // fn test_part2_simple() {
    //     assert_eq!(part_two("1 3\n4 6\n+ + "), 1);
    // }

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
