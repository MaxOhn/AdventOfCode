use std::ops::{Add, Mul};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    // let p1 = part1(input);
    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn part1(input: &str) -> i32 {
    let Some(ww) = input.lines().next().map(str::len) else {
        return 0;
    };

    let w = ww + 1; // consider '\n'

    let bytes = input.as_bytes();
    let mut count = 0;

    for x in memchr::memrchr_iter(b'X', bytes) {
        let not_right = (x + 4) % w >= 3;
        let not_left = x % w >= 3;
        let not_top = x > 3 * w;
        let not_bot = x < bytes.len() - ww - 2 * w;

        if not_right {
            if input[x + 1..].starts_with("MAS") {
                count += 1;
            }

            if not_bot
                && bytes[x + w + 1] == b'M'
                && bytes[x + 2 * w + 2] == b'A'
                && bytes[x + 3 * w + 3] == b'S'
            {
                count += 1;
            }

            if not_top
                && bytes[x - w + 1] == b'M'
                && bytes[x - 2 * w + 2] == b'A'
                && bytes[x - 3 * w + 3] == b'S'
            {
                count += 1;
            }
        }

        if not_left {
            if input[..x].ends_with("SAM") {
                count += 1;
            }

            if not_bot
                && bytes[x + w - 1] == b'M'
                && bytes[x + 2 * w - 2] == b'A'
                && bytes[x + 3 * w - 3] == b'S'
            {
                count += 1;
            }

            if not_top
                && bytes[x - w - 1] == b'M'
                && bytes[x - 2 * w - 2] == b'A'
                && bytes[x - 3 * w - 3] == b'S'
            {
                count += 1;
            }
        }

        if not_bot && bytes[x + w] == b'M' && bytes[x + 2 * w] == b'A' && bytes[x + 3 * w] == b'S' {
            count += 1;
        }

        if not_top && bytes[x - w] == b'M' && bytes[x - 2 * w] == b'A' && bytes[x - 3 * w] == b'S' {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> i32 {
    let Some(ww) = input.lines().next().map(str::len) else {
        return 0;
    };

    let w = ww + 1; // consider '\n'

    let bytes = input.as_bytes();
    let mut count = 0;

    for a in memchr::memrchr_iter(b'A', &bytes[w..bytes.len() - ww]) {
        if a % w == 0 || (a + 1) % w == 0 {
            continue;
        }

        let ul = a - 1;
        let ur = a + 1;
        let dl = ul + (w << 1); // = a - 1 + 2 * ww
        let dr = ur + (w << 1);

        if ((bytes[dr] == b'M' && bytes[ul] == b'S') || (bytes[dr] == b'S' && bytes[ul] == b'M'))
            && ((bytes[ur] == b'M' && bytes[dl] == b'S')
                || (bytes[ur] == b'S' && bytes[dl] == b'M'))
        {
            count += 1;
        }
    }

    count
}

pub fn part1_structured(input: &str) -> i32 {
    let Some(puzzle) = Puzzle::new(input) else {
        return 0;
    };

    let mut count = 0;

    for needle in memchr::memchr_iter(b'X', puzzle.inner) {
        let x = needle % puzzle.w;
        let y = needle / puzzle.w;
        let curr = Pos::new(x as i16, y as i16);

        for dir in DIRECTIONS {
            if puzzle.dir_iter(curr, dir).take(3).eq(*b"MAS") {
                count += 1;
            }
        }
    }

    count
}

struct Puzzle<'a> {
    inner: &'a [u8],
    w: usize,
    h: usize,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Option<Self> {
        let w = input.lines().next()?.len();
        let h = (input.len() + 1) / w;

        Some(Self {
            inner: input.as_bytes(),
            w: w + 1,
            h,
        })
    }

    fn w(&self) -> usize {
        self.w - 1
    }

    fn h(&self) -> usize {
        self.h
    }

    fn get(&self, idx: Pos) -> Option<u8> {
        let x = usize::try_from(idx.x).ok()?;
        let y = usize::try_from(idx.y).ok()?;

        if x >= self.w() || y >= self.h() {
            return None;
        }

        let idx = x + y * self.w;

        self.inner.get(idx).copied()
    }

    fn dir_iter(&self, idx: Pos, dir: Pos) -> PuzzleIter<'_> {
        PuzzleIter::new(idx, self, dir)
    }
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    const fn new(x: i16, y: i16) -> Self {
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

impl Mul<i16> for Pos {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Copy, Clone)]
struct OffsetIter {
    offset: Pos,
    progress: i16,
}

impl OffsetIter {
    fn new(dir: Pos) -> Self {
        Self {
            offset: dir,
            progress: 1,
        }
    }
}

impl Iterator for OffsetIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.offset * self.progress;
        self.progress += 1;

        Some(idx)
    }
}

struct PuzzleIter<'a> {
    pos: Pos,
    puzzle: &'a Puzzle<'a>,
    offset: OffsetIter,
}

impl<'a> PuzzleIter<'a> {
    fn new(pos: Pos, puzzle: &'a Puzzle<'a>, offset: Pos) -> Self {
        Self {
            pos,
            puzzle,
            offset: OffsetIter::new(offset),
        }
    }
}

impl Iterator for PuzzleIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.puzzle.get(self.pos + self.offset.next()?)
    }
}

const DIRECTIONS: [Pos; 8] = [
    Pos::new(1, 0),   // Right
    Pos::new(1, 1),   // DownRight
    Pos::new(0, 1),   // Down
    Pos::new(-1, 1),  // DownLeft
    Pos::new(-1, 0),  // Left
    Pos::new(-1, -1), // UpLeft
    Pos::new(0, -1),  // Up
    Pos::new(1, -1),  // UpRight
];
