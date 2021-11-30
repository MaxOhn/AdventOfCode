use std::fmt;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

use util::Parse;

#[derive(Copy, Clone, Debug)]
enum Op {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn main() -> Result<(), OpParseError> {
    let input = std::fs::read_to_string("./input").unwrap();

    let ops = input
        .lines()
        .map(Op::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let start = Instant::now();
    let p1 = part1(&ops);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    assert_eq!(p1, 119);

    Ok(())
}

const W: usize = 50;
const H: usize = 6;

fn part1(ops: &[Op]) -> usize {
    let mut display = Display::default();

    for &op in ops {
        print!("\x1B[2J\x1B[1;1H");
        display.apply(op);
        println!("{}", display);
        thread::sleep(Duration::from_millis(100));
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", display);

    display.grid.iter().filter(|&&b| b).count()
}

struct Display {
    grid: [bool; W * H],
}

impl Display {
    fn apply(&mut self, op: Op) {
        match op {
            Op::Rect(w, h) => {
                for y in 0..h {
                    for x in 0..w {
                        self.grid[x + y * W] = true;
                    }
                }
            }
            Op::RotateRow(y, k) => self.grid[y * W..(y + 1) * W].rotate_right(k),
            Op::RotateCol(x, rot) => {
                for _ in 0..rot {
                    for y in 0..H {
                        self.grid.swap(x, x + y * W);
                    }
                }
            }
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            grid: [false; W * H],
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = self.grid.chunks_exact(W);

        if let Some(first) = rows.next() {
            for &e in first {
                write!(f, "{}", if e { '#' } else { '.' })?;
            }

            for row in rows {
                f.write_str("\n")?;

                for &e in row {
                    write!(f, "{}", if e { '#' } else { '.' })?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum OpParseError {
    InsufficientWords,
    UnknownOperation,
    InvalidRectDimension,
    MissingRotateValue,
}

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');

        let op = words.next().ok_or(OpParseError::InsufficientWords)?;
        let dim = words.next().ok_or(OpParseError::InsufficientWords)?;

        let op = match (op, dim) {
            ("rect", dim) => {
                let mut dims = dim.split('x').map(str::as_bytes).map(usize::parse);

                let w = dims.next().ok_or(OpParseError::InvalidRectDimension)?;
                let h = dims.next().ok_or(OpParseError::InvalidRectDimension)?;

                Op::Rect(w, h)
            }
            ("rotate", dim) => {
                let val = words
                    .next()
                    .ok_or(OpParseError::InsufficientWords)?
                    .split('=')
                    .nth(1)
                    .ok_or(OpParseError::MissingRotateValue)?;
                let val = usize::parse(val.as_bytes());

                let rotation = words.nth(1).ok_or(OpParseError::InsufficientWords)?;
                let rotation = usize::parse(rotation.as_bytes());

                match dim {
                    "row" => Op::RotateRow(val, rotation),
                    "column" => Op::RotateCol(val, rotation),
                    _ => return Err(OpParseError::UnknownOperation),
                }
            }
            _ => return Err(OpParseError::UnknownOperation),
        };

        Ok(op)
    }
}
