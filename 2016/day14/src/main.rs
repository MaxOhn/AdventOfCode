use std::collections::HashMap;
use std::fmt::Write;
use std::time::Instant;

#[macro_use]
extern crate util;

static INPUT: &str = "zpqevtbw";

fn main() {
    let start = Instant::now();
    let p1 = part1();
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2();
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 16_106);
    assert_eq!(p2, 22_423);
}

fn part1() -> u16 {
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

fn part2() -> u16 {
    let mut idx = 0;

    let mut awaiting = HashMap::new();
    let mut done = 0;

    loop {
        let mut buf = format!("{}{}", INPUT, idx);

        let hash = (0..2016).fold(md5::compute(&buf), |last, _| {
            buf.clear();
            let _ = write!(buf, "{:x}", last);
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
