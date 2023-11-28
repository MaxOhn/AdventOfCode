use std::{fmt, str::FromStr, thread, time::Duration};

use aoc_rust::Solution;

pub fn run(input: &str) -> eyre::Result<Solution> {
    let ops = input
        .lines()
        .map(Op::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Solution::new().part1(part1(&ops)))
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

const W: usize = 50;
const H: usize = 6;

pub fn part1(ops: &[Op]) -> usize {
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
pub enum OpParseError {
    InsufficientWords,
    UnknownOperation,
    InvalidRectDimension,
    MissingRotateValue,
}

impl std::error::Error for OpParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for OpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');

        let op = words.next().ok_or(OpParseError::InsufficientWords)?;
        let dim = words.next().ok_or(OpParseError::InsufficientWords)?;

        let op = match (op, dim) {
            ("rect", dim) => {
                let mut dims = dim.split('x').map(|s| s.parse().unwrap());

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
                let val = val.parse().unwrap();

                let rotation = words.nth(1).ok_or(OpParseError::InsufficientWords)?;
                let rotation = rotation.parse().unwrap();

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
