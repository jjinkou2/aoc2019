pub mod day01;
pub mod day02;
pub mod day03;

#[macro_use]
mod macros;

pub use self::error::Error;
pub use self::reader::Reader;

mod error {
    use std::{fmt, io};

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        Io(io::Error),
        ParseInt(std::num::ParseIntError),
    }

    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Self::Io(value)
        }
    }
    impl From<std::num::ParseIntError> for Error {
        fn from(value: std::num::ParseIntError) -> Self {
            Self::ParseInt(value)
        }
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(s) => write!(f, "{}", s),
                Self::ParseInt(s) => write!(f, "{}", s),
            }
        }
    }

    impl std::error::Error for Error {}
}

mod reader {
    use std::{
        fs,
        io::{self, BufRead, Read},
    };
    pub enum Reader<'a> {
        File(io::BufReader<fs::File>),
        Stdin(io::StdinLock<'a>),
    }
    impl<'a> Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self {
                Self::File(file) => file.read(buf),
                Self::Stdin(guard) => guard.read(buf),
            }
        }
    }
    impl<'a> BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            match self {
                Self::File(file) => file.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf(),
            }
        }
        fn consume(&mut self, amt: usize) {
            match self {
                Self::File(file) => file.consume(amt),
                Self::Stdin(guard) => guard.consume(amt),
            }
        }
    }
}
