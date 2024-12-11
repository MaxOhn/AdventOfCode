use std::{cell::RefCell, collections::HashMap, thread::LocalKey};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1_sequential(input);
    let p2 = part2_parallel(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn part1_sequential(input: &str) -> u64 {
    let res = solve_sequential::<25>(input, || &P1_CACHE);
    P1_CACHE.take();

    res
}

pub fn part2_sequential(input: &str) -> u64 {
    let res = solve_sequential::<75>(input, || &P2_CACHE);
    P2_CACHE.take();

    res
}

pub fn part1_parallel(input: &str) -> u64 {
    let res = solve_parallel::<25>(input, || &P1_CACHE);
    rayon::broadcast(|_| P1_CACHE.take());

    res
}

pub fn part2_parallel(input: &str) -> u64 {
    let res = solve_parallel::<75>(input, || &P2_CACHE);
    rayon::broadcast(|_| P2_CACHE.take());

    res
}

type Stone = i64;
type Step = u8;
type Cache = HashMap<(Stone, Step), u64, FxBuildHasher>;
type LocalCache = LocalKey<RefCell<Cache>>;

fn solve_sequential<const TARGET: Step>(input: &str, cache: fn() -> &'static LocalCache) -> u64 {
    input
        .split_ascii_whitespace()
        .map(str::parse::<Stone>)
        .filter_map(Result::ok)
        .map(|stone| cache().with_borrow_mut(|cache| recurse::<TARGET>(0, stone, cache)))
        .sum()
}

fn solve_parallel<const TARGET: Step>(input: &str, cache: fn() -> &'static LocalCache) -> u64 {
    input
        .par_split_ascii_whitespace()
        .map(str::parse::<Stone>)
        .filter_map(Result::ok)
        .map(|stone| cache().with_borrow_mut(|cache| recurse::<TARGET>(0, stone, cache)))
        .sum()
}

fn recurse<const TARGET: Step>(step: Step, stone: Stone, cache: &mut Cache) -> u64 {
    let key = (stone, step);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    } else if step == TARGET {
        return 1;
    }

    let step = step + 1;

    let count = if stone == 0 {
        recurse::<TARGET>(step, 1, cache)
    } else if let Some((left, right)) = try_split(stone) {
        recurse::<TARGET>(step, left, cache) + recurse::<TARGET>(step, right, cache)
    } else {
        recurse::<TARGET>(step, stone * 2024, cache)
    };

    cache.insert(key, count);

    count
}

fn try_split(stone: Stone) -> Option<(Stone, Stone)> {
    let digits = stone.ilog10() + 1;

    (digits % 2 == 0).then(|| {
        let ten_pow = 10_i64.pow(digits / 2);

        (stone / ten_pow, stone % ten_pow)
    })
}

thread_local! {
    static P1_CACHE: RefCell<Cache> = RefCell::new(Cache::default());
    static P2_CACHE: RefCell<Cache> = RefCell::new(Cache::default());
}
