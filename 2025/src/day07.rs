use std::mem;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let Some(start) = lines.next().and_then(|line| line.find('S')) else {
        return 0;
    };

    let mut curr = Vec::with_capacity(32);
    let mut next = Vec::with_capacity(32);

    curr.push(start);

    let mut splits = 0;

    for line in lines.map(str::as_bytes) {
        for c in curr.drain(..) {
            if line[c] == b'.' {
                next.push(c);
            } else {
                next.extend([c - 1, c + 1]);
                splits += 1;
            }
        }

        next.sort_unstable();
        next.dedup();

        mem::swap(&mut curr, &mut next);
    }

    splits
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let Some(first) = lines.next() else { return 0 };

    let width = first.len();

    let Some(start) = first.find('S') else {
        return 0;
    };

    let mut curr = vec![None; width];
    let mut next = vec![None; width];

    curr[start] = Some(1_u64);

    for line in lines.map(str::as_bytes) {
        for (x, c) in curr.iter_mut().enumerate() {
            let Some(timelines) = c.take() else { continue };

            if line[x] == b'.' {
                *next[x].get_or_insert_default() += timelines;
            } else {
                *next[x - 1].get_or_insert_default() += timelines;
                *next[x + 1].get_or_insert_default() += timelines;
            }
        }

        mem::swap(&mut curr, &mut next);
    }

    curr.into_iter().flat_map(std::convert::identity).sum()
}
