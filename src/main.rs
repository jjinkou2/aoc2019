use aoc2019::{self, bail, day01, day02, Error, Reader};
use clap::Parser;
use std::{fs, io, path::PathBuf, process::exit};

/// Advent of code 2019
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// optional input file
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Day of advent of code
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error:{}", e);

        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("   - caused by {}", source);
            e = source;
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();
    let stdin = io::stdin();

    let input = match args.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            let input_number = io::BufReader::new(file);
            Reader::File(input_number)
        }
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        }
    };
    match args.day {
        1 => day01::run(input)?,
        2 => day02::run(input)?,
        n if n > 2 && n < 26 => bail!("Day {} not implemented", n),
        _ => bail!("Only day > 1 and < 26 allowed"),
    };
    Ok(())
}
