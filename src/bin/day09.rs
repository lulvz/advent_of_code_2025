use advent_of_code_2025::utils::{self, HasX, HasY, Vector};
use std::{array, cmp::max, cmp::min};

fn read_red_positions(input: &str) -> Vec<Vector<u64, 2>> {
    let mut red_positions: Vec<Vector<u64, 2>> = Vec::new();
    for l in input.lines() {
        let mut parts = l.split(',').map(|s| s.trim().parse::<u64>().unwrap());
        let red_position: Vector<u64, 2> =
            array::from_fn(|_| parts.next().unwrap()).into();
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

fn edge_cuts_rectangle(
    p1: &Vector<u64, 2>, // Polygon Edge Start
    p2: &Vector<u64, 2>, // Polygon Edge End
    r_left: u64,
    r_right: u64,
    r_top: u64,
    r_bottom: u64,
) -> bool {
    // basically find out if the edge is vertical or horizontal, then take the
    // constant value for where that edge is located in the x or y coordinate
    // and check if it's between the bounds of the rectangle, if so, check the
    // edge's other coordinate and see if it overlaps the height/width of the
    // rectangle
    if p1.x() == p2.x() {
        // vertical edge
        let x = *p1.x();
        let y_min = *p1.y().min(p2.y());
        let y_max = *p1.y().max(p2.y());

        // does it pass STRICTLY between the left and right walls?
        // AND does its height overlap with the rectangle?
        x > r_left && x < r_right && y_min < r_top && y_max > r_bottom
    } else {
        // horizontal edge
        let y = *p1.y();
        let x_min = *p1.x().min(p2.x());
        let x_max = *p1.x().max(p2.x());

        // does it pass STRICTLY between the top and bottom walls?
        y > r_bottom && y < r_top && x_min < r_right && x_max > r_left
    }
}

fn part_two(input: &str) -> u64 {
    let red_positions: Vec<Vector<u64, 2>> = read_red_positions(input);
    // polygon is pre built so the list comes already in
    // the final shape of the edges
    let mut max_area = 0;
    for i in 0..red_positions.len() {
        for j in (i + 1)..red_positions.len() {
            let corner_one = red_positions[i];
            let corner_three = red_positions[j];

            let r_right = max(*corner_one.x(), *corner_three.x());
            let r_left = min(*corner_one.x(), *corner_three.x());
            let r_top = max(*corner_one.y(), *corner_three.y());
            let r_bottom = min(*corner_one.y(), *corner_three.y());

            // check if the middle point is in the polygon by counting the
            // amount of crossings to the right, if the number is odd, then the
            // point at the center of this rectangle is inside the polygon
            let r_mid_x = r_left + (r_right - r_left) / 2;
            let r_mid_y = r_bottom + (r_top - r_bottom) / 2;

            let mut crossings_count = 0;
            let mut cuts = false;
            for k in 0..red_positions.len() {
                let p1 = red_positions[k];
                let p2 = red_positions[(k + 1) % red_positions.len()];

                if p1.x() == p2.x() {
                    // vertical edge
                    let x_wall = *p1.x();
                    let y_min = *p1.y().min(p2.y());
                    let y_max = *p1.y().max(p2.y());

                    if x_wall > r_mid_x && r_mid_y >= y_min && r_mid_y <= y_max
                    {
                        crossings_count += 1;
                    }
                }

                if edge_cuts_rectangle(
                    &p1, &p2, r_left, r_right, r_top, r_bottom,
                ) {
                    cuts = true;
                    break;
                }
            }
            // if it's not cut and the amount of crossings is odd, check if the
            // area is bigger than the last valid max_area
            if !cuts && crossings_count % 2 == 1 {
                let r_area =
                    ((r_right - r_left) + 1) * ((r_top - r_bottom) + 1);
                if r_area > max_area {
                    max_area = r_area;
                }
            }
        }
    }

    max_area
}

fn main() {
    match utils::read_input("day09.txt") {
        Ok(input) => {
            println!("Day 09 - Part 1: {}", part_one(input.as_str()));
            println!("Day 09 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day09.txt' exists.");
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

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 24);
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
