use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader};

pub fn run(input: BufReader<File>) -> std::result::Result<String, Box<dyn Error>> {
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let l = line?;

        let mut row: Vec<u8> = Vec::new();

        for c in l.chars() {
            let num = (c as u8) - ('0' as u8);
            row.push(num);
        }

        map.push(row);
    }

    let mut max_score = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, h) in row.iter().enumerate() {
            let mut score_left = 0;
            for other_j in (0..j).rev() {
                let other_h = row.get(other_j).unwrap();
                if other_h >= h {
                    score_left += 1;
                    break;
                }
                score_left += 1
            }
            let mut score_right = 0;
            for other_j in j + 1..row.len() {
                let other_h = row.get(other_j).unwrap();
                if other_h >= h {
                    score_right += 1;
                    break;
                }
                score_right += 1;
            }

            let mut score_up = 0;
            for other_i in (0..i).rev() {
                let other_h = map.get(other_i).unwrap().get(j).unwrap();
                if other_h >= h {
                    score_up += 1;
                    break;
                }
                score_up += 1;
            }
            let mut score_down = 0;
            for other_i in i + 1..map.len() {
                let other_h = map.get(other_i).unwrap().get(j).unwrap();
                if other_h >= h {
                    score_down += 1;
                    break;
                }
                score_down += 1;
            }

            let tree_score = score_down * score_up * score_left * score_right;
            if tree_score > max_score {
                max_score = tree_score;
            }
        }
    }
    Ok(format!("{}", max_score))
}
