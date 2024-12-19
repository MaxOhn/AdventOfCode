use std::{collections::HashMap, convert::identity};

use aoc_rust::Solution;
use eyre::Result;
use nom::{
    bytes::complete as by,
    character::complete as ch,
    combinator::{flat_map, map, opt},
    multi::{fold_many1, separated_list1},
    sequence::terminated,
};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

type Cache<'a, P> = HashMap<&'a str, <P as Part>::Cached, ahash::RandomState>;

fn part1(input: &str) -> usize {
    solve::<Part1>(input)
}

fn part2(input: &str) -> i64 {
    solve::<Part2>(input)
}

fn solve<P: Part>(input: &str) -> P::Output {
    let available = terminated(separated_list1(by::tag(", "), ch::alpha1), ch::multispace1);

    let applied = |available: Vec<_>| {
        let mut cache = Cache::<P>::default();
        let line = terminated(ch::alpha1, opt(ch::newline::<_, ()>));
        let apply = move |line| P::recurse(&available, line, &mut cache);

        fold_many1(map(line, apply), <P::Output>::default, P::fold_output)
    };

    let (_, res) = flat_map(available, applied)(input).expect("bad input");

    res
}

trait Part {
    type Cached: Copy + Default;
    type Output: Default;

    const EMPTY: Self::Cached;

    fn fold_cached(iter: impl Iterator<Item = Self::Cached>) -> Self::Cached;

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output;

    fn recurse<'a>(available: &[&str], find: &'a str, cache: &mut Cache<'a, Self>) -> Self::Cached {
        if let Some(cached) = cache.get(find) {
            *cached
        } else if find.is_empty() {
            Self::EMPTY
        } else {
            let iter = available
                .iter()
                .filter_map(|a| find.strip_prefix(a))
                .map(|find| Self::recurse(available, find, cache));

            let res = Self::fold_cached(iter);
            cache.insert(find, res);

            res
        }
    }
}

struct Part1;

impl Part for Part1 {
    type Cached = bool;
    type Output = usize;

    const EMPTY: Self::Cached = true;

    fn fold_cached(mut iter: impl Iterator<Item = Self::Cached>) -> Self::Cached {
        iter.any(identity)
    }

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output {
        acc + usize::from(next)
    }
}

struct Part2;

impl Part for Part2 {
    type Cached = i64;
    type Output = Self::Cached;

    const EMPTY: Self::Cached = 1;

    fn fold_cached(iter: impl Iterator<Item = Self::Cached>) -> Self::Cached {
        iter.sum()
    }

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output {
        acc + next
    }
}
