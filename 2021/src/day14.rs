use std::collections::{BTreeMap, HashMap};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let template = lines.next().unwrap().as_bytes().to_owned();

    let mut pairs = Pairs::new();

    for line in lines {
        let (left, right) = line.trim_end().split_once(" -> ").unwrap();
        let left = left.as_bytes();

        pairs.insert((left[0], left[1]), right.as_bytes()[0]);
    }

    let p1 = solve(&template, &pairs, 10);
    let p2 = solve(&template, &pairs, 40);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn solve(template: &[u8], pairs: &Pairs, depth: u8) -> usize {
    let mut counts = Counts::new();
    let mut cache = Cache::new();

    for (a, b) in template.iter().zip(template.iter().skip(1)) {
        *counts.entry(*a).or_default() += 1;
        recurse(*a, *b, depth, &pairs, &mut counts, &mut cache);
    }

    let (min, max) = counts
        .into_iter()
        .map(|(_, v)| v)
        .fold((usize::MAX, 0), |(min, max), count| {
            (min.min(count), max.max(count))
        });

    max - min
}

type Pairs = HashMap<(u8, u8), u8>;
type Counts = BTreeMap<u8, usize>; // faster than HashMap
type Cache = HashMap<(u8, u8, u8), Counts>;

fn recurse(a: u8, b: u8, depth: u8, pairs: &Pairs, total: &mut Counts, cache: &mut Cache) {
    let counts = match cache.get(&(a, b, depth)) {
        Some(counts) => counts,
        None => {
            let mut counts = Counts::new();

            if let Some(c) = pairs.get(&(a, b)).filter(|_| depth > 0).copied() {
                *counts.entry(c).or_default() += 1;

                recurse(a, c, depth - 1, pairs, &mut counts, cache);
                recurse(c, b, depth - 1, pairs, &mut counts, cache);
            }

            cache.entry((a, b, depth)).or_insert(counts)
        }
    };

    for (k, v) in counts {
        *total.entry(*k).or_default() += *v;
    }
}
