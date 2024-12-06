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
    static UPDATE: RefCell<Update> = RefCell::new(Update::new());
}

type Rules = [Vec<u8>; 100];

fn parse_rules(input: &str) -> Option<(Rules, &str)> {
    let mut rules = [(); 100].map(|_| Vec::new());

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
                rules[x as usize].push(y);
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

struct Update {
    pages: Vec<u8>,
    indices: [u8; 100],
}

impl Update {
    fn new() -> Self {
        Self {
            pages: Vec::new(),
            indices: [u8::MAX; 100],
        }
    }

    fn parse(&mut self, line: &str) {
        self.pages.clear();
        self.indices.fill(u8::MAX);

        let iter = line.as_bytes().chunks(3).enumerate().map(|(i, chunk)| {
            let [a, b, ..] = chunk else { unreachable!() };

            let n = (a & 0xF) * 10 + (b & 0xF);
            self.indices[n as usize] = i as u8;

            n
        });

        self.pages.extend(iter);
    }

    fn find(&self, needle: u8) -> Option<u8> {
        let idx = self.indices[needle as usize];

        (idx < u8::MAX).then_some(idx)
    }

    fn middle(&self) -> u16 {
        self.pages[self.pages.len() / 2] as u16
    }

    fn swap(&mut self, x: u8, y: u8) {
        let ix = self.indices[x as usize];
        let iy = self.indices[y as usize];

        self.indices.swap(x as usize, y as usize);
        self.pages.swap(ix as usize, iy as usize);
    }
}

pub fn part1(input: &str) -> u16 {
    let Some((rules, updates)) = parse_rules(input) else {
        return 0;
    };

    updates
        .par_lines()
        .filter_map(|line| {
            UPDATE.with_borrow_mut(|update| {
                update.parse(line);

                rules
                    .iter()
                    .zip(0..)
                    .all(|(ys, x)| {
                        let Some(ix) = update.find(x) else {
                            return true;
                        };

                        ys.iter()
                            .all(|&y| update.find(y).map_or(true, |iy| ix < iy))
                    })
                    .then_some(update.middle())
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
            UPDATE.with_borrow_mut(|update| {
                update.parse(line);

                let mut iters = 0;
                let mut sorted = false;

                while !sorted {
                    iters += 1;
                    sorted = true;

                    for (ys, x) in rules.iter().zip(0..) {
                        let Some(ix) = update.find(x) else {
                            continue;
                        };

                        for &y in ys {
                            if let Some(iy) = update.find(y) {
                                if iy < ix {
                                    sorted = false;
                                    update.swap(x, y);
                                }
                            }
                        }
                    }
                }

                (iters > 1).then_some(update.middle())
            })
        })
        .sum()
}
