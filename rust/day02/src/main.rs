use std::{fmt::Display, fs::read_to_string, io::Error, num::ParseIntError, path::Path};

use itertools::{Either, Itertools};

#[derive(Debug)]
enum ParsingError {
    ReadError(String),
    ParseError(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::ReadError(err) => write!(f, "Failed to read file: {err}"),
            ParsingError::ParseError(err) => write!(f, "Failed to parse to int: {err}"),
        }
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(value: ParseIntError) -> Self {
        ParsingError::ParseError(value.to_string())
    }
}

impl From<Error> for ParsingError {
    fn from(value: Error) -> Self {
        ParsingError::ReadError(value.to_string())
    }
}

#[derive(Clone)]
struct Report(Vec<u16>);

impl Report {
    fn is_increasing(&self) -> bool {
        let Report(data) = self;
        let mut sorted = data.clone();
        sorted.sort();
        data.clone() == sorted
    }

    fn is_decreasing(&self) -> bool {
        let Report(data) = self;
        let mut sorted = data.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        data.clone() == sorted
    }

    /// Check if a report is strictly increasing or strictly decreasing
    fn is_monotonic(&self) -> bool {
        self.is_increasing() || self.is_decreasing()
    }

    /// Find the minimum and maximum difference between two consecutive elements
    fn difference_range(&self) -> (Option<u16>, Option<u16>) {
        let Report(data) = self;

        let diff: Vec<u16> = data
            .iter()
            .zip(data.iter().skip(1))
            .map(|(&x, &y)| u16::abs_diff(x, y))
            .collect();
        (diff.iter().min().cloned(), diff.iter().max().cloned())
    }

    /// The problem defines a safe report as fulfilling both of the following conditions:
    /// - The levels are either all increasing or all decreasing.
    /// - Any two adjacent levels differ by at least one and at most three.
    fn is_safe(&self) -> Option<bool> {
        let (min, max) = self.difference_range();
        let min = match min {
            Some(value) => value,
            None => return None,
        };
        let max = match max {
            Some(value) => value,
            None => return None,
        };

        Some(self.is_monotonic() && (min >= 1) && (max <= 3))
    }

    fn try_remove(&self) -> bool {
        let Report(data) = self;
        let safe: Vec<bool> = (0..data.len())
            .filter_map(|i| {
                let mut removed = data.clone();
                removed.remove(i);
                Report(removed).is_safe()
            })
            .filter(|&x| x)
            .collect();
        !safe.is_empty()
    }
}

/// Parse the input for this problem
///
/// * `path`: The path to the input file
fn parse_input(path: &Path) -> Result<Vec<Report>, Vec<ParsingError>> {
    let input = match read_to_string(path) {
        Ok(res) => res,
        Err(e) => return Err(vec![ParsingError::from(e)]),
    };

    let result: Vec<Vec<Result<u16, ParseIntError>>> = input
        .trim()
        // Split the string line by line
        .split("\n")
        .map(|str| -> Vec<Result<u16, ParseIntError>> {
            // Split the string by column...
            str.split(" ")
                // ...and try to parse it.
                .map(|x| x.parse())
                .collect()
        })
        .collect();

    // Separate the results from the errors
    let (parsed, errors): (Vec<Report>, Vec<ParsingError>) =
        result
            .into_iter()
            .fold((Vec::new(), Vec::new()), |mut acc, curr| {
                let (succ, err): (Vec<u16>, Vec<ParsingError>) =
                    // Put any successess into `succ` and errors into `err`
                    curr.into_iter().partition_map(|x| match x {
                        Ok(res) => Either::Left(res),
                        Err(err) => Either::Right(ParsingError::from(err)),
                    });

                // We want to preserve the structure for the successess...
                acc.0.push(Report(succ));
                // ...but we want to flatten errors to make it easier to display.
                acc.1.extend(err);
                acc
            });

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(parsed)
}

fn main() {
    let path = Path::new("/home/demiurge/Documents/Projects/AoC/rust/day02/input");
    let reports = match parse_input(path) {
        Ok(result) => result,
        Err(errors) => panic!("Failed to parse data: {:?}", errors),
    };
    let (p1, p2): (u16, u16) = reports.iter().fold((0, 0), |count, curr| {
        if curr.is_safe().expect("the report to have content") {
            (count.0 + 1, count.1 + 1)
        } else if curr.try_remove() {
            (count.0, count.1 + 1)
        } else {
            count
        }
    });
    println!("There are {p1} safe reports for part 1 and {p2} for part 2!");
}
