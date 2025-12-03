use advent_of_code_2025::utils; 

fn part_one(input: &str) -> u32 {
    // Every line has a denominator for the direction of rotation and the amount
    // of steps to rotate, like R21, would be rotate right 21 times
    // There are 100 positions, from 0 to 99, so we can do a modulo 100 and
    // treat rotating right as addition and rotating left as subtraction
    // Finally, have to count the amount of times the knob pointer lands at 0
    // after a rotation
    let mut rotator_state = 50;
    let mut zero_stops = 0;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        match direction {
            'R' =>  {
                rotator_state = (rotator_state + amount) % 100;
            },
            'L' => {
                rotator_state = (rotator_state - amount) % 100;
            },
            _ => {},
        }

        if rotator_state == 0 {
            zero_stops += 1;
        }
    }

    zero_stops
}

fn part_two(input: &str) -> u32 {
    // Same thing as part one, but we have to count every time the pointer
    // passes by 0 too
    let mut rotator_state = 50;
    let mut zero_stops: u32 = 0;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        match direction {
            'R' =>  {
                let raw_rotator_state = rotator_state + amount;
                zero_stops = zero_stops + ((raw_rotator_state / 100).abs() as u32);
                // println!("Zero stops R {:?}", zero_stops);
                rotator_state = raw_rotator_state % 100;
            },
            'L' => {
                let raw_rotator_state = rotator_state - amount;
                // println!("raw: {:?}", raw_rotator_state);
                if raw_rotator_state < 0 {
                    zero_stops += ((raw_rotator_state-100) / 100).abs() as u32;
                    if rotator_state == 0 {
                        zero_stops -= 1;
                    }
                    // println!("Zero stops L {:?}", zero_stops);
                }
                if raw_rotator_state == 0 {
                    zero_stops += ((raw_rotator_state-100) / 100).abs() as u32;
                }
                rotator_state = ((raw_rotator_state % 100) + 100) % 100;

            },
            _ => {},
        }
        // println!("{:?}", rotator_state);

        // if rotator_state == 0 {
        //     zero_stops += 1;
        // }
    }

    zero_stops
}

fn main() {
    // Use the imported function to read the specific input file
    match utils::read_input("day01.txt") {
        Ok(input) => {
            println!("Day 01 - Part 1: {}", part_one(input.as_str()));
            println!("Day 01 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day01.txt' exists.");
        }
    }
}