use std::{
    cell::RefCell,
    collections::HashSet,
    hash::Hash,
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
    solve::<Part1>(input)
}

fn part2(input: &str) -> usize {
    solve::<Part2>(input)
}

trait Part {
    type Set: Set;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize;
}

trait Set {
    fn reset(&mut self);
    fn insert(&mut self, pos: Pos);
    fn len(&self) -> usize;
}

fn solve<P: Part>(input: &str) -> usize {
    let grid = Grid::new(input);
    let w = grid.w as usize + 1;

    grid.bytes
        .iter()
        .enumerate()
        .filter(|(_, &byte)| byte == b'0')
        .map(|(i, _)| {
            P::with_data(|ThreadData { stack, set }| {
                let start = Pos::from_index(i, w);
                stack.push(start);
                set.reset();

                while let Some(curr) = stack.pop() {
                    let height = grid[curr];

                    if height == b'9' {
                        set.insert(curr);
                        continue;
                    }

                    for dir in DIRECTIONS {
                        let next = curr + dir;

                        if !grid.contains(next) {
                            continue;
                        }

                        if height + 1 == grid[next] {
                            stack.push(next);
                        }
                    }
                }

                set.len()
            })
        })
        .sum()
}

struct Part1;

impl Part for Part1 {
    type Set = HashSet<Pos, FxBuildHasher>;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize {
        P1_DATA.with_borrow_mut(f)
    }
}

struct Part2;

impl Part for Part2 {
    type Set = usize;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize {
        P2_DATA.with_borrow_mut(f)
    }
}

impl Set for HashSet<Pos, FxBuildHasher> {
    fn reset(&mut self) {
        self.clear();
    }

    fn insert(&mut self, pos: Pos) {
        self.insert(pos);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl Set for usize {
    fn reset(&mut self) {
        *self = 0;
    }

    fn insert(&mut self, _: Pos) {
        *self += 1;
    }

    fn len(&self) -> usize {
        *self
    }
}

struct Grid<'a> {
    bytes: &'a [u8],
    w: i8,
    h: i8,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', bytes).unwrap_or(bytes.len());
        let h = bytes.len() / w;

        Self {
            bytes,
            w: w as i8,
            h: h as i8,
        }
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }
}

impl Index<Pos> for Grid<'_> {
    type Output = u8;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.bytes[pos.y as usize * (self.w as usize + 1) + pos.x as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    fn from_index(idx: usize, w: usize) -> Self {
        Self::new((idx % w) as i8, (idx / w) as i8)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

const DIRECTIONS: [Pos; 4] = [
    Pos::new(0, 1),
    Pos::new(0, -1),
    Pos::new(1, 0),
    Pos::new(-1, 0),
];

struct ThreadData<P: Part + ?Sized> {
    stack: Vec<Pos>,
    set: P::Set,
}

impl<P: Part<Set: Default>> Default for ThreadData<P> {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            set: Default::default(),
        }
    }
}

thread_local! {
    static P1_DATA: RefCell<ThreadData<Part1>> = RefCell::new(ThreadData::default());
    static P2_DATA: RefCell<ThreadData<Part2>> = RefCell::new(ThreadData::default());
}
