use std::ops::{Add, Mul};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

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

    for y in 0..puzzle.h() as isize {
        for x in 0..puzzle.w() as isize {
            let curr = Idx::new(x, y);

            for dir in Direction::enumerate() {
                if puzzle.dir_iter(curr, dir).take(4).eq(*b"XMAS") {
                    count += 1;
                }
            }
        }
    }

    count
}

struct Puzzle<'a> {
    inner: &'a [u8],
    w: usize,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Option<Self> {
        let w = input.lines().next()?.len();

        Some(Self {
            inner: input.as_bytes(),
            w: w + 1,
        })
    }

    fn w(&self) -> usize {
        self.w - 1
    }

    fn h(&self) -> usize {
        (self.inner.len() + 1) / self.w
    }

    fn get(&self, idx: Idx) -> Option<u8> {
        let x = usize::try_from(idx.x).ok()?;
        let y = usize::try_from(idx.y).ok()?;

        if x >= self.w() || y >= self.h() {
            return None;
        }

        let idx = x + y * self.w;

        self.inner.get(idx).copied()
    }

    fn dir_iter(&self, idx: Idx, dir: Direction) -> PuzzleIter<'_> {
        PuzzleIter::new(idx, self, dir)
    }
}

#[derive(Copy, Clone, Debug)]
struct Idx {
    x: isize,
    y: isize,
}

impl Idx {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Idx {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Idx {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Copy, Clone)]
struct OffsetIter {
    offset: Idx,
    progress: isize,
}

impl OffsetIter {
    fn new(dir: Direction) -> Self {
        Self {
            offset: dir.offset(),
            progress: 0,
        }
    }
}

impl Iterator for OffsetIter {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.offset * self.progress;
        self.progress += 1;

        Some(idx)
    }
}

struct PuzzleIter<'a> {
    idx: Idx,
    puzzle: &'a Puzzle<'a>,
    offset: OffsetIter,
}

impl<'a> PuzzleIter<'a> {
    fn new(idx: Idx, puzzle: &'a Puzzle<'a>, dir: Direction) -> Self {
        Self {
            idx,
            puzzle,
            offset: OffsetIter::new(dir),
        }
    }
}

impl Iterator for PuzzleIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.puzzle.get(self.idx + self.offset.next()?)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

impl Direction {
    fn offset(self) -> Idx {
        match self {
            Self::Right => Idx::new(1, 0),
            Self::DownRight => Idx::new(1, 1),
            Self::Down => Idx::new(0, 1),
            Self::DownLeft => Idx::new(-1, 1),
            Self::Left => Idx::new(-1, 0),
            Self::UpLeft => Idx::new(-1, -1),
            Self::Up => Idx::new(0, -1),
            Self::UpRight => Idx::new(1, -1),
        }
    }

    fn enumerate() -> impl IntoIterator<Item = Self> {
        [
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
            Self::UpLeft,
            Self::Up,
            Self::UpRight,
        ]
    }
}
