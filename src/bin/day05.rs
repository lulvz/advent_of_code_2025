use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    // The input is going to be a list of ranges of fresh product IDs, so like
    // 1-4\n6-7 which are inclusive, then an empty line, and then the available
    // product IDs, one per line. We have to find the amount of fresh available
    // products using their IDs and the ranges
    let mut acc: u64 = 0;
    let (id_ranges_str, ids_str) = input.split_once("\n\n").expect("Couldn't find empty line to separate ranges from IDs");
    let mut id_ranges: Vec<(u64, u64)> = Vec::new();
    for l in id_ranges_str.lines() {
        // get the range in tuples and put it in the vector
        let id_range_str = l.split_once("-")
            .expect("Malformed range");
        id_ranges.push((id_range_str.0.parse().expect("Not a number (start)"),
            id_range_str.1.parse().expect("Not a number (end)")));
        
    }
    id_ranges.sort_by(|id_range_a, id_range_b| id_range_a.0.cmp(&id_range_b.0));

    let mut id_ranges_merged: Vec<(u64, u64)> = Vec::new();
    for i in 0..id_ranges.len() {
        match id_ranges_merged.last_mut() {
            Some(last_range) => {
                if last_range.1 >= id_ranges[i].0 { // they intercept
                    if id_ranges[i].1 > last_range.1 {
                        last_range.1 = id_ranges[i].1;
                    }
                } else {
                    id_ranges_merged.push(id_ranges[i]);
                }
            },
            None => {
                id_ranges_merged.push(id_ranges[i]);
            }
        }
    }

    // for id_range_merged in id_ranges_merged.iter() {
        // println!("{:?}", id_range_merged);
    // }

    // TODO BINARY SEARCH HERE
    for id_str in ids_str.lines() {
        let id: u64 = id_str.parse().expect("ID not a number");
        for id_range_merged in id_ranges_merged.iter() {
            if id >= id_range_merged.0 && id <= id_range_merged.1 {
                acc += 1;
            }
        }
    }

    acc
}

fn part_two(input: &str) -> u64 {
    // For the second part we have to find every ID that is fresh, so that would
    // be every ID in the ranges
    let mut acc: u64 = 0;
    let (id_ranges_str, _ids_str) = input.split_once("\n\n").expect("Couldn't find empty line to separate ranges from IDs");
    let mut id_ranges: Vec<(u64, u64)> = Vec::new();
    for l in id_ranges_str.lines() {
        // get the range in tuples and put it in the vector
        let id_range_str = l.split_once("-")
            .expect("Malformed range");
        id_ranges.push((id_range_str.0.parse().expect("Not a number (start)"),
            id_range_str.1.parse().expect("Not a number (end)")));
        
    }
    id_ranges.sort_by(|id_range_a, id_range_b| id_range_a.0.cmp(&id_range_b.0));

    let mut id_ranges_merged: Vec<(u64, u64)> = Vec::new();
    for i in 0..id_ranges.len() {
        match id_ranges_merged.last_mut() {
            Some(last_range) => {
                if last_range.1 >= id_ranges[i].0 { // they intercept
                    if id_ranges[i].1 > last_range.1 {
                        last_range.1 = id_ranges[i].1;
                    }
                } else {
                    id_ranges_merged.push(id_ranges[i]);
                }
            },
            None => {
                id_ranges_merged.push(id_ranges[i]);
            }
        }
    }

    for id_range_merged in id_ranges_merged.iter() {
        acc += id_range_merged.1 - id_range_merged.0 + 1;
        // println!("{:?}", id_range_merged);
    }

    acc
}

fn main() {
    match utils::read_input("day05.txt") {
        Ok(input) => {
            println!("Day 05 - Part 1: {}", part_one(input.as_str()));
            println!("Day 05 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day05.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 3);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 14);
    }
    //
    // #[test]
    // fn test_part2_simple() {
    //     assert_eq!(part_two("987654321111111"), 987654321111);
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
