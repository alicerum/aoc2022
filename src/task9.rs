use std::collections::HashSet;
use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Rope {
    visited: HashSet<Pos>,

    tail: [Pos; 10],
}

enum Direction {
    R(i32),
    L(i32),
    U(i32),
    D(i32),
}

impl Direction {
    fn from(s: &str) -> Option<Self> {
        let mut it = s.split(' ');
        let dir = match it.next() {
            Some(d) => d,
            None => return None,
        };

        let count = match it.next() {
            Some(c) => c,
            None => return None,
        };

        let count = match count.parse::<i32>() {
            Err(_) => return None,
            Ok(i) => i,
        };

        match dir {
            "L" => Some(Direction::L(count)),
            "R" => Some(Direction::R(count)),
            "U" => Some(Direction::U(count)),
            "D" => Some(Direction::D(count)),
            _ => None,
        }
    }
}

impl Rope {
    fn new() -> Rope {
        let mut r = Rope {
            visited: HashSet::new(),
            tail: [Pos(0, 0); 10],
        };

        r.visited.insert(Pos(0, 0));
        r
    }

    fn move_tail_if_needed(&mut self) {
        for i in 1..10 {
            let mut dif_x = 0;
            let mut dif_y = 0;

            if !self.is_tail_touch_h(i) || !self.is_tail_touch_v(i) {
                if self.tail[i].0 != self.tail[i - 1].0 {
                    if (self.tail[i - 1].0 - self.tail[i].0) >= 0 {
                        dif_x = 1;
                    } else {
                        dif_x = -1;
                    }
                }

                if self.tail[i].1 != self.tail[i - 1].1 {
                    if (self.tail[i - 1].1 - self.tail[i].1) >= 0 {
                        dif_y = 1;
                    } else {
                        dif_y = -1;
                    }
                }
            }
            self.tail[i].0 += dif_x;
            self.tail[i].1 += dif_y;
        }

        self.visited.insert(self.tail[9]);
    }

    fn move_head_times(&mut self, p: Pos, c: i32) {
        for _ in 0..c {
            self.tail[0].0 += p.0;
            self.tail[0].1 += p.1;
            self.move_tail_if_needed();
        }
    }

    fn move_head(&mut self, d: Direction) {
        match d {
            Direction::L(c) => self.move_head_times(Pos(-1, 0), c),
            Direction::R(c) => self.move_head_times(Pos(1, 0), c),
            Direction::U(c) => self.move_head_times(Pos(0, 1), c),
            Direction::D(c) => self.move_head_times(Pos(0, -1), c),
        }
    }

    fn is_tail_touch_v(&self, i: usize) -> bool {
        (self.tail[i].1 - self.tail[i - 1].1).abs() <= 1
    }

    fn is_tail_touch_h(&self, i: usize) -> bool {
        (self.tail[i].0 - self.tail[i - 1].0).abs() <= 1
    }
}

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut rope = Rope::new();

    for l in input.lines() {
        let line = l?;

        if let Some(dir) = Direction::from(&line) {
            rope.move_head(dir);
        } else {
            eprintln!("weird input: {}", line);
        }
    }

    Ok(format!("{}", rope.visited.len()))
}
