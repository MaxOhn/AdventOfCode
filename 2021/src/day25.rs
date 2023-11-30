use std::fmt;

use aoc_rust::Solution;
use eyre::Result;

use crate::util::Matrix;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();
    let mut grid = Vec::new();

    let line = lines.next().unwrap().trim_end().as_bytes();
    let w = line.len();
    grid.extend(line.iter().map(Field::from_byte));

    for line in lines {
        let row = line.trim_end().as_bytes().iter().map(Field::from_byte);

        grid.extend(row);
    }

    let mut matrix = Matrix::from_vec(grid, w);
    let mut next_matrix = Matrix::new(w, matrix.height());
    let mut step = 0;

    let w = matrix.width();
    let w_ = w - 1;
    let h = matrix.height();
    let h_ = h - 1;

    loop {
        step += 1;
        let mut no_move = true;

        for x in 0..w_ {
            for y in 0..h {
                if matrix[y][x] == Field::Right {
                    if matrix[y][x + 1] == Field::Free {
                        next_matrix[y][x] = Field::Free;
                        next_matrix[y][x + 1] = Field::Right;
                        no_move = false;
                    } else {
                        next_matrix[y][x] = Field::Right;
                    }
                }
            }
        }

        for y in 0..h {
            if matrix[y][w_] == Field::Right {
                if matrix[y][0] == Field::Free {
                    next_matrix[y][w_] = Field::Free;
                    next_matrix[y][0] = Field::Right;
                    no_move = false;
                } else {
                    next_matrix[y][w_] = Field::Right;
                }
            }
        }

        for (curr, next) in matrix.iter_mut().zip(next_matrix.iter()) {
            match (*curr, *next) {
                (_, Field::Right) => *curr = Field::Right,
                (Field::Right, Field::Free) => *curr = Field::Free,
                _ => {}
            }
        }

        for y in 0..h_ {
            for x in 0..w {
                if matrix[y][x] == Field::Down {
                    if matrix[y + 1][x] == Field::Free {
                        next_matrix[y][x] = Field::Free;
                        next_matrix[y + 1][x] = Field::Down;
                        no_move = false;
                    } else {
                        next_matrix[y][x] = Field::Down;
                    }
                }
            }
        }

        for x in 0..w {
            if matrix[h_][x] == Field::Down {
                if matrix[0][x] == Field::Free {
                    next_matrix[h_][x] = Field::Free;
                    next_matrix[0][x] = Field::Down;
                    no_move = false;
                } else {
                    next_matrix[h_][x] = Field::Down;
                }
            }
        }

        for (curr, next) in matrix.iter_mut().zip(next_matrix.iter_mut()) {
            match (*curr, *next) {
                (_, Field::Down) => *curr = Field::Down,
                (Field::Down, Field::Free) => *curr = Field::Free,
                _ => {}
            }
        }

        if no_move {
            return Ok(Solution::new().part1(step));
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Field {
    Right,
    Down,
    Free,
}

impl Field {
    fn from_byte(byte: &u8) -> Self {
        match byte {
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'.' => Self::Free,
            _ => unreachable!("{}", *byte as char),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Right => write!(f, ">"),
            Field::Down => write!(f, "v"),
            Field::Free => write!(f, "."),
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::Free
    }
}
