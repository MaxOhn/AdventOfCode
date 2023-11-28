#[macro_use]
extern crate aoc_rust;

use std::{collections::HashMap, fmt::Write};

use aoc_rust::Solution;

pub fn run(_input: &str) -> eyre::Result<Solution> {
    Ok(Solution::new().part1(part1()).part2(part2()))
}

const INPUT: &str = "zpqevtbw";

pub fn part1() -> u16 {
    let mut idx = 0;

    let mut awaiting = HashMap::new();
    let mut done = 0;

    loop {
        let hash = md5::compute(format!("{}{}", INPUT, idx));

        if let Some(i) = check_hash(idx, &*hash, &mut awaiting, &mut done) {
            return i;
        }

        idx += 1;
    }
}

pub fn part2() -> u16 {
    let mut idx = 0;

    let mut awaiting = HashMap::new();
    let mut done = 0;

    loop {
        let mut buf = format!("{}{}", INPUT, idx);

        let hash = (0..2016).fold(md5::compute(&buf), |last, _| {
            buf.clear();
            let _ = write!(buf, "{last:x}");
            md5::compute(&buf)
        });

        if let Some(i) = check_hash(idx, &*hash, &mut awaiting, &mut done) {
            return i;
        }

        idx += 1;
    }
}

type AwaitList = HashMap<u8, Vec<u16>>;

fn check_hash(idx: u16, hash: &[u8; 16], awaiting: &mut AwaitList, done: &mut u8) -> Option<u16> {
    for w in hash.windows(3) {
        if (get!(w, 1) % 16 == get!(w, 1) >> 4)
            && ((get!(w, 0) == get!(w, 1) && get!(w, 1) % 16 == get!(w, 2) >> 4)
                || (get!(w, 0) % 16 == get!(w, 1) % 16 && get!(w, 1) == get!(w, 2)))
        {
            if let Some(indices) = awaiting.remove(&(get!(w, 0) % 16)) {
                for i in indices {
                    if idx - i > 1000 {
                        continue;
                    }

                    *done += 1;

                    if *done == 64 {
                        return Some(i);
                    }
                }
            }
        }
    }

    for w in hash.windows(2) {
        if (get!(w, 0) % 16 == get!(w, 0) >> 4 && get!(w, 0) % 16 == get!(w, 1) >> 4)
            || (get!(w, 0) % 16 == get!(w, 1) >> 4 && get!(w, 1) >> 4 == get!(w, 1) % 16)
        {
            awaiting
                .entry(get!(w, 0) % 16)
                .or_insert(Vec::with_capacity(1))
                .push(idx);

            break;
        }
    }

    None
}
