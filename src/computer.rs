use crate::Error;
use std::io::BufRead;

pub struct Computer {
    rom: Vec<i64>,
    ram: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    pc: u64,
}

impl Computer {
    pub fn new<R>(mut rom_reader: R) -> Result<Self, Error>
    where
        R: BufRead,
    {
        let mut buffer = String::new();
        rom_reader.read_to_string(&mut buffer)?;

        let rom = buffer
            .trim()
            .split(',')
            .map(|s| Ok(s.parse::<i64>()?))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self {
            rom,
            ram: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            pc: 0,
        })
    }

    pub fn execute(
        &mut self,
        input: Option<Vec<i64>>,
        noun_and_verb: Option<(i64, i64)>,
    ) -> Result<i64, Error> {
        //reset
        self.ram.clone_from(&self.rom);
        self.pc = 0;
        self.output = Vec::new();

        // set input
        if let Some(input) = input {
            self.input = input;
        }
        if let Some((noun, verb)) = noun_and_verb {
            self.ram[1] = noun;
            self.ram[2] = verb;
        }

        let instruction = self.read_instruction()?;
        self.execute_instruction(instruction)?;
        Ok(self.ram[0])
    }

    fn read_instruction(&mut self) -> Result<Instruction, Error> {
        let mut n = self.ram[self.pc as usize];
        let opcode = n % 100;
        n /= 100;

        let modes = Modes(n as u64);
        todo!()
    }
    fn execute_instruction(&mut self, inst: Instruction) -> Result<(), Error> {
        todo!()
    }
    #[allow(dead_code)]
    pub fn ram(&self) -> &[i64] {
        &self.ram
    }
}

enum Mode {
    Pointer,
    Immediate,
}

struct Modes(u64);

impl Iterator for Modes {
    type Item = Result<Mode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let m = match Mode::try_from(self.0 % 10) {
            Ok(mode) => mode,
            Err(e) => Some(Err(e)),
        };
        Ok(m)
    }
}
trait Memory {
    fn read(&self, ptr: u64) -> Result<i64, Error>;
    fn write(&mut self, ptr: u64, val: i64) -> Result<(), Error>;
}

impl Memory for Vec<i64> {
    fn read(&self, ptr: u64) -> Result<i64, Error> {
        let value = *self
            .get(ptr as usize)
            .ok_or_else(|| error!("Unable to get Memory at {}", ptr))?;
        Ok(value)
    }

    fn write(&mut self, ptr: u64, val: i64) -> Result<(), Error> {
        let reference = self
            .get_mut(ptr as usize)
            .ok_or_else(|| error!("Unable to set memory at {}", ptr))?;
        *reference = val;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Add { a: i64, b: i64, w: u64 },
    Multiply { a: i64, b: i64, w: u64 },
    Input { w: u64 },
    Output { a: i64 },
    Halt,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io;

    #[test]
    fn test_02() {
        let test_cases = &[
            //(input,noun, verb,  expexted)
            ("1,0,0,0,99", 0, 0, "2,0,0,0,99"),
            ("2,3,0,3,99", 3, 0, "2,3,0,6,99"),
            ("2,4,4,5,99,0", 4, 4, "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", 1, 1, "30,1,1,4,2,5,6,0,99"),
        ];

        for (input, noun, verb, expected) in test_cases {
            let reader = io::BufReader::new(input.as_bytes());
            let mut computer = Computer::new(reader).unwrap();
            let _ = computer.execute(*noun, *verb);

            let expected_ram: Vec<i64> = expected
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            assert_eq!(&expected_ram[..], computer.ram());
        }
    }
}
