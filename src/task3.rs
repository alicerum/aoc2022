use std::collections::HashSet;
use std::io::BufRead;
use std::rc::Rc;
use std::{error::Error, fs::File, io::BufReader};

fn char_to_score(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        (c as i32) - ('a' as i32) + 1
    } else if c >= 'A' && c <= 'Z' {
        (c as i32) - ('A' as i32) + 27
    } else {
        0
    }
}

pub fn run(input: BufReader<File>) -> Result<String, Box<dyn Error>> {
    let mut score = 0;
    let mut elves: Vec<Rc<String>> = Vec::new();
    let mut counter = 0;

    for (i, l) in input.lines().enumerate() {
        let line = l?;

        elves.push(Rc::new(line.trim().to_string()));
        counter += 1;

        if counter == 3 {
            if line.len() == 0 {
                continue;
            }

            let mut second = HashSet::new();
            for c in elves[1].chars() {
                second.insert(c);
            }

            let mut third = HashSet::new();
            for c in elves[2].chars() {
                third.insert(c);
            }

            let mut found = false;
            for c in elves[0].chars() {
                if second.contains(&c) && third.contains(&c) {
                    found = true;
                    score += char_to_score(c);
                    break;
                }
            }

            if !found {
                eprintln!("could not find common char on line {}", i);
                std::process::exit(1);
            }

            counter = 0;
            elves.clear();
        }
    }

    Ok(format!("score: {}", score))
}
