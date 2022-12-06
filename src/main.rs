use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Opts {
    #[clap()]
    command: String,

    #[clap(short, long)]
    input: String,
}

fn main() {
    let o = Opts::parse();

    match aoc2022::run(&o.command, &o.input) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
