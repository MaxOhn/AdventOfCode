use std::cell::RefCell;

use aoc_rust::Solution;
use eyre::Result;
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

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules
        .lines()
        .filter_map(|line| line.split_once('|'))
        .filter_map(|(x, y)| x.parse().ok().zip(y.parse().ok()))
        .map(|(x, y)| Rule::new(x, y))
        .collect()
}

fn part1(input: &str) -> u16 {
    let Some((rules, updates)) = input.split_once("\n\n") else {
        return 0;
    };

    let rules = parse_rules(rules);

    updates
        .par_lines()
        .filter_map(|line| {
            let iter = line.split(',').map(str::parse::<u8>).map(Result::unwrap);

            UPDATES_BUF.with_borrow_mut(|updates| {
                updates.clear();
                updates.extend(iter);

                rules
                    .iter()
                    .all(|rule| {
                        memchr::memchr(rule.x, updates)
                            .zip(memchr::memchr(rule.y, updates))
                            .map_or(true, |(x, y)| x < y)
                    })
                    .then_some(updates[updates.len() / 2] as u16)
            })
        })
        .sum()
}

fn part2(input: &str) -> u16 {
    let Some((rules, updates)) = input.split_once("\n\n") else {
        return 0;
    };

    let rules = parse_rules(rules);

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

                    for rule in rules.iter() {
                        let x = memchr::memchr(rule.x, updates);
                        let y = memchr::memchr(rule.y, updates);

                        if let (Some(x), Some(y)) = (x, y) {
                            if y < x {
                                sorted = false;
                                updates.swap(x, y);
                            }
                        }
                    }
                }

                (iters > 1).then_some(updates[updates.len() / 2] as u16)
            })
        })
        .sum()
}

struct Rule {
    x: u8,
    y: u8,
}

impl Rule {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
