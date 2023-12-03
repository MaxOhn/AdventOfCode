use std::{collections::HashSet, ops::Index};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    assert_eq!(p1, 536576);
    assert_eq!(p2, 75741499);

    Ok(Solution::new().part1(p1).part2(p2))
}

struct Schematic<'i> {
    width: usize,
    height: usize,
    input: &'i [u8],
}

impl<'i> Schematic<'i> {
    fn new(input: &'i str) -> Self {
        Self {
            width: input.find('\n').unwrap_or(input.len()),
            height: input.lines().count(),
            input: input.as_bytes(),
        }
    }
}

impl Index<usize> for Schematic<'_> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let y = index * (self.width + 1);

        &self.input[y..][..self.width]
    }
}

fn part1(input: &str) -> u32 {
    let schematic = Schematic::new(input);

    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();

        let mut start = 0;

        while start < line.len() - 1 {
            if !bytes[start].is_ascii_digit() {
                start += 1;
                continue;
            }

            let mut end = start + 1;

            while end < line.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }

            let num: u32 = line[start..end].parse().unwrap();

            let x_range_start = if start == 0 { start } else { start - 1 };
            let x_range_end = if end == line.len() { end - 1 } else { end };

            let y_range_start = if i == 0 { i } else { i - 1 };
            let y_range_end = if i == schematic.height - 1 { i } else { i + 1 };

            'neighbors: for ny in y_range_start..=y_range_end {
                for nx in x_range_start..=x_range_end {
                    let n = schematic[ny][nx];

                    if !n.is_ascii_digit() && n != b'.' {
                        sum += num;
                        break 'neighbors;
                    }
                }
            }

            start = end + 1;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut plan = Vec::new();

    for line in input.lines() {
        let bytes = line.bytes().collect::<Vec<_>>();
        let mut nums = Vec::new();

        let mut i = 0;

        while i < bytes.len() {
            if bytes[i].is_ascii_digit() {
                let mut end = i + 1;

                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }

                let num: u32 = line[i..end].parse().unwrap();

                for _ in i..end {
                    nums.push(Kind::Num(num));
                }

                i = end;
            } else if bytes[i] == b'.' {
                nums.push(Kind::Dot);
                i += 1;
            } else if bytes[i] == b'*' {
                nums.push(Kind::Symbol(Symbol::Gear));
                i += 1;
            } else {
                nums.push(Kind::Symbol(Symbol::Other));
                i += 1;
            }
        }

        plan.push(nums);
    }

    let mut sum = 0;

    for x in 0..plan.len() {
        for y in 0..plan[x].len() {
            let Kind::Symbol(Symbol::Gear) = plan[x][y] else {
                continue;
            };

            let mut seen = HashSet::new();

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == j && i == 0 {
                        continue;
                    }

                    let nx = x as i32 + i;
                    let ny = y as i32 + j;

                    let Some(Kind::Num(n)) = (nx >= 0 && ny >= 0)
                        .then(|| plan.get(nx as usize).and_then(|line| line.get(ny as usize)))
                        .flatten()
                        .copied()
                    else {
                        continue;
                    };

                    seen.insert(n);
                }
            }

            let mut seen = seen.into_iter();

            let (Some(a), Some(b), None) = (seen.next(), seen.next(), seen.next()) else {
                continue;
            };

            sum += a * b;
        }
    }

    sum
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Kind {
    Num(u32),
    Dot,
    Symbol(Symbol),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Symbol {
    Gear,
    Other,
}
