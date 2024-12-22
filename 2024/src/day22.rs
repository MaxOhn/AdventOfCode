use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;
use rayon::{prelude::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

trait SecretNumber {
    fn step(&mut self);
}

impl SecretNumber for i64 {
    fn step(&mut self) {
        *self = (*self ^ (*self * 64)) % 16777216;
        *self = (*self ^ (*self / 32)) % 16777216;
        *self = (*self ^ (*self * 2048)) % 16777216;
    }
}

fn parse_line(line: &str) -> i64 {
    line.bytes().fold(0, |n, byte| n * 10 + (byte & 0xF) as i64)
}

fn part1(input: &str) -> i64 {
    input
        .par_lines()
        .map(parse_line)
        .map(|mut n| {
            (0..2000).for_each(|_| n.step());

            n
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let iter = |mut n: i64| {
        (0..=2000).map(move |_| {
            let res = n;
            n.step();

            (res % 10) as i8
        })
    };

    input.par_lines().map(parse_line).for_each(|n| {
        DATA.with_borrow_mut(
            |ThreadData {
                 bananas,
                 seen,
                 nums,
             }| {
                seen.clear();
                nums.clear();
                nums.extend(iter(n));

                for window in nums.windows(5) {
                    let [a, b, c, d, e] = window else {
                        unreachable!()
                    };

                    let changes = [b - a, c - b, d - c, e - d];

                    if seen.insert(changes) {
                        *bananas.entry(changes).or_default() += *e as i32;
                    }
                }
            },
        )
    });

    let mut iter = rayon::broadcast(|_| DATA.with_borrow(|data| data.bananas.clone())).into_iter();

    let Some(mut bananas) = iter.next() else {
        return 0;
    };

    for thread_bananas in iter {
        for (changes, sum) in thread_bananas {
            *bananas.entry(changes).or_default() += sum;
        }
    }

    bananas.values().max().copied().unwrap_or(0)
}

struct ThreadData {
    bananas: HashMap<[i8; 4], i32, FxBuildHasher>,
    nums: Vec<i8>,
    seen: HashSet<[i8; 4], FxBuildHasher>,
}

impl ThreadData {
    const fn new() -> Self {
        Self {
            bananas: HashMap::with_hasher(FxBuildHasher::new()),
            nums: Vec::new(),
            seen: HashSet::with_hasher(FxBuildHasher::new()),
        }
    }
}

thread_local! {
    static DATA: RefCell<ThreadData> = const { RefCell::new(ThreadData::new()) };
}
