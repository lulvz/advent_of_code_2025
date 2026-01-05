use std::collections::VecDeque;

use advent_of_code_2025::utils;

type Device = [u8; 3];

// Compressed Sparse Row (CSR)
// the edges would have like 3 5 6 all next to eachother, in a node that has
// edges to those indexes for example node 0. So edges[0] = 3, edges[1] = 5,
// edges[2] = 6. And offsets[0] = 0. Then it would follow that offsets[1] = 3
// because that's the next available spot. And we can determine the amount of
// outgoing edges 0 has by doing offsets[1] - offsets[0]
struct AdjacencyList {
    edges: Vec<usize>, // vector of compacted edges outgoing from every node
    offsets: Vec<usize>, // offsets indexing into the edges vec indicating the
                       // start of the current node's outgoing edges
}

struct DeviceGraph {
    devices: Vec<Device>,
    connections: AdjacencyList,
}

fn read_input(input: &str) -> DeviceGraph {
    let mut devices: Vec<Device> = Vec::new();
    // could entirely remove this storage of edges, but honestly i started out
    // by reading the edges and devices, then moved on to a CSR becasue it's
    // better for bfs and I don't wanna change it now cause it works
    // after removing this, simply doing two passes over the input data would
    // work too to fill the adjacency list
    let mut edge_list: Vec<(usize, usize)> = Vec::new();

    for l in input.lines() {
        let (start_device_name, rest) = l
            .split_once(':')
            .map(|(a, b)| (a.as_bytes(), b.as_bytes()))
            .expect("Couldn't split line at ':'");

        assert_eq!(
            start_device_name.len(),
            3,
            "device name's len is different than 3"
        );

        let start_device: Device = start_device_name
            .try_into()
            .expect("couldn't convert into [u8; 3]");

        // using a vec and linearly searching is probably faster than a hashmap
        // for smaller device amounts
        let start_device_id = devices
            .iter()
            .position(|&d| d == start_device) // iterator is consumed here
            .unwrap_or_else(|| {
                devices.push(start_device);
                devices.len() - 1
            });

        for end_device_name in rest.split(|&b| b == b' ') {
            if end_device_name.len() != 3 {
                continue;
            }
            let end_device: Device = end_device_name
                .try_into()
                .expect("couldn't convert end device name into Device type");

            let end_device_id = devices
                .iter()
                .position(|&d| d == end_device)
                .unwrap_or_else(|| {
                    devices.push(end_device);
                    devices.len() - 1
                });

            edge_list.push((start_device_id, end_device_id));
        }
    }

    let mut outgoing_edges_count: Vec<usize> = vec![0; devices.len()];
    for &edge in edge_list.iter() {
        outgoing_edges_count[edge.0] += 1;
    }

    let mut connections = AdjacencyList {
        edges: vec![0; edge_list.len()],
        offsets: Vec::with_capacity(devices.len() + 1),
    };

    let mut total: usize = 0;
    for &outgoing_edge_count in outgoing_edges_count.iter() {
        connections.offsets.push(total);
        total += outgoing_edge_count;
    }
    // last value is used just to get the amount of edges for the last node
    connections.offsets.push(total);

    let mut current_offsets_in_use = connections.offsets.clone();
    for &edge in edge_list.iter() {
        connections.edges[current_offsets_in_use[edge.0]] = edge.1;
        current_offsets_in_use[edge.0] += 1;
    }

    DeviceGraph {
        devices,
        connections,
    }
}

fn part_one(input: &str) -> u64 {
    let device_graph = read_input(input);

    let you_idx = device_graph
        .devices
        .iter()
        .position(|d| d == b"you")
        .unwrap();
    let out_idx = device_graph
        .devices
        .iter()
        .position(|d| d == b"out")
        .unwrap();

    let mut independent_paths: Vec<u64> = vec![0; device_graph.devices.len()];
    independent_paths[you_idx] = 1;

    let mut bfs_deque: VecDeque<usize> = VecDeque::new();
    bfs_deque.push_back(you_idx);

    let mut visited = vec![false; device_graph.devices.len()];

    while let Some(current_idx) = bfs_deque.pop_front() {
        let start = device_graph.connections.offsets[current_idx];
        let end = device_graph.connections.offsets[current_idx + 1];
        for &linked_node_idx in
            device_graph.connections.edges[start..end].iter()
        {
            independent_paths[linked_node_idx] +=
                independent_paths[current_idx];

            if !visited[linked_node_idx] {
                visited[linked_node_idx] = true;
                bfs_deque.push_back(linked_node_idx);
            }
        }
    }

    independent_paths[out_idx]
}

fn part_two(input: &str) -> u64 {
    // now we must find every path from svr to out that passes through both
    // dac and fft, in any order
    let device_graph = read_input(input);

    let (mut svr_idx, mut out_idx, mut dac_idx, mut fft_idx): (
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
    ) = (None, None, None, None);
    for (i, device) in device_graph.devices.iter().enumerate() {
        match device.as_slice() {
            b"svr" => svr_idx = Some(i),
            b"out" => out_idx = Some(i),
            b"dac" => dac_idx = Some(i),
            b"fft" => fft_idx = Some(i),
            _ => {}
        }
    }
    let svr_idx: usize = svr_idx.expect("Couldn't find svr idx");
    let out_idx: usize = out_idx.expect("Couldn't find out idx");
    let dac_idx: usize = dac_idx.expect("Couldn't find dac idx");
    let fft_idx: usize = fft_idx.expect("Couldn't find fft idx");

    let mut independent_paths: Vec<u64> = vec![0; device_graph.devices.len()];
    independent_paths[svr_idx] = 1;

    let mut bfs_deque: VecDeque<usize> = VecDeque::new();
    bfs_deque.push_back(svr_idx);

    // TODO DEFINITILY SWITCH TO A BFS AND KEEP TRACK OF WHETHER THE DAC AND
    // FFT HAVE BEEN SEEN IN THE CURRENT PATH

    let mut visited = vec![false; device_graph.devices.len()];

    while let Some(current_idx) = bfs_deque.pop_front() {
        let start = device_graph.connections.offsets[current_idx];
        let end = device_graph.connections.offsets[current_idx + 1];
        for &linked_node_idx in
            device_graph.connections.edges[start..end].iter()
        {
            independent_paths[linked_node_idx] +=
                independent_paths[current_idx];

            if !visited[linked_node_idx] {
                visited[linked_node_idx] = true;
                bfs_deque.push_back(linked_node_idx);
            }
        }
    }

    independent_paths[out_idx]
}

fn main() {
    match utils::read_input("day11.txt") {
        Ok(input) => {
            println!("Day 11 - Part 1: {}", part_one(input.as_str()));
            println!("Day 11 - Part 2: {}", part_two(input.as_str()));
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day11.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 5);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }
    const EXAMPLE_STRING_PART2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING_PART2), 2);
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
