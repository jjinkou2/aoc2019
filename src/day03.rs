use crate::error::Error;
use std::io::BufRead;

pub fn run<R>(_input: R) -> Result<(String, String), Error>
where
    R: BufRead,
{
    unimplemented!();
}
#[cfg(test)]
mod tests {

    //use std::io;
    //use super::*;

    #[test]
    fn test_03() {}
}
