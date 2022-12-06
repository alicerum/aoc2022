use std::cmp::Ordering;
use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

enum Result {
    Lose,
    Draw,
    Win,
}

enum Figure {
    Rock,
    Paper,
    Scizzors,
}
impl PartialEq for Figure {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Figure {}

impl PartialOrd for Figure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fig_to_score(f: &Figure) -> u32 {
    match f {
        Figure::Rock => 1,
        Figure::Paper => 2,
        Figure::Scizzors => 3,
    }
}

fn result_and_him_to_fig(him: &Figure, r: Result) -> Figure {
    match (him, r) {
        (Figure::Rock, Result::Lose) => Figure::Scizzors,
        (Figure::Rock, Result::Draw) => Figure::Rock,
        (Figure::Rock, Result::Win) => Figure::Paper,
        (Figure::Paper, Result::Lose) => Figure::Rock,
        (Figure::Paper, Result::Draw) => Figure::Paper,
        (Figure::Paper, Result::Win) => Figure::Scizzors,
        (Figure::Scizzors, Result::Lose) => Figure::Paper,
        (Figure::Scizzors, Result::Draw) => Figure::Scizzors,
        (Figure::Scizzors, Result::Win) => Figure::Rock,
    }
}

impl Ord for Figure {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Figure::Rock, Figure::Rock) => Ordering::Equal,
            (Figure::Rock, Figure::Paper) => Ordering::Less,
            (Figure::Rock, Figure::Scizzors) => Ordering::Greater,
            (Figure::Paper, Figure::Rock) => Ordering::Greater,
            (Figure::Paper, Figure::Paper) => Ordering::Equal,
            (Figure::Paper, Figure::Scizzors) => Ordering::Less,
            (Figure::Scizzors, Figure::Rock) => Ordering::Less,
            (Figure::Scizzors, Figure::Paper) => Ordering::Greater,
            (Figure::Scizzors, Figure::Scizzors) => Ordering::Equal,
        }
    }
}

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut score: u32 = 0;

    for (i, l) in input.lines().enumerate() {
        let line = l?;

        let mut it = line.split(' ');
        let f1 = it.next().unwrap();
        let f2 = it.next().unwrap();

        let him = match f1 {
            "A" => Figure::Rock,
            "B" => Figure::Paper,
            "C" => Figure::Scizzors,
            x => {
                eprintln!("incorrect pattern for 'him': {}", x);
                std::process::exit(1);
            }
        };
        let result = match f2 {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            x => {
                eprintln!("incorrect pattern for 'result': {} on line {}", x, i);
                std::process::exit(1);
            }
        };

        let me = result_and_him_to_fig(&him, result);

        if him < me {
            score += 6;
        }
        if him == me {
            score += 3;
        }
        score += fig_to_score(&me);
    }

    Ok(format!("my score is {}", score))
}
