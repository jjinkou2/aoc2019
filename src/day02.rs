use crate::error::Error;
use std::io::BufRead;
pub fn run<R>(mut _input: R) -> Result<(), Error>
where
    R: BufRead,
{
    Ok(())
}
