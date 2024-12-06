use std::{cell::RefCell, collections::HashMap};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

thread_local! {
    static UPDATES_BUF: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

fn parse_rules(input: &str) -> Option<(HashMap<u8, Vec<u8>, FxBuildHasher>, &str)> {
    let mut rules = HashMap::<u8, Vec<u8>, _>::with_hasher(FxBuildHasher::default());

    let mut x = 0;
    let mut y = 0;
    let mut curr = &mut x;
    let mut newline = false;
    let mut chars = input.chars();

    for ch in chars.by_ref() {
        match ch as u8 {
            digit @ b'0'..=b'9' => *curr = *curr * 10 + (digit & 0xF),
            b'|' => {
                curr = &mut y;
                newline = false;
            }
            b'\n' if newline => break,
            b'\n' => {
                rules.entry(x).or_default().push(y);
                newline = true;
                x = 0;
                y = 0;
                curr = &mut x;
            }
            _ => return None,
        }
    }

    Some((rules, chars.as_str()))
}

pub fn part1(input: &str) -> u16 {
    let Some((rules, updates)) = parse_rules(input) else {
        return 0;
    };

    updates
        .par_lines()
        .filter_map(|line| {
            let iter = line.split(',').map(str::parse::<u8>).map(Result::unwrap);

            UPDATES_BUF.with_borrow_mut(|updates| {
                updates.clear();
                updates.extend(iter);

                rules
                    .iter()
                    .all(|(x, ys)| {
                        let Some(ix) = memchr::memchr(*x, updates) else {
                            return true;
                        };

                        ys.iter()
                            .all(|&y| memchr::memchr(y, updates).map_or(true, |iy| ix < iy))
                    })
                    .then_some(updates[updates.len() / 2] as u16)
            })
        })
        .sum()
}

pub fn part2(input: &str) -> u16 {
    let Some((rules, updates)) = parse_rules(input) else {
        return 0;
    };

    updates
        .par_lines()
        .filter_map(|line| {
            let iter = line.split(',').map(str::parse::<u8>).map(Result::unwrap);

            UPDATES_BUF.with_borrow_mut(|updates| {
                updates.clear();
                updates.extend(iter);

                let mut iters = 0;
                let mut sorted = false;

                while !sorted {
                    iters += 1;
                    sorted = true;

                    for (x, ys) in rules.iter() {
                        let Some(ix) = memchr::memchr(*x, updates) else {
                            continue;
                        };

                        for &y in ys {
                            if let Some(iy) = memchr::memchr(y, updates) {
                                if iy < ix {
                                    sorted = false;
                                    updates.swap(ix, iy);
                                }
                            }
                        }
                    }
                }

                (iters > 1).then_some(updates[updates.len() / 2] as u16)
            })
        })
        .sum()
}
