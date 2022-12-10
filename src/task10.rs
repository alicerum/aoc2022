use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

#[derive(PartialEq, Eq, Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn from_str(s: &str) -> Option<Self> {
        if s == "noop" {
            Some(Command::Noop)
        } else if s.starts_with("addx ") {
            if let Ok(amount) = s[5..s.len()].parse::<i32>() {
                return Some(Command::Addx(amount));
            }
            None
        } else {
            return None;
        }
    }
}

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut x = 1;

    let mut input = input.lines();

    let mut next_addx = false;
    let mut next_addx_value = 0;

    for cycle in 0..240 {
        let position = (cycle) % 40;

        if position == x - 1 || position == x || position == x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if position == 39 {
            println!();
        }

        if !next_addx {
            let current_command = Command::from_str(&input.next().unwrap().unwrap()).unwrap();
            match current_command {
                Command::Noop => {}
                Command::Addx(c) => {
                    next_addx = true;
                    next_addx_value = c;
                }
            }
        } else {
            x += next_addx_value;
            next_addx = false;
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_command() {
        let s = "addx -51";

        assert_eq!(Command::from_str(s).unwrap(), Command::Addx(-51));
    }
}
