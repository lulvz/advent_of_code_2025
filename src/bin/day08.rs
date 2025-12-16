use advent_of_code_2025::utils::{self, HasX, HasY, HasZ, Vector};
use std::array;

#[allow(dead_code)] // Suppress the warning for the whole struct
#[derive(Debug, Copy, Clone)]
struct JBConnection {
    jbox_idxs: (usize, usize),
    distance_squared: u64,
}

fn euclidean_distance_squared(a: &Vector<u64, 3>, b: &Vector<u64, 3>) -> u64 {
    let c1 = b.x().abs_diff(*a.x());
    let c2 = b.y().abs_diff(*a.y());
    let c3 = b.z().abs_diff(*a.z());
    c1 * c1 + c2 * c2 + c3 * c3
}

fn read_jbox_positions(input: &str) -> Vec<Vector<u64, 3>> {
    let mut jbox_positions: Vec<Vector<u64, 3>> = Vec::new();
    for l in input.lines() {
        let mut parts = l.split(',').map(|s| s.trim().parse::<u64>().unwrap());
        let jbox_position: Vector<u64, 3> = array::from_fn(|_| parts.next().unwrap()).into();
        jbox_positions.push(jbox_position);
    }
    jbox_positions
}

fn part_one(input: &str, connections_to_take: usize) -> u64 {
    // We are given a list of junction box positions, in the format 1,2,3\n4,5,6
    // We have to find the junction boxes that are closer together in a straight line
    // After connecting the two closest junction boxes, they form a circuit, then
    // we have to find the next two closest boxes, and if one of them belongs to a
    // circuit already, then that circuit gets a new box
    // We have to return the multilication of the 3 largest circuit sizes

    // basically im gonna slap all the points on a vector of f32 just to read the
    // values into memory, then I'm gonna iterate through the vector and calculate
    // the distances between every pair (I don't exactly know how I would represent
    // this in memory though), then I order those distances, and finally I take the
    // top 1000 distances and make the circuits
    let jbox_positions: Vec<Vector<u64, 3>> = read_jbox_positions(input);

    let mut jbox_connections: Vec<JBConnection> =
        Vec::with_capacity((jbox_positions.len() * (jbox_positions.len() - 1)) / 2);
    for i in 0..jbox_positions.len() {
        for j in i + 1..jbox_positions.len() {
            jbox_connections.push(JBConnection {
                jbox_idxs: (i, j),
                distance_squared: euclidean_distance_squared(
                    &jbox_positions[i],
                    &jbox_positions[j],
                ),
            })
        }
    }
    jbox_connections.sort_by(|a, b| a.distance_squared.cmp(&b.distance_squared));

    // ds_roots has to be same size as the initial jbox_positions vector for the algorithm
    let mut ds_roots: Vec<usize> = (0..jbox_positions.len()).collect();
    let mut ds_sizes: Vec<u64> = vec![1; jbox_positions.len()];
    // println!("{:?}", ds_roots);

    for jbox_connection in jbox_connections.iter().take(connections_to_take) {
        // find the roots of the trees
        let mut j: usize = jbox_connection.jbox_idxs.0;
        while ds_roots[j] != j {
            j = ds_roots[j];
        } // after this loop j will be the root of the disjoint set or tree
        ds_roots[jbox_connection.jbox_idxs.0] = j; // seems optional path compression
        // j has to be same as ds_roots[j]
        let mut k: usize = jbox_connection.jbox_idxs.1;
        while ds_roots[k] != k {
            k = ds_roots[k];
        } // after this loop k will be the root of the disjoint set or tree
        ds_roots[jbox_connection.jbox_idxs.1] = k;
        // join the roots of the trees
        if j != k {
            ds_roots[k] = j;
            ds_sizes[j] += ds_sizes[k];
            ds_sizes[k] = 0;
        }
    }
    ds_sizes.sort_by(|a, b| b.cmp(a));
    ds_sizes.iter().take(3).product()
}

fn part_two(input: &str) -> u64 {
    let jbox_positions: Vec<Vector<u64, 3>> = read_jbox_positions(input);

    let mut jbox_connections: Vec<JBConnection> =
        Vec::with_capacity((jbox_positions.len() * (jbox_positions.len() - 1)) / 2);
    for i in 0..jbox_positions.len() {
        for j in i + 1..jbox_positions.len() {
            jbox_connections.push(JBConnection {
                jbox_idxs: (i, j),
                distance_squared: euclidean_distance_squared(
                    &jbox_positions[i],
                    &jbox_positions[j],
                ),
            })
        }
    }
    jbox_connections.sort_by(|a, b| a.distance_squared.cmp(&b.distance_squared));

    let mut ds_roots: Vec<usize> = (0..jbox_positions.len()).collect();
    let mut ds_sizes: Vec<u64> = vec![1; jbox_positions.len()];

    let mut final_connection: JBConnection = JBConnection {
        jbox_idxs: (0, 0),
        distance_squared: 0,
    };
    // Now we take connections until we have a set that contains all the boxes
    for jbox_connection in jbox_connections.iter() {
        // find the roots of the trees
        let mut j: usize = jbox_connection.jbox_idxs.0;
        while ds_roots[j] != j {
            j = ds_roots[j];
        } // after this loop j will be the root of the disjoint set or tree
        ds_roots[jbox_connection.jbox_idxs.0] = j; // seems optional path compression
        // j has to be same as ds_roots[j]
        let mut k: usize = jbox_connection.jbox_idxs.1;
        while ds_roots[k] != k {
            k = ds_roots[k];
        } // after this loop k will be the root of the disjoint set or tree
        ds_roots[jbox_connection.jbox_idxs.1] = k;
        // join the roots of the trees
        if j != k {
            ds_roots[k] = j;
            ds_sizes[j] += ds_sizes[k];
            ds_sizes[k] = 0;
        }
        if ds_sizes[j] >= jbox_positions.len() as u64 {
            final_connection = *jbox_connection;
            break;
        }
    }
    jbox_positions[final_connection.jbox_idxs.0].x()
        * jbox_positions[final_connection.jbox_idxs.1].x()
}

fn main() {
    match utils::read_input("day08.txt") {
        Ok(input) => {
            println!("Day 08 - Part 1: {}", part_one(input.as_str(), 1000));
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

    const EXAMPLE_STRING: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING, 10), 40);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 25272);
    }
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
