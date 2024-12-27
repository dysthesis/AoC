use std::{
    fmt::Display, fs::read_to_string, io::Error, num::ParseIntError, ops::RangeInclusive,
    path::Path,
};

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
struct Report(Vec<i32>);

impl Report {
    /// Returns the number of elements to be removed for the report to be
    /// monotonically increasing. If it returns 0, then it already is
    /// monotonically increasing.
    fn is_increasing(&self) -> usize {
        let diff = self.differences();
        let descending: Vec<i32> = diff.into_iter().filter(|&x| x > 0).collect();
        dbg!(descending).len()
    }

    fn is_decreasing(&self) -> usize {
        let diff = self.differences();
        let ascending: Vec<i32> = diff.into_iter().filter(|&x| x < 0).collect();
        dbg!(ascending).len()
    }

    /// Check if a report is strictly increasing or strictly decreasing
    fn is_monotonic(&self) -> bool {
        (self.is_increasing() <= 1) || (self.is_decreasing() <= 1)
    }

    /// Find the differences between consecutive elements
    fn differences(&self) -> Vec<i32> {
        let Report(data) = self;

        data.iter()
            .zip(data.iter().skip(1))
            .map(|(&x, &y)| x - y)
            .collect()
    }

    fn too_large_differences(&self) -> usize {
        let violations: Vec<i32> = self
            .differences()
            .into_iter()
            .map(|x| x.abs())
            .filter(|x| !RangeInclusive::new(1, 3).contains(x))
            .collect();
        dbg!(violations).len()
    }

    /// The problem defines a safe report as fulfilling both of the following conditions:
    /// - The levels are either all increasing or all decreasing.
    /// - Any two adjacent levels differ by at least one and at most three.
    fn is_safe(&self) -> Option<bool> {
        Some(self.is_monotonic() && (self.too_large_differences() <= 1))
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

    let result: Vec<Vec<Result<i32, ParseIntError>>> = input
        .trim()
        // Split the string line by line
        .split("\n")
        .map(|str| -> Vec<Result<i32, ParseIntError>> {
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
                let (succ, err): (Vec<i32>, Vec<ParsingError>) =
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
    let safe_count: i32 = reports.iter().fold(0, |count, curr| {
        if curr.is_safe().expect("This report has no content!") {
            count + 1
        } else {
            count
        }
    });
    println!("There are {safe_count} safe reports!")
}
