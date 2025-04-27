use crate::computer::Computer;
use crate::error;
use crate::error::Error;
use std::io::BufRead;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: BufRead,
{
    let mut computer = Computer::new(input)?;
    let answer1 = computer.execute(12, 2)?;

    let mut answer2 = Err(error!("unable to find noun or verb"));
    for noun in 0..=99 {
        for verb in 0..=99 {
            if computer.execute(noun, verb)? == 19690720 {
                answer2 = Ok(100 * noun + verb);
            }
        }
    }

    Ok((format!("{answer1}"), format!("{}", answer2?)))
}
