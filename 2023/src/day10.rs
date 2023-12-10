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

    for x in 0..field.width() {
        for y in 0..field.height() {
            let pos = Pos::new(x, y);

            if !field.path().contains(&pos) {
                enclosed += path_crossings(pos, Direction::North, field) as usize % 2;
            }
        }
    }

    enclosed
}

fn path_crossings(pos: Pos, dir: Direction, field: &Field) -> u8 {
    let offset = dir.offset();
    let mut curr = pos + offset;
    let mut counts = [0_u8; 8];

    while field.contains(curr) {
        if field.path().contains(&curr) {
            counts[field[curr] as usize] += 1;
        }

        curr += offset;
    }

    match dir {
        Direction::North | Direction::South => {
            let horizontal = counts[Cell::Horizontal as usize];
            let left = counts[Cell::NorthWest as usize] + counts[Cell::SouthWest as usize];

            horizontal + left
        }
        Direction::West | Direction::East => {
            let vertical = counts[Cell::Vertical as usize];
            let up = counts[Cell::NorthWest as usize] + counts[Cell::NorthEast as usize];

            vertical + up
        }
    }
}

mod model {
    use std::{
        ops::{Add, AddAssign, Index, IndexMut},
        str::FromStr,
    };

    use eyre::{ContextCompat, Report, Result};
    use fxhash::FxHashSet as HashSet;

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

            let mut inner = FieldInner { width, field };

            let (start, mut dir) = inner.start_dir()?;
            let mut curr = start + dir.offset();

            let mut path = HashSet::default();
            path.insert(start);

            while path.insert(curr) {
                dir = inner[curr].follow(dir.opposite()).unwrap();
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

        fn start_dir(&mut self) -> Result<(Pos, Direction)> {
            let start = self
                .field
                .iter()
                .zip(0..)
                .find(|(&c, _)| c == Cell::Start)
                .map(|(_, i)| Pos::new(i % self.width, i / self.width))
                .wrap_err("missing start")?;

            let mut neighbors = Direction::ALL.into_iter().filter(|dir| {
                let n_pos = start + dir.offset();

                self.contains(n_pos) && self[n_pos].follow(dir.opposite()).is_some()
            });

            let first = neighbors.next().wrap_err("missing neighbor of start")?;
            let second = neighbors.next().wrap_err("missing neighbor of start")?;

            let replace = match (first, second) {
                (Direction::North, Direction::West) | (Direction::West, Direction::North) => {
                    Cell::NorthWest
                }
                (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                    Cell::Vertical
                }
                (Direction::North, Direction::East) | (Direction::East, Direction::North) => {
                    Cell::NorthEast
                }
                (Direction::West, Direction::South) | (Direction::South, Direction::West) => {
                    Cell::SouthWest
                }
                (Direction::West, Direction::East) | (Direction::East, Direction::West) => {
                    Cell::Horizontal
                }
                (Direction::South, Direction::East) | (Direction::East, Direction::South) => {
                    Cell::SouthEast
                }
                _ => unreachable!(),
            };

            self[start] = replace;

            Ok((start, first))
        }
    }

    impl Index<Pos> for FieldInner {
        type Output = Cell;

        fn index(&self, pos: Pos) -> &Self::Output {
            let idx = (pos.y * self.width + pos.x) as usize;

            &self.field[idx]
        }
    }

    impl IndexMut<Pos> for FieldInner {
        fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
            let idx = (pos.y * self.width + pos.x) as usize;

            &mut self.field[idx]
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
        pub fn follow(self, from: Direction) -> Option<Direction> {
            let to = match (self, from) {
                (Cell::NorthWest, Direction::North)
                | (Cell::SouthWest, Direction::South)
                | (Cell::Horizontal, Direction::East) => Direction::West,
                (Cell::NorthWest, Direction::West)
                | (Cell::NorthEast, Direction::East)
                | (Cell::Vertical, Direction::South) => Direction::North,
                (Cell::NorthEast, Direction::North)
                | (Cell::SouthEast, Direction::South)
                | (Cell::Horizontal, Direction::West) => Direction::East,
                (Cell::SouthWest, Direction::West)
                | (Cell::SouthEast, Direction::East)
                | (Cell::Vertical, Direction::North) => Direction::South,
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
        pub const ALL: [Direction; 4] = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];

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
