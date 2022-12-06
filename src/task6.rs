use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

const MSG_SIZE: u8 = 14;

#[derive(Debug)]
struct Buf {
    buf: [char; MSG_SIZE as usize],
    len: u8,
    pos: u8,
}

impl std::fmt::Display for Buf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.len {
            s.push(self.buf[i as usize]);
        }
        write!(f, "{}, len: {}, pos: {}", s, self.len, self.pos)
    }
}

impl Buf {
    fn append(&mut self, c: char) {
        self.buf[self.pos as usize] = c;

        self.pos = (self.pos + 1) % MSG_SIZE;
        self.len = std::cmp::min(self.len + 1, MSG_SIZE);
    }

    fn is_all_different(&self) -> bool {
        if self.len < MSG_SIZE {
            return false;
        }

        for i in 0..self.len - 1 {
            let cur_c = self.buf[i as usize];
            for j in i + 1..self.len {
                if cur_c == self.buf[j as usize] {
                    return false;
                }
            }
        }

        true
    }

    fn new() -> Buf {
        Buf {
            buf: [' '; MSG_SIZE as usize],
            len: 0,
            pos: 0,
        }
    }
}

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut buf = Buf::new();
    let mut result = 0;

    for l in input.lines() {
        // only one line
        let line = l?;
        for (i, c) in line.chars().enumerate() {
            buf.append(c);
            if buf.is_all_different() {
                result = i + 1;
                break;
            }
        }
    }

    Ok(format!("{}", result))
}
