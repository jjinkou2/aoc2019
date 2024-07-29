use crate::error::Error;
use crate::{bail, error};
use std::convert::TryFrom;
use std::{collections::HashMap, io::BufRead};

type State = HashMap<Point, [bool; 2]>;

const ORIGIN: Point = Point(0, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: BufRead,
{
    let mut state: State = HashMap::new();
    for (id, res) in input.lines().enumerate() {
        assert!(id < 2);
        let line = res?;

        let mut point: Point = ORIGIN;

        for s in line.trim().split(',').map(|w| w.trim()) {
            let instruction = s.parse::<Instruction>()?;
            point = process_instruction(id, point, instruction, &mut state);
        }
    }

    let answer =
        state
            .iter()
            .filter(|(_, v)| v[0] && v[1])
            .fold(u32::MAX, |mut min, (point, _)| {
                let dist = manhattan_distance(*point, ORIGIN);
                if dist < min {
                    min = dist;
                }
                min
            });
    Ok((format!("{answer}"), "bar".to_string()))
}

fn process_instruction(
    id: usize,
    origin: Point,
    instruction: Instruction,
    state: &mut State,
) -> Point {
    let (i, j) = match instruction.dir {
        Direction::U => (0, 1),
        Direction::D => (0, -1),
        Direction::R => (1, 0),
        Direction::L => (-1, 0),
    };

    let mut destination = origin;
    for n in 1..=instruction.dist {
        destination = Point(origin.0 + i * n as i32, origin.1 + j * n as i32);
        let value = state.entry(destination).or_insert([false, false]);
        value[id] = true;
    }
    println!("{:?}", state);
    destination
}

fn manhattan_distance(a: Point, b: Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

struct Instruction {
    dir: Direction,
    dist: u32,
}

enum Direction {
    U,
    R,
    D,
    L,
}
impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let dir = Direction::try_from(bytes[0] as char)?;
        let dist = atoi::atoi::<u32>(&bytes[1..]).ok_or_else(|| error!("unable to parse {s}"))?;
        Ok(Instruction { dir, dist })
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::U),
            'R' => Ok(Self::R),
            'D' => Ok(Self::D),
            'L' => Ok(Self::L),
            _ => bail!("unable to parse {value} into direction"),
        }
    }
}
#[cfg(test)]
mod tests {

    //use std::io;
    //use super::*;

    #[test]
    fn test_03() {
        let test_cases = [
            //(input,  expexted)
            ("R8,U5,L5,D3", ""),
            ("", ""),
            ("", ""),
            ("", ""),
        ];
    }
}
