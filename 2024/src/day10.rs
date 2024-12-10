use std::{
    cell::RefCell,
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::{Add, Index},
};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::{FxBuildHasher, FxHasher32};

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
    type StackItem: Send + Sync;
    type UniqueItem: Hash + Eq;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize;
    fn init(start: Pos) -> Self::StackItem;
    fn pos(item: &Self::StackItem) -> Pos;
    fn unique_item(item: Self::StackItem) -> Self::UniqueItem;
    fn push_item(item: &Self::StackItem, next: Pos) -> Self::StackItem;
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
                stack.push(P::init(start));
                set.clear();

                while let Some(item) = stack.pop() {
                    let curr = P::pos(&item);
                    let height = grid[curr];

                    if height == b'9' {
                        set.insert(P::unique_item(item));
                        continue;
                    }

                    for dir in DIRECTIONS {
                        let next = curr + dir;

                        if !grid.contains(next) {
                            continue;
                        }

                        if height + 1 == grid[next] {
                            stack.push(P::push_item(&item, next));
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
    type StackItem = Pos;
    type UniqueItem = Pos;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize {
        P1_DATA.with_borrow_mut(f)
    }

    fn init(start: Pos) -> Self::StackItem {
        start
    }

    fn pos(item: &Self::StackItem) -> Pos {
        *item
    }

    fn unique_item(item: Self::StackItem) -> Self::UniqueItem {
        item
    }

    fn push_item(_: &Self::StackItem, next: Pos) -> Self::StackItem {
        next
    }
}

struct Part2;

impl Part for Part2 {
    type StackItem = (Pos, Self::UniqueItem, FxHasher32);
    type UniqueItem = u32;

    fn with_data(f: impl Fn(&mut ThreadData<Self>) -> usize) -> usize {
        P2_DATA.with_borrow_mut(f)
    }

    fn init(start: Pos) -> Self::StackItem {
        (start, 0, FxHasher32::default())
    }

    fn pos((pos, ..): &Self::StackItem) -> Pos {
        *pos
    }

    fn unique_item((_, hash, _): Self::StackItem) -> Self::UniqueItem {
        hash
    }

    fn push_item((_, _, hasher): &Self::StackItem, next: Pos) -> Self::StackItem {
        let mut hasher = hasher.clone();
        next.hash(&mut hasher);

        (next, hasher.finish() as u32, hasher)
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
    stack: Vec<P::StackItem>,
    set: HashSet<P::UniqueItem, FxBuildHasher>,
}

impl<P: Part> Default for ThreadData<P> {
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
