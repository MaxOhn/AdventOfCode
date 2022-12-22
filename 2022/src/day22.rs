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
    let mut direction = Direction::Right;

    while let Some(next) = path.next() {
        match next? {
            Move::Distance(n) => board.steps(&mut pos, n, &mut direction, wrap),
            Move::Turn(Turn::Left) => direction = direction.turn_left(),
            Move::Turn(Turn::Right) => direction = direction.turn_right(),
        }
    }

    Ok(1000 * (pos.y + 1) + 4 * (pos.x + 1) + direction as i32)
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
    fn steps(&self, pos: &mut Pos, count: i32, direction: &mut Direction, wrap: WrapFn) {
        let mut delta = direction.delta();

        for _ in 0..count {
            let next = *pos + delta;

            match self.get(next) {
                Tile::Wall => return,
                Tile::Open => {
                    *pos = next;

                    continue;
                }
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
            (0, 2, Direction::Up) => (Pos::new(1, 1), Direction::Right),
            (0, 3, Direction::Down) => (Pos::new(2, 0), Direction::Down),

            (1, 0, Direction::Up) => (Pos::new(0, 3), Direction::Right),
            (1, 2, Direction::Down) => (Pos::new(0, 3), Direction::Left),

            (2, 0, Direction::Up) => (Pos::new(0, 3), Direction::Up),
            (2, 0, Direction::Down) => (Pos::new(1, 1), Direction::Left),

            (1, 0, Direction::Left) => (Pos::new(0, 2), Direction::Right),
            (2, 0, Direction::Right) => (Pos::new(1, 2), Direction::Left),

            (1, 1, Direction::Left) => (Pos::new(0, 2), Direction::Down),
            (1, 1, Direction::Right) => (Pos::new(2, 0), Direction::Up),

            (0, 2, Direction::Left) => (Pos::new(1, 0), Direction::Right),
            (1, 2, Direction::Right) => (Pos::new(2, 0), Direction::Left),

            (0, 3, Direction::Left) => (Pos::new(1, 0), Direction::Down),
            (0, 3, Direction::Right) => (Pos::new(1, 2), Direction::Up),

            // EXAMPLE:
            // (0, 1, Direction::Up) => (Pos::new(2, 0), Direction::Down),
            // (0, 1, Direction::Down) => (Pos::new(2, 2), Direction::Up),

            // (1, 1, Direction::Up) => (Pos::new(2, 0), Direction::Right),
            // (1, 1, Direction::Down) => (Pos::new(2, 2), Direction::Right),

            // (2, 0, Direction::Up) => (Pos::new(0, 1), Direction::Down),
            // (2, 2, Direction::Down) => (Pos::new(0, 1), Direction::Up),

            // (3, 2, Direction::Up) => (Pos::new(2, 1), Direction::Left),
            // (3, 2, Direction::Down) => (Pos::new(0, 1), Direction::Right),

            // (2, 0, Direction::Left) => (Pos::new(1, 1), Direction::Down),
            // (2, 0, Direction::Right) => (Pos::new(3, 2), Direction::Left),

            // (0, 1, Direction::Left) => (Pos::new(3, 2), Direction::Up),
            // (2, 1, Direction::Right) => (Pos::new(3, 2), Direction::Down),

            // (2, 2, Direction::Left) => (Pos::new(1, 1), Direction::Up),
            // (3, 2, Direction::Right) => (Pos::new(2, 0), Direction::Right),
            _ => unreachable!(),
        };

        let in_face = *pos % SIDE_LEN;

        let i = match *direction {
            Direction::Right => in_face.y,
            Direction::Down => SIDE_LEN - 1 - in_face.x,
            Direction::Left => SIDE_LEN - 1 - in_face.y,
            Direction::Up => in_face.x,
        };

        let in_face_delta = match next_direction {
            Direction::Right => Pos::new(0, i),
            Direction::Down => Pos::new(SIDE_LEN - 1 - i, 0),
            Direction::Left => Pos::new(SIDE_LEN - 1, SIDE_LEN - 1 - i),
            Direction::Up => Pos::new(i, SIDE_LEN - 1),
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
        let (height, width) = input
            .lines()
            .take_while(|line| !line.is_empty())
            .fold((0, 0), |(height, width), line| {
                (height + 1, width.max(line.len()))
            });

        let mut tiles = Vec::with_capacity(height * width);
        let mut lines = input.lines().map(str::as_bytes);

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            for &byte in line.iter() {
                tiles.push(Tile::try_from(byte)?);
            }

            let remaining = width - line.len();

            if remaining > 0 {
                tiles.extend(iter::repeat(Tile::Closed).take(remaining))
            }
        }

        let path = lines.next().wrap_err("missing path")?;

        let this = Self {
            tiles: tiles.into(),
            width: width as i32,
        };

        Ok((this, Path(path)))
    }
}

#[derive(Copy, Clone)]
struct Path<'p>(&'p [u8]);

enum Move {
    Distance(i32),
    Turn(Turn),
}

enum Turn {
    Left,
    Right,
}

impl Path<'_> {
    fn next(&mut self) -> Option<Result<Move>> {
        let [first, rest @ ..] = self.0 else { return None };
        self.0 = rest;

        match first {
            b'L' => Some(Ok(Move::Turn(Turn::Left))),
            b'R' => Some(Ok(Move::Turn(Turn::Right))),
            b'0'..=b'9' => {
                let mut n = (*first & 0xF) as i32;

                loop {
                    let [next, rest @ ..] = self.0 else { break };

                    match next {
                        b'L' | b'R' => break,
                        b'0'..=b'9' => {
                            n = n * 10 + (*next & 0xF) as i32;
                            self.0 = rest;
                        }
                        _ => return Some(Err(eyre!("invalid move `{}`", *first as char))),
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

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn delta(self) -> Pos {
        match self {
            Self::Right => Pos::new(1, 0),
            Self::Down => Pos::new(0, 1),
            Self::Left => Pos::new(-1, 0),
            Self::Up => Pos::new(0, -1),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
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
