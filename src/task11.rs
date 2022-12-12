use std::cmp::Ordering;
use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

use regex::Regex;

struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: fn(usize) -> usize,
        test: usize,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspected: 0,
        }
    }
}

fn main_monkes(monkeys: &mut Vec<Monkey>) {
    monkeys.push(Monkey::new(vec![64], |x| x * 7, 13, 1, 3));
    monkeys.push(Monkey::new(vec![60, 84, 84, 65], |x| x + 7, 19, 2, 7));
    monkeys.push(Monkey::new(
        vec![52, 67, 74, 88, 51, 61],
        |x| x * 3,
        5,
        5,
        7,
    ));
    monkeys.push(Monkey::new(vec![67, 72], |x| x + 3, 2, 1, 2));
    monkeys.push(Monkey::new(
        vec![80, 79, 58, 77, 68, 74, 98, 64],
        |x| x.pow(2),
        17,
        6,
        0,
    ));
    monkeys.push(Monkey::new(vec![62, 53, 61, 89, 86], |x| x + 8, 11, 4, 6));
    monkeys.push(Monkey::new(vec![86, 89, 82], |x| x + 2, 7, 3, 0));
    monkeys.push(Monkey::new(
        vec![92, 81, 70, 96, 69, 84, 83],
        |x| x + 4,
        3,
        4,
        5,
    ));
}

fn test_monkeys(monkeys: &mut Vec<Monkey>) {
    monkeys.push(Monkey::new(vec![79, 98], |x| x * 19, 23, 2, 3));
    monkeys.push(Monkey::new(vec![54, 65, 75, 74], |x| x + 6, 19, 2, 0));
    monkeys.push(Monkey::new(vec![79, 60, 97], |x| x.pow(2), 13, 1, 3));
    monkeys.push(Monkey::new(vec![74], |x| x + 3, 17, 0, 1));
}

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut monkeys = Vec::new();
    main_monkes(&mut monkeys);
    // test_monkeys(&mut monkeys);

    let mut all = 1;
    for m in &monkeys {
        all *= m.test;
    }

    for i in 0..10000 {
        println!("{}", i);
        for m in 0..monkeys.len() {
            for item in 0..monkeys[m].items.len() {
                let item = monkeys[m].items[item].clone();
                let mut throw_to = 0;
                let new_worry = (monkeys[m].operation)(item);
                if new_worry % monkeys[m].test == 0 {
                    throw_to = monkeys[m].if_true;
                } else {
                    throw_to = monkeys[m].if_false;
                }
                monkeys[throw_to].items.push(new_worry % all);
                monkeys[m].inspected += 1;
            }

            monkeys[m].items.clear();
        }
    }

    monkeys.sort_by(|m1, m2| {
        if m1.inspected < m2.inspected {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    Ok(format!(
        "{}, {:?}",
        monkeys[0].inspected * monkeys[1].inspected,
        monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>()
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_command() {}
}
