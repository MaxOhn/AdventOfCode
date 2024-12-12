use std::{
    collections::{BTreeSet, HashSet},
    ops::{Add, Index},
};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    solve(input, |Data { region, .. }| {
        let mut perimeter = 0;

        for &pos in region.iter() {
            for dir in DIRECTIONS {
                if !region.contains(&(pos + dir)) {
                    perimeter += 1;
                }
            }
        }

        region.len() * perimeter
    })
}

fn part2(input: &str) -> usize {
    solve(
        input,
        |Data {
             region,
             sides,
             remove,
             ..
         }| {
            let mut side_count = 0;

            for dir in DIRECTIONS {
                sides.clear();

                for &pos in region.iter() {
                    let next = pos + dir;

                    if !region.contains(&next) {
                        sides.insert(next);
                    }
                }

                remove.clear();

                for &side in sides.iter() {
                    let mut next = Pos::new(side.x + dir.y, side.y + dir.x);

                    while sides.contains(&next) && remove.insert(next) {
                        next = Pos::new(next.x + dir.y, next.y + dir.x);
                    }
                }

                side_count += sides.len() - remove.len();
            }

            region.len() * side_count
        },
    )
}

fn solve(input: &str, process_region: fn(&mut Data) -> usize) -> usize {
    let grid = Grid::new(input);
    let w = grid.w + 1;

    let mut done = HashSet::with_hasher(FxBuildHasher::default());
    let mut stack = Vec::new();
    let mut data = Data::default();

    let mut sum = 0;

    for (byte, i) in grid.bytes.iter().copied().zip(0..) {
        if byte == b'\n' || !done.insert(i) {
            continue;
        }

        let pos = Pos::from_idx(i, w);
        data.region.insert(pos);
        stack.push(pos);

        while let Some(curr) = stack.pop() {
            for dir in DIRECTIONS {
                let next = curr + dir;

                if grid
                    .get(next)
                    .is_some_and(|n| n == byte && data.region.insert(next))
                {
                    stack.push(next);
                }
            }
        }

        sum += process_region(&mut data);
        done.extend(data.region.drain().map(|pos| pos.to_idx(w)));
    }

    sum
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    const fn from_idx(idx: i16, w: i16) -> Self {
        Self::new(idx % w, idx / w)
    }

    const fn to_idx(self, w: i16) -> i16 {
        self.y * w + self.x
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

struct Grid<'a> {
    bytes: &'a [u8],
    w: i16,
    h: i16,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', bytes).unwrap_or(bytes.len());
        let h = bytes.len() / w;

        Self {
            bytes,
            w: w as i16,
            h: h as i16,
        }
    }

    fn get(&self, pos: Pos) -> Option<u8> {
        (pos.x >= 0 && pos.y >= 0 && pos.x < self.w && pos.y < self.h).then(|| self[pos])
    }
}

impl Index<Pos> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.bytes[index.to_idx(self.w + 1) as usize]
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Region = HashSet<Pos, FxBuildHasher>;

const DIRECTIONS: [Pos; 4] = [
    Pos::new(1, 0),
    Pos::new(0, 1),
    Pos::new(-1, 0),
    Pos::new(0, -1),
];

#[derive(Default)]
struct Data {
    region: Region,
    sides: BTreeSet<Pos>,
    remove: HashSet<Pos, FxBuildHasher>,
}
