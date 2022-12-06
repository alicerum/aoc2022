use std::cmp::Ordering;
use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut elves: Vec<i32> = Vec::new();
    let mut counter = 0;

    for l in input.lines() {
        let line = l?;
        if line.trim().len() == 0 {
            elves.push(counter);
            counter = 0;
            continue;
        }

        let calories = line.parse::<i32>().unwrap();
        counter += calories;
    }
    if counter > 0 {
        elves.push(counter);
    }

    elves.sort_by(|e1, e2| {
        if *e1 > *e2 {
            Ordering::Less
        } else if *e1 == *e2 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    Ok(format!(
        "{} {} {}, {}",
        elves[0],
        elves[1],
        elves[2],
        elves[0] + elves[1] + elves[2]
    ))
}
