use std::{
    iter,
    ops::{Add, Mul, Rem, Sub},
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let (board, path) = Board::parse(input)?;

    let start = board
        .tiles
        .iter()
        .position(|&tile| tile == Tile::Open)
        .map(|idx| Pos::new(idx as i32 % board.width, idx as i32 / board.width))
        .wrap_err("missing open tile")?;

    let p1 = solve(&board, path, start, Board::wrap_grid)?;
    let p2 = solve(&board, path, start, Board::wrap_cube)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

type WrapFn = fn(&Board, &mut Pos, &mut Direction, &mut Pos) -> Wrap;

fn solve(board: &Board, mut path: Path, start: Pos, wrap: WrapFn) -> Result<i32> {
    let mut pos = start;
    let mut direction = RIGHT;

    while let Some(next) = path.next() {
        match next? {
            Move::Distance(n) => board.steps(&mut pos, n, &mut direction, wrap),
            Move::Turn(Turn::Left) => direction = direction.turn_left(),
            Move::Turn(Turn::Right) => direction = direction.turn_right(),
        }
    }

    Ok(1000 * (pos.y + 1) + 4 * (pos.x + 1) + direction.0 as i32)
}

struct Board {
    tiles: Box<[Tile]>,
    width: i32,
}

enum Wrap {
    Continue,
    Stop,
}

impl Board {
    fn steps(&self, pos: &mut Pos, count: i16, direction: &mut Direction, wrap: WrapFn) {
        let mut delta = direction.delta();

        for _ in 0..count {
            let next = *pos + delta;

            match self.get(next) {
                Tile::Wall => return,
                Tile::Open => *pos = next,
                Tile::Closed => match (wrap)(self, pos, direction, &mut delta) {
                    Wrap::Continue => {}
                    Wrap::Stop => break,
                },
            }
        }
    }

    fn wrap_grid(&self, pos: &mut Pos, _: &mut Direction, delta: &mut Pos) -> Wrap {
        let mut next = *pos - *delta;

        while !matches!(self.get(next), Tile::Closed) {
            next = next - *delta;
        }

        next = next + *delta;

        match self.get(next) {
            Tile::Wall => Wrap::Stop,
            Tile::Open => {
                *pos = next;

                Wrap::Continue
            }
            Tile::Closed => unreachable!(),
        }
    }

    fn wrap_cube(&self, pos: &mut Pos, direction: &mut Direction, delta: &mut Pos) -> Wrap {
        const SIDE_LEN: i32 = 50;

        // draw faces on paper, connect sides with lines, hardcode those lines
        let (face, next_direction) = match (pos.x / SIDE_LEN, pos.y / SIDE_LEN, *direction) {
            (0, 2, UP) => (Pos::new(1, 1), RIGHT),
            (0, 3, DOWN) => (Pos::new(2, 0), DOWN),

            (1, 0, UP) => (Pos::new(0, 3), RIGHT),
            (1, 2, DOWN) => (Pos::new(0, 3), LEFT),

            (2, 0, UP) => (Pos::new(0, 3), UP),
            (2, 0, DOWN) => (Pos::new(1, 1), LEFT),

            (1, 0, LEFT) => (Pos::new(0, 2), RIGHT),
            (2, 0, RIGHT) => (Pos::new(1, 2), LEFT),

            (1, 1, LEFT) => (Pos::new(0, 2), DOWN),
            (1, 1, RIGHT) => (Pos::new(2, 0), UP),

            (0, 2, LEFT) => (Pos::new(1, 0), RIGHT),
            (1, 2, RIGHT) => (Pos::new(2, 0), LEFT),

            (0, 3, LEFT) => (Pos::new(1, 0), DOWN),
            (0, 3, RIGHT) => (Pos::new(1, 2), UP),

            // EXAMPLE:
            // (0, 1, UP) => (Pos::new(2, 0), DOWN),
            // (0, 1, DOWN) => (Pos::new(2, 2), UP),

            // (1, 1, UP) => (Pos::new(2, 0), RIGHT),
            // (1, 1, DOWN) => (Pos::new(2, 2), RIGHT),

            // (2, 0, UP) => (Pos::new(0, 1), DOWN),
            // (2, 2, DOWN) => (Pos::new(0, 1), UP),

            // (3, 2, UP) => (Pos::new(2, 1), LEFT),
            // (3, 2, DOWN) => (Pos::new(0, 1), RIGHT),

            // (2, 0, LEFT) => (Pos::new(1, 1), DOWN),
            // (2, 0, RIGHT) => (Pos::new(3, 2), LEFT),

            // (0, 1, LEFT) => (Pos::new(3, 2), UP),
            // (2, 1, RIGHT) => (Pos::new(3, 2), DOWN),

            // (2, 2, LEFT) => (Pos::new(1, 1), UP),
            // (3, 2, RIGHT) => (Pos::new(2, 0), RIGHT),
            _ => unreachable!(),
        };

        let in_face = *pos % SIDE_LEN;

        let i = match *direction {
            RIGHT => in_face.y,
            DOWN => SIDE_LEN - 1 - in_face.x,
            LEFT => SIDE_LEN - 1 - in_face.y,
            UP => in_face.x,
            _ => unreachable!(),
        };

        let in_face_delta = match next_direction {
            RIGHT => Pos::new(0, i),
            DOWN => Pos::new(SIDE_LEN - 1 - i, 0),
            LEFT => Pos::new(SIDE_LEN - 1, SIDE_LEN - 1 - i),
            UP => Pos::new(i, SIDE_LEN - 1),
            _ => unreachable!(),
        };

        let next = face * SIDE_LEN + in_face_delta;

        match self.get(next) {
            Tile::Wall => Wrap::Stop,
            Tile::Open => {
                *pos = next;
                *direction = next_direction;
                *delta = direction.delta();

                Wrap::Continue
            }
            Tile::Closed => unreachable!(),
        }
    }

    fn is_valid_pos(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.x < self.width
            && pos.y >= 0
            && pos.y < self.tiles.len() as i32 / self.width
    }

    fn get(&self, pos @ Pos { x, y }: Pos) -> Tile {
        if self.is_valid_pos(pos) {
            self.tiles[(y * self.width + x) as usize]
        } else {
            Tile::Closed
        }
    }

    fn parse(input: &str) -> Result<(Self, Path<'_>)> {
        let (board, path) = input.split_once("\n\n").wrap_err("missing blank line")?;

        let (height, width) = board.lines().fold((0, 0), |(height, width), line| {
            (height + 1, width.max(line.len()))
        });

        let mut tiles = Vec::with_capacity(height * width);

        for line in board.lines().map(str::as_bytes) {
            for &byte in line.iter() {
                tiles.push(Tile::try_from(byte)?);
            }

            let remaining = width - line.len();

            if remaining > 0 {
                tiles.extend(iter::repeat(Tile::Closed).take(remaining))
            }
        }

        let this = Self {
            tiles: tiles.into_boxed_slice(),
            width: width as i32,
        };

        Ok((this, Path(path.as_bytes())))
    }
}

#[derive(Copy, Clone)]
struct Path<'p>(&'p [u8]);

enum Move {
    Distance(i16),
    Turn(Turn),
}

enum Turn {
    Left,
    Right,
}

impl Path<'_> {
    #[inline]
    fn next(&mut self) -> Option<Result<Move>> {
        let [first, rest @ ..] = self.0 else {
            return None;
        };
        self.0 = rest;

        match first {
            b'L' => Some(Ok(Move::Turn(Turn::Left))),
            b'R' => Some(Ok(Move::Turn(Turn::Right))),
            b'0'..=b'9' => {
                let mut n = (*first & 0xF) as i16;

                loop {
                    match self.0 {
                        [b'L' | b'R', ..] | [] => break,
                        [next @ b'0'..=b'9', rest @ ..] => {
                            n = n * 10 + (*next & 0xF) as i16;
                            self.0 = rest;
                        }
                        [next, ..] => return Some(Err(eyre!("invalid digit `{}`", *next as char))),
                    }
                }

                Some(Ok(Move::Distance(n)))
            }
            _ => Some(Err(eyre!("invalid move `{}`", *first as char))),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Closed,
}

impl TryFrom<u8> for Tile {
    type Error = Report;

    #[inline]
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b' ' => Ok(Self::Closed),
            b'.' => Ok(Self::Open),
            b'#' => Ok(Self::Wall),
            _ => bail!("invalid tile `{}`", byte as char),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Direction(u8);

const RIGHT: Direction = Direction(0);
const DOWN: Direction = Direction(1);
const LEFT: Direction = Direction(2);
const UP: Direction = Direction(3);

impl Direction {
    fn turn_right(self) -> Self {
        Self((self.0 + 1) % 4)
    }

    fn turn_left(self) -> Self {
        Self((self.0 + 3) % 4)
    }

    fn delta(self) -> Pos {
        const DELTAS: [Pos; 4] = [
            Pos::new(1, 0),
            Pos::new(0, 1),
            Pos::new(-1, 0),
            Pos::new(0, -1),
        ];

        get!(DELTAS, self.0 as usize)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem<i32> for Pos {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}
