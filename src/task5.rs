use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

use regex::Regex;

fn line_to_stacks(stacks: &mut Vec<Vec<char>>, l: &str) {
    let bb = l.as_bytes();

    for (i, _) in l.chars().enumerate() {
        if i % 4 == 0 {
            let stack = match stacks.get_mut(i / 4) {
                Some(s) => s,
                None => {
                    stacks.push(Vec::new());
                    stacks.get_mut(i / 4).unwrap()
                }
            };

            if i == l.len() - 1 {
                return;
            }

            if bb[i + 1] as char != ' ' {
                stack.push(bb[i + 1] as char);
            }
        }
    }
}

pub fn run(input: BufReader<File>) -> Result<String, Box<dyn Error>> {
    let mut reading_stack = true;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for l in input.lines() {
        let line = l?;

        if reading_stack {
            if line.starts_with(" 1") {
                reading_stack = false;
            } else {
                line_to_stacks(&mut stacks, &line);
            }
            continue;
        }

        if line.len() == 0 {
            continue;
        }

        for cap in re.captures_iter(&line) {
            let amount = cap[1].parse::<u32>().unwrap();
            let from = cap[2].parse::<u32>().unwrap() - 1;
            let to = cap[3].parse::<u32>().unwrap() - 1;

            for i in 0..amount {
                let i = amount - i - 1;
                let c = stacks[from as usize].remove(i as usize);
                stacks[to as usize].insert(0, c);
            }
        }
    }

    let mut result = String::new();

    for s in stacks {
        if s.len() == 0 {
            result.push(' ');
        } else {
            result.push(s[0]);
        }
    }

    Ok(result)
}
