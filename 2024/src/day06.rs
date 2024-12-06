use std::{borrow::Cow, cell::RefCell, collections::HashSet, hash::Hash, ops::Add, sync::OnceLock};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let Some(input) = InputMap::new(input) else {
        return 0;
    };

    let Some(start) = input.start() else {
        return 0;
    };

    let seen = input.trace_path(start as i16);

    seen.len()
}

fn part2(input: &str) -> usize {
    let Some(input) = InputMap::new(input) else {
        return 0;
    };

    let Some(start) = input.start() else {
        return 0;
    };

    input
        .trace_path(start as i16)
        .into_par_iter()
        .filter(|&i| {
            INPUT.with(|once| {
                let mut state = once
                    .get_or_init(|| RefCell::new(State::new(&input)))
                    .borrow_mut();

                state.input.bytes.to_mut()[i as usize] = b'#';
                let is_loop = state.is_loop(start as i16);
                state.input.bytes.to_mut()[i as usize] = b'.';

                is_loop
            })
        })
        .count()
}

#[derive(Clone)]
struct InputMap<'a> {
    bytes: Cow<'a, [u8]>,
    w: i16,
    h: i16,
}

impl<'a> InputMap<'a> {
    fn new(input: &'a str) -> Option<Self> {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', bytes)? as i16 + 1;
        let h = (bytes.len() as i16 + 1) / w;

        Some(Self {
            bytes: Cow::Borrowed(bytes),
            w,
            h,
        })
    }

    fn to_owned<'b>(&self) -> InputMap<'b> {
        InputMap {
            bytes: Cow::Owned(self.bytes.as_ref().to_vec()),
            w: self.w,
            h: self.h,
        }
    }

    fn start(&self) -> Option<usize> {
        memchr::memchr(b'^', &*self.bytes)
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }

    fn trace_path(&self, start: i16) -> HashSet<i16, FxBuildHasher> {
        let mut seen = HashSet::with_hasher(FxBuildHasher::default());
        seen.insert(start);

        let mut offset = Pos::new(0, -1);
        let mut curr = Pos::new(start % self.w, start / self.w);

        loop {
            let next = curr + offset;

            if !self.contains(next) {
                return seen;
            }

            let idx = next.to_idx(self.w);

            if self.bytes[idx as usize] == b'#' {
                offset.rotate();
            } else {
                seen.insert(idx);
                curr = next;
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    const fn rotate(&mut self) {
        *self = match (self.x, self.y) {
            (-1, 0) => Self::new(0, -1),
            (0, -1) => Self::new(1, 0),
            (1, 0) => Self::new(0, 1),
            (0, 1) => Self::new(-1, 0),
            _ => unreachable!(),
        }
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

thread_local! {
    static INPUT: OnceLock<RefCell<State>> = OnceLock::new();
}

struct State {
    input: InputMap<'static>,
    seen: HashSet<(i16, Pos), FxBuildHasher>,
}

impl State {
    fn new(input: &InputMap<'_>) -> Self {
        Self {
            input: input.to_owned(),
            seen: HashSet::with_hasher(FxBuildHasher::default()),
        }
    }

    fn is_loop(&mut self, start: i16) -> bool {
        let mut offset = Pos::new(0, -1);
        let mut curr = Pos::new(start % self.input.w, start / self.input.w);

        self.seen.clear();
        self.seen.insert((start, offset));

        loop {
            let next = curr + offset;

            if !self.input.contains(next) {
                break;
            }

            let idx = next.to_idx(self.input.w);

            if self.input.bytes[idx as usize] == b'#' {
                offset.rotate();
            } else {
                if !self.seen.insert((idx, offset)) {
                    return true;
                }

                curr = next;
            }
        }

        false
    }
}
