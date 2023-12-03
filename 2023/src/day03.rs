use std::collections::HashSet;

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

fn part1(input: &str) -> u32 {
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
            } else {
                nums.push(Kind::Symbol(Symbol::Other));
                i += 1;
            }
        }

        plan.push(nums);
    }

    let mut sum = 0;

    let mut x = 0;

    while x < plan.len() {
        // println!("{:?}", plan[x]);

        let mut y = 0;

        while y < plan[x].len() {
            let Kind::Num(curr) = plan[x][y] else {
                y += 1;
                continue;
            };

            let mut end = y + 1;

            while end < plan[x].len() && plan[x][end] == Kind::Num(curr) {
                end += 1;
            }

            // println!("{curr}: {y}..{end}");

            'done: for yp in y..end {
                // println!("{curr} >> {y}");

                for i in [-1, 0, 1] {
                    for j in [-1, 0, 1] {
                        if i == j && i == 0 {
                            continue;
                        }

                        let nx = x as i32 + i;
                        let ny = yp as i32 + j;

                        let Some(n) = (nx >= 0 && ny >= 0)
                            .then(|| plan.get(nx as usize).and_then(|line| line.get(ny as usize)))
                            .flatten()
                            .copied()
                        else {
                            continue;
                        };

                        // println!("{curr} => {nx},{ny}");

                        if matches!(n, Kind::Symbol(_)) {
                            // println!("{curr} because ({x},{yp})-({nx},{ny})");
                            sum += curr;

                            break 'done;
                        }
                    }
                }
            }

            y = end;
        }

        x += 1;
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
