use core::panic;
use std::{fs::read_to_string, io::Result, path::Path};

/// Parse the input for this problem
///
/// * `path`: The path to the input file
fn parse_input(path: &Path) -> Result<(Vec<u32>, Vec<u32>)> {
    let input = read_to_string(path)?;
    let result = input
        .trim()
        .split("\n")
        .map(|str| -> Vec<&str> { str.split("   ").take(2).collect() })
        .fold((Vec::new(), Vec::new()), |mut acc, curr| {
            acc.0
                .push(curr.first().unwrap().to_owned().parse().unwrap());
            acc.1.push(curr.get(1).unwrap().to_owned().parse().unwrap());
            acc
        });
    Ok(result)
}
fn main() {
    let path = Path::new("/home/demiurge/Documents/Projects/AoC/rust/day01/input");
    let (mut left, mut right) = match parse_input(path) {
        Ok(res) => res,
        Err(e) => panic!("Cannot parse input: {e}"),
    };

    left.sort();
    right.sort();

    let distances: Vec<u32> = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| u32::abs_diff(left, right))
        .collect();

    let result: u32 = distances.iter().sum();

    println!("The result is {result}")
}
