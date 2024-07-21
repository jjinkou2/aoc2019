use crate::error::Error;
use std::io::{self, BufRead};

pub fn run<R>(mut input: R) -> Result<(), Error>
where
    R: BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let mut reader = io::BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;
    let mut reader = io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;
    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
    F: Fn(usize) -> usize,
    R: BufRead,
{
    let mut buffer = String::new();

    let mut total = 0;
    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        let n = buffer.trim().parse::<usize>()?;

        let fuel = func(n);

        total += fuel;
        buffer.clear();
    }
    println!("{total}");
    Ok(())
}

pub fn part_one(n: usize) -> usize {
    (n / 3).saturating_sub(2)
}

pub fn part_two(mut n: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (n / 3).checked_sub(2) {
            Some(m) => m,
            None => break total,
        };
        total += m;
        n = m;
    }
}
