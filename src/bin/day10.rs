use advent_of_code_2025::utils;
use std::collections::HashSet;

// use a bitmask to represent the state
#[derive(Debug, Clone)]
struct Machine {
    desired_indicator_lights: u64, // bitmask of the final state
    button_wiring_schematics: Vec<u64>, // vec of bitmasks
    joltage_requirements: Vec<u64>, // vec of joltage requirements
}

fn read_input(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    for l in input.lines() {
        let (lights_wrap, rest) =
            l.split_once(' ').expect("Missing space after lights");
        let (buttons_wrap, joltage_wrap) =
            rest.rsplit_once(' ').expect("Missing space before joltage");

        let mut desired_indicator_lights: u64 = 0;
        // we are gonna fill the mask from the right to the left, but the input
        // has index 0 at the left, either way it should work the same
        for (i, c) in lights_wrap.chars().skip(1).enumerate() {
            if c == '#' {
                desired_indicator_lights |= 1 << i;
            }
        }

        let mut button_wiring_schematics: Vec<u64> = Vec::new();
        for button in buttons_wrap.split_whitespace() {
            let mut button_wiring_schematic: u64 = 0;
            for idx in
                button.trim_matches(|bs| bs == '(' || bs == ')').split(',')
            {
                button_wiring_schematic |= 1 << idx.parse::<u64>().unwrap();
            }
            button_wiring_schematics.push(button_wiring_schematic);
        }

        let mut joltage_requirements: Vec<u64> = Vec::new();
        for joltage in joltage_wrap
            .trim_matches(|js| js == '{' || js == '}')
            .split(',')
        {
            joltage_requirements.push(joltage.parse::<u64>().unwrap());
        }

        machines.push(Machine {
            desired_indicator_lights,
            button_wiring_schematics,
            joltage_requirements,
        })
    }
    machines
}

fn part_one(input: &str) -> u64 {
    let machines = read_input(input);
    // print machines buttons and desired lights as bitmask
    // for machine in machines.iter() {
    //     println!(
    //         "Machine: buttons: {:?}, desired lights: {:b}",
    //         machine.button_wiring_schematics, machine.desired_indicator_lights
    //     );
    // }

    // VERY INNEFICIENT WITH HASHSET, BUT IT WORKS :/
    let mut acc: u64 = 0;
    for machine in machines.iter() {
        // do a sort of bfs, but without storing every past state, only the
        // last level states and the current depth level which will be the
        // smallest amount of presses needed to reach the desired final light
        // state when we find it somewhere
        let mut last_states: HashSet<u64> = HashSet::new();
        last_states.insert(0u64);
        let mut current_level: u64 = 1;

        'bfs: loop {
            let mut next_states: HashSet<u64> = HashSet::new();
            for button in machine.button_wiring_schematics.iter() {
                for s in last_states.iter() {
                    let resulting_lights = s ^ button;
                    // println!(
                    //     "{:b}, {:b}",
                    //     resulting_lights, machine.desired_indicator_lights
                    // );
                    if resulting_lights == machine.desired_indicator_lights {
                        break 'bfs;
                    }
                    next_states.insert(resulting_lights);
                }
            }
            current_level += 1;
            last_states = next_states;
        }

        acc += current_level;
    }
    acc
}

fn part_two_bfs(input: &str) -> u64 {
    // now each button's number adds one to the respective joltage counter
    // ignore the logic for the indicator lights
    let machines = read_input(input);

    let mut acc: u64 = 0;
    for machine in machines.iter() {
        let mut last_states: HashSet<Vec<u64>> = HashSet::new();
        last_states.insert(vec![0; machine.joltage_requirements.len()]);
        let mut current_level: u64 = 1;

        'bfs: loop {
            let mut next_states: HashSet<Vec<u64>> = HashSet::new();
            for button in machine.button_wiring_schematics.iter() {
                for s in last_states.iter() {
                    let mut resulting_joltages: Vec<u64> = s.clone();
                    for (i, resulting_joltage) in
                        resulting_joltages.iter_mut().enumerate()
                    {
                        *resulting_joltage += (button >> i) & 0b1;
                    }

                    let over_limit = resulting_joltages
                        .iter()
                        .zip(&machine.joltage_requirements)
                        .any(|(rj, jr)| rj > jr);

                    if resulting_joltages == machine.joltage_requirements {
                        break 'bfs;
                    }
                    if !over_limit {
                        next_states.insert(resulting_joltages);
                    }
                }
            }
            current_level += 1;
            last_states = next_states;
        }

        acc += current_level;
    }
    acc
}

fn part_two_linalg(_input: &str) -> u64 {
    32
}

fn main() {
    match utils::read_input("day10.txt") {
        Ok(input) => {
            println!("Day 10 - Part 1: {}", part_one(input.as_str()));
            println!("Day 10 - Part 2 bfs: {}", part_two_bfs(input.as_str()));
            println!(
                "Day 10 - Part 2 bfs: {}",
                part_two_linalg(input.as_str())
            );
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day10.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str =
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 7);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }

    #[test]
    fn test_part2_bfs_example() {
        assert_eq!(part_two_bfs(EXAMPLE_STRING), 33);
    }

    #[test]
    fn test_part2_linalg_example() {
        assert_eq!(part_two_linalg(EXAMPLE_STRING), 33);
    }
    // #[test]
    // fn test_part2_example_custom() {
    //     assert_eq!(part_two(EXAMPLE_STRING_CUSTOM), 24);
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
