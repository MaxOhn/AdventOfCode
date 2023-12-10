use aoc_rust::Solution;
use eyre::Result;

use self::model::*;

pub fn run(input: &str) -> Result<Solution> {
    let field: Field = input.trim().parse()?;

    let p1 = part1(&field);
    let p2 = part2(&field);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(field: &Field) -> usize {
    field.path().len() / 2
}

fn part2(field: &Field) -> usize {
    let mut enclosed = 0;

    for y in 0..field.height() {
        'next: for x in 0..field.width() {
            let pos = Pos::new(x, y);

            if field.path().contains(&pos) {
                continue;
            }

            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let offset = dir.offset();
                let mut curr = pos;

                let mut counts = [0_i8; 8];

                loop {
                    curr += offset;

                    if !field.contains(curr) {
                        break;
                    }

                    if field.path().contains(&curr) {
                        counts[field[curr] as usize] += 1;
                    }
                }

                let path_count = match dir {
                    Direction::North | Direction::South => {
                        let horizontal = counts[Cell::Horizontal as usize];
                        let left =
                            counts[Cell::NorthWest as usize] + counts[Cell::SouthWest as usize];

                        horizontal + left
                    }
                    Direction::West | Direction::East => {
                        let vertical = counts[Cell::Vertical as usize];
                        let up =
                            counts[Cell::NorthWest as usize] + counts[Cell::NorthEast as usize];

                        vertical + up
                    }
                };

                if path_count % 2 == 0 {
                    continue 'next;
                }
            }

            enclosed += 1;
        }
    }

    enclosed
}

mod model {
    use std::{
        collections::HashSet,
        ops::{Add, AddAssign, Index},
        str::FromStr,
    };

    use eyre::{ContextCompat, Report, Result};

    pub struct Field {
        inner: FieldInner,
        path: HashSet<Pos>,
    }

    impl Field {
        pub fn width(&self) -> i32 {
            self.inner.width
        }

        pub fn height(&self) -> i32 {
            self.inner.field.len() as i32 / self.width()
        }

        pub fn path(&self) -> &HashSet<Pos> {
            &self.path
        }

        pub fn contains(&self, pos: Pos) -> bool {
            self.inner.contains(pos)
        }
    }

    impl FromStr for Field {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map_or(0, str::len) as i32;

            let field = s
                .lines()
                .flat_map(|line| line.bytes().map(Cell::try_from))
                .collect::<Result<Vec<_>>>()?;

            let inner = FieldInner { width, field };

            let (mut curr, mut dir) = inner
                .start_neighbor()
                .wrap_err("missing neighboring pipe of start")?;

            let mut path = HashSet::new();

            while path.insert(curr) {
                let cell = inner[curr];

                if cell == Cell::Start {
                    break;
                }

                dir = cell.next_dir(dir.opposite()).unwrap();
                curr += dir.offset();
            }

            Ok(Self { inner, path })
        }
    }

    impl Index<Pos> for Field {
        type Output = Cell;

        fn index(&self, pos: Pos) -> &Self::Output {
            self.inner.index(pos)
        }
    }

    struct FieldInner {
        width: i32,
        field: Vec<Cell>,
    }

    impl FieldInner {
        fn contains(&self, pos: Pos) -> bool {
            pos.x >= 0
                && pos.x < self.width
                && pos.y >= 0
                && pos.y < self.field.len() as i32 / self.width
        }

        fn start_neighbor(&self) -> Option<(Pos, Direction)> {
            let start = self
                .field
                .iter()
                .zip(0..)
                .find(|(&c, _)| c == Cell::Start)
                .map(|(_, i)| Pos::new(i % self.width, i / self.width))
                .unwrap();

            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let offset = dir.offset();
                let n_pos = start + offset;

                if !self.contains(n_pos) {
                    continue;
                }

                if self[n_pos].next_dir(dir.opposite()).is_some() {
                    return Some((n_pos, dir));
                }
            }

            None
        }
    }

    impl Index<Pos> for FieldInner {
        type Output = Cell;

        fn index(&self, pos: Pos) -> &Self::Output {
            let idx = (pos.y * self.width + pos.x) as usize;

            &self.field[idx]
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Pos {
        pub x: i32,
        pub y: i32,
    }

    impl Pos {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    impl Add for Pos {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Cell {
        Ground,
        Start,
        NorthWest,
        NorthEast,
        SouthWest,
        SouthEast,
        Vertical,
        Horizontal,
    }

    impl Cell {
        pub fn next_dir(self, from: Direction) -> Option<Direction> {
            let to = match (self, from) {
                (Cell::NorthWest, Direction::North) => Direction::West,
                (Cell::NorthWest, Direction::West) => Direction::North,
                (Cell::NorthEast, Direction::North) => Direction::East,
                (Cell::NorthEast, Direction::East) => Direction::North,
                (Cell::SouthWest, Direction::West) => Direction::South,
                (Cell::SouthWest, Direction::South) => Direction::West,
                (Cell::SouthEast, Direction::South) => Direction::East,
                (Cell::SouthEast, Direction::East) => Direction::South,
                (Cell::Vertical, Direction::South) => Direction::North,
                (Cell::Vertical, Direction::North) => Direction::South,
                (Cell::Horizontal, Direction::West) => Direction::East,
                (Cell::Horizontal, Direction::East) => Direction::West,
                _ => return None,
            };

            Some(to)
        }
    }

    impl TryFrom<u8> for Cell {
        type Error = Report;

        fn try_from(byte: u8) -> Result<Self, Self::Error> {
            let cell = match byte {
                b'.' => Self::Ground,
                b'J' => Self::NorthWest,
                b'L' => Self::NorthEast,
                b'7' => Self::SouthWest,
                b'F' => Self::SouthEast,
                b'S' => Self::Start,
                b'|' => Self::Vertical,
                b'-' => Self::Horizontal,
                _ => eyre::bail!("invalid cell byte `{byte}`"),
            };

            Ok(cell)
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Direction {
        North,
        West,
        South,
        East,
    }

    impl Direction {
        pub fn offset(self) -> Pos {
            match self {
                Direction::North => Pos::new(0, -1),
                Direction::West => Pos::new(-1, 0),
                Direction::South => Pos::new(0, 1),
                Direction::East => Pos::new(1, 0),
            }
        }

        pub fn opposite(self) -> Self {
            match self {
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
            }
        }
    }
}
