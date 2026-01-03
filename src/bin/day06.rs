use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    // There are rows of numbers stacked on eachother, and the last row is the
    // opretaion that has to be applied to the numbers in that column
    // Every row has the same length
    let mut lines = input.lines();
    // Count numbers in first row
    let cols = lines
        .next()
        .map(|l| l.split_whitespace().count())
        .expect("Zero length first row?");
    let rows = lines.count() + 1; // add back the skipped first line

    // numerical part only values
    let n_cols = cols;
    let n_rows = rows - 1;

    // transposed matrix values
    let t_cols = n_rows;
    let t_rows = n_cols;

    println!("Rows: {:?}, Cols: {:?}", rows, cols);

    // Transposed matrix of the numbers, so that every line contains the numbers
    // in the original column
    let mut numbers: Vec<u64> = vec![0; t_rows * t_cols];
    for (i, l) in input.lines().take(n_rows).enumerate() {
        for (j, c) in l.split_whitespace().enumerate() {
            numbers[j * (t_cols) + i] = c.parse().expect("Not a number");
        }
    }

    let mut acc = 0;

    for l in input.lines().skip(n_rows) {
        for (i, op) in l.split_whitespace().enumerate() {
            // println!("{:?}, {:?}", i, op);
            match op {
                "*" => {
                    let mut small_acc = numbers[i * t_cols];
                    for j in 1..t_cols {
                        small_acc *= numbers[i * t_cols + j];
                    }
                    acc += small_acc;
                }
                "+" => {
                    let mut small_acc = numbers[i * t_cols];
                    for j in 1..t_cols {
                        small_acc += numbers[i * t_cols + j];
                    }
                    acc += small_acc;
                }
                _ => {}
            }
        }
    }

    acc
}

// NOT GONNA LIE THIS IS WAYYY OVERENGINEERED
// The simplest way is to take the transpose of the matrix of all the characters
// in the input string, I only thought of that after starting this madness, so
// I had to finish it...
fn part_two(input: &str) -> u64 {
    let rows = input.lines().count();
    let n_rows = rows - 1;

    // Get the digit amount in each column
    let mut digit_amounts: Vec<usize> = Vec::new();
    let mut last_digit_amount = 1;
    let mut cols = 1;
    let mut largest_digit_amount: usize = 0;
    for l in input.lines().skip(n_rows) {
        // last line
        for c in l.bytes().skip(1) {
            if c == b'+' || c == b'*' {
                if last_digit_amount - 1 > largest_digit_amount {
                    largest_digit_amount = last_digit_amount - 1;
                }
                digit_amounts.push(last_digit_amount - 1);
                last_digit_amount = 1;
                cols += 1;
            } else {
                last_digit_amount += 1;
            }
        }
        digit_amounts.push(last_digit_amount);
    }
    // println!("{:?}", digit_amounts);

    let n_cols = cols;

    let t_cols = n_rows;
    let t_rows = n_cols;

    let mut numbers: Vec<u64> = vec![0; t_rows * largest_digit_amount];
    for l in input.lines().take(n_rows) {
        let mut col_idx = 0;
        let mut digit_idx: usize = 0;
        for c in l.bytes() {
            if digit_idx >= digit_amounts[col_idx] {
                digit_idx = 0;
                col_idx += 1;
                continue;
            }
            match c {
                b'0'..=b'9' => {
                    numbers[col_idx * (t_cols) + digit_idx] *= 10;
                    numbers[col_idx * (t_cols) + digit_idx] +=
                        (c - b'0') as u64;
                }
                b' ' => {}
                _ => {}
            }
            digit_idx += 1;
        }
    }
    // println!("{:?}", numbers);

    let mut acc = 0;

    for l in input.lines().skip(n_rows) {
        for (i, op) in l.split_whitespace().enumerate() {
            // println!("{:?}, {:?}", i, op);
            match op {
                "*" => {
                    let mut small_acc = if numbers[i * t_cols] > 0 {
                        numbers[i * t_cols]
                    } else {
                        1
                    };
                    for j in 1..largest_digit_amount {
                        small_acc *= if numbers[i * t_cols + j] > 0 {
                            numbers[i * t_cols + j]
                        } else {
                            1
                        };
                    }
                    acc += small_acc;
                }
                "+" => {
                    let mut small_acc = numbers[i * t_cols];
                    for j in 1..largest_digit_amount {
                        small_acc += numbers[i * t_cols + j];
                    }
                    acc += small_acc;
                }
                _ => {}
            }
        }
    }

    acc
}

fn part_two_alt(input: &str) -> u64 {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    // original matrix sizes
    let rows = lines.len();
    let cols = lines[0].len();

    // numerical matrix sizes
    let n_rows = rows - 1;
    let n_cols = cols;

    // transposed matrix sizes
    let t_cols = n_rows;
    let t_rows = n_cols;

    let mut transposed_nums: Vec<u8> = vec![0; n_rows * n_cols];
    for i in 0..n_rows {
        for j in 0..n_cols {
            transposed_nums[j * t_cols + i] = lines[i][j];
        }
    }

    let mut ops: Vec<u8> = Vec::new();
    for &c in lines[n_rows].iter() {
        if c != b' ' {
            ops.push(c);
        }
    }

    assert!(!ops.is_empty(), "NO OPS?");

    let mut acc: u64 = 0;
    let mut small_acc: u64 = if ops[0] == b'+' { 0 } else { 1 };
    let mut op_idx: usize = 0;
    for i in 0..t_rows {
        // use unchecked, cause its fineeeeeeeeeeeeeeeeeeeeeeeee
        if let Some(s) = unsafe {
            str::from_utf8_unchecked(
                &transposed_nums[i * t_cols..i * t_cols + t_cols],
            )
            .split_whitespace()
            .next()
        } {
            let num: u64 = s.parse().unwrap();
            match ops[op_idx] {
                b'+' => {
                    small_acc += num;
                }
                b'*' => {
                    small_acc *= num;
                }
                _ => {}
            }
        } else {
            acc += small_acc;
            // increment op idx and check the operator type
            op_idx += 1;
            small_acc = if ops[op_idx] == b'+' { 0 } else { 1 };
        }
        #[cfg(debug_assertions)]
        {
            println!(
                "{:?}",
                str::from_utf8(
                    &transposed_nums[i * t_cols..i * t_cols + t_cols]
                )
                .unwrap()
            );
        }
    }
    acc += small_acc;

    acc
}

fn main() {
    match utils::read_input("day06.txt") {
        Ok(input) => {
            println!("Day 06 - Part 1: {}", part_one(input.as_str()));
            println!("Day 06 - Part 2: {}", part_two(input.as_str()));
            println!(
                "Day 06 - Part 2 alternative solution: {}",
                part_two_alt(input.as_str())
            );
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            eprintln!("Make sure 'inputs/day06.txt' exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    #[test]
    fn test_part1_example() {
        assert_eq!(part_one(EXAMPLE_STRING), 4277556);
    }

    // #[test]
    // fn test_part1_simple() {
    //     assert_eq!(part_one("987654321111111"), 98);
    // }

    #[test]
    fn test_part2_example() {
        assert_eq!(part_two(EXAMPLE_STRING), 3263827);
    }

    #[test]
    fn test_part2_alt_example() {
        assert_eq!(part_two_alt(EXAMPLE_STRING), 3263827);
    }
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
