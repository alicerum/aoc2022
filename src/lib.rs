use std::fmt;
use std::{error::Error, fmt::Display, fs::File, io::BufReader};

mod task1;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;
mod task7;
mod task8;
mod task9;

#[derive(Debug)]
struct AocError(String);

impl Error for AocError {}

impl Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn task_to_run(command: &str, input: BufReader<File>) -> Result<String, Box<dyn Error>> {
    match command {
        "task1" => task1::run(input),
        "task2" => task2::run(input),
        "task3" => task3::run(input),
        "task4" => task4::run(input),
        "task5" => task5::run(input),
        "task6" => task6::run(input),
        "task7" => task7::run(input),
        "task8" => task8::run(input),
        "task9" => task9::run(input),
        _ => Err(Box::new(AocError(format!("wrong command '{}'", command)))),
    }
}

pub fn run(command: &str, input: &str) -> Result<String, Box<dyn Error>> {
    let f = File::open(input)?;
    let br = BufReader::new(f);

    task_to_run(command, br)
}
