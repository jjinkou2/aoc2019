use crate::error::Error;
use crate::{bail, error};
use std::io::BufRead;
pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: BufRead,
{
    let (low, high) = read_input(input)?;
    let (mut answer1, mut answer2) = (0, 0);

    for n in low..=high {
        let is_valid = is_valid(n)?;
        if is_valid.0 {
            answer1 += 1;
        }
        if is_valid.1 {
            answer2 += 1;
        }
    }
    Ok((answer1.to_string(), answer2.to_string()))
}

fn is_valid(n: usize) -> Result<(bool, bool), Error> {
    let mut is_valid = (false, false);
    let mut min = 0;
    let mut previous_digit = PreviousDigit::None;
    for (i, digit) in parse_digits(n)?.iter().enumerate() {
        let digit = *digit;
        if digit < min {
            return Ok((false, false));
        }

        min = digit;

        match previous_digit {
            PreviousDigit::None => previous_digit = PreviousDigit::One(digit),
            PreviousDigit::One(d) => {
                if digit == d {
                    is_valid.0 = true;
                    if i == 5 {
                        is_valid.1 = true
                    };
                    previous_digit = PreviousDigit::Two(digit);
                } else {
                    previous_digit = PreviousDigit::One(digit);
                }
            }
            PreviousDigit::Two(d) => {
                if digit == d {
                    previous_digit = PreviousDigit::ThreeeOrMore(digit);
                } else {
                    is_valid.1 = true;
                    previous_digit = PreviousDigit::One(digit);
                }
            }
            PreviousDigit::ThreeeOrMore(d) => {
                if digit == d {
                } else {
                    previous_digit = PreviousDigit::One(digit);
                }
            }
        }
    }
    Ok(is_valid)
}

enum PreviousDigit {
    None,
    One(u8),
    Two(u8),
    ThreeeOrMore(u8),
}
fn parse_digits(n: usize) -> Result<[u8; 6], Error> {
    if !(100_000..=999_999).contains(&n) {
        bail!("invalid digits")
    }
    let mut output = [0u8; 6];
    for i in 0..6 {
        let foio = n / 10usize.pow(i as u32);
        let bar = (foio / 10) * 10;
        output[5 - i] = (foio - bar) as u8;
    }
    Ok(output)
}

fn read_input<R>(mut reader: R) -> Result<(usize, usize), Error>
where
    R: BufRead,
{
    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    let parse = |s: &str| s.trim().parse::<usize>();
    let error = || error!("input invalid");

    let mut iter = s.split('-');

    let low = iter.next().map(parse).ok_or_else(error)??;
    let high = iter.next().map(parse).ok_or_else(error)??;

    if iter.next().is_some() {
        bail!("input invalid");
    }
    Ok((low, high))
}
mod tests {

    use super::*;

    #[test]
    fn test_04() {
        let test_cases = &[
            (111111, true, false),
            (223450, false, false),
            (123789, false, false),
            (112233, true, true),
            (123444, true, false),
            (111122, true, true),
        ];

        for (n, expected1, expected2) in test_cases {
            let (actual1, actual2) = is_valid(*n).unwrap();
            assert_eq!(actual1, *expected1);
            assert_eq!(actual2, *expected2);
        }
    }
}
