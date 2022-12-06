use std::fmt;
use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug)]
struct RangeError(String);

impl Error for RangeError {}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn overlaps_with(&self, other: &Range) -> bool {
        (self.end >= other.start && self.start <= other.start)
            || (other.end >= self.start && other.start <= self.start)
    }

    fn from_line(s: &str) -> Result<Range, RangeError> {
        let rs: Vec<&str> = s.split("-").collect();

        if rs.len() != 2 {
            Err(RangeError(format!("wrong range string: {}", s)))
        } else {
            let start = match rs[0].parse::<u32>() {
                Err(_) => return Err(RangeError(format!("wrong number in range {}", s))),
                Ok(s) => s,
            };
            let end = match rs[1].parse::<u32>() {
                Err(_) => return Err(RangeError(format!("wrong number in range {}", s))),
                Ok(s) => s,
            };
            Ok(Range { start, end })
        }
    }
}

pub fn run(input: BufReader<File>) -> Result<String, Box<dyn Error>> {
    let mut score = 0;

    for (i, l) in input.lines().enumerate() {
        let line = l?;

        let ranges: Vec<&str> = line.split(',').collect();
        if ranges.len() != 2 {
            eprintln!("wrong ranges on line {}: {}", i, line);
            continue;
        }

        let r1 = match Range::from_line(ranges[0]) {
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
            Ok(r) => r,
        };

        let r2 = match Range::from_line(ranges[1]) {
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
            Ok(r) => r,
        };

        if r1.overlaps_with(&r2) {
            score += 1;
        }
    }

    Ok(format!("ranges: {}", score))
}
