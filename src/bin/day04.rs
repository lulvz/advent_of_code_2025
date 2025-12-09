use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    // There is a grid of rolls of paper represented by '@', and empty spaces represented by '.',
    // we have to find the rolls of paper that can be accessed
    // For a roll of paper to be accessible, there have to be less than 4 rolls of paper in the 8
    // adjacent positions
    // Maybe make a 2d matrix of bools idk
    let printing_department_map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut acc = 0;
    for i in 0..printing_department_map.len() {
        for j in 0..printing_department_map[i].len() {
            if !printing_department_map[i][j] {
                continue;
            }
            let mut neighbours = 0;

            for li in -1..=1_isize {
                let ei = i as isize + li;
                if ei < 0 || ei as usize >= printing_department_map.len() {
                    continue;
                }
                for lj in -1..=1_isize {
                    if li == 0 && lj == 0 {
                        continue;
                    }

                    let ej = j as isize + lj;
                    if ej < 0 || ej as usize >= printing_department_map[ei as usize].len() {
                        continue;
                    }

                    if printing_department_map[ei as usize][ej as usize] {
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 4 {
                acc += 1;
            }
        }
    }

    acc
}

fn part_two(input: &str) -> u64 {
    // for part two we have to remove the rolls of paper we identify
    // and in the next iteration identify the new rolls of paper accesible
    // and remove them too, repeating this, then return the total removed
    let mut printing_department_map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut acc = 0;
    let mut last_removed = 1;
    let mut last_removed_positions: Vec<(usize, usize)> = Vec::new();
    while last_removed > 0 {
        for last_removed_position in last_removed_positions.iter() {
            printing_department_map[last_removed_position.0][last_removed_position.1] = false;
        }
        last_removed_positions.clear();
        last_removed = 0;
        for i in 0..printing_department_map.len() {
            for j in 0..printing_department_map[i].len() {
                if !printing_department_map[i][j] {
                    continue;
                }
                let mut neighbours = 0;

                for li in -1..=1_isize {
                    let ei = i as isize + li;
                    if ei < 0 || ei as usize >= printing_department_map.len() {
                        continue;
                    }
                    for lj in -1..=1_isize {
                        if li == 0 && lj == 0 {
                            continue;
                        }

                        let ej = j as isize + lj;
                        if ej < 0 || ej as usize >= printing_department_map[ei as usize].len() {
                            continue;
                        }

                        if printing_department_map[ei as usize][ej as usize] {
                            neighbours += 1;
                        }
                    }
                }

                if neighbours < 4 {
                    last_removed += 1;
                    last_removed_positions.push((i, j));
                }
            }
        }
        acc += last_removed;
    }

    acc
}

fn main() {
    match utils::read_input("day04.txt") {
        Ok(input) => {
            println!("Day 04 - Part 1: {}", part_one(input.as_str()));
            println!("Day 04 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day04.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 13);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 43);
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
