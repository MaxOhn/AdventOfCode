#![allow(clippy::many_single_char_names)]

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::hint::unreachable_unchecked;

use aoc_rust::util::int_hasher::IntHasher;
use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u16 {
    let mut rules = Vec::with_capacity(4);

    let mut add_rule = |a, b| {
        let mut not_added = true;

        if let Some((x, _)) = rules.iter_mut().find(|(x, _)| a < *x && b >= *x) {
            *x = a;
            not_added = false;
        }

        if let Some((_, y)) = rules.iter_mut().find(|(_, y)| a <= *y && b > *y) {
            *y = b;
            return;
        }

        if not_added {
            rules.push((a, b));
        }
    };

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.len() <= 1 {
            break;
        }

        let bytes = line.as_bytes();
        let mut j = 0;

        while get!(bytes, j) != b':' {
            j += 1;
        }

        let read_num = |j: &mut usize, end| {
            let mut n = 0;
            let mut byte;

            while *j < bytes.len() {
                byte = unsafe { *bytes.get_unchecked(*j) };

                if byte == end {
                    return n;
                }

                n = n * 10 + (byte & 0x0F) as u16;
                *j += 1;
            }

            n
        };

        j += 2;
        let a = read_num(&mut j, b'-');

        j += 1;
        let b = read_num(&mut j, b' ');

        j += 4;
        let c = read_num(&mut j, b'-');

        j += 1;
        let d = read_num(&mut j, b'\n');

        add_rule(a, b);
        add_rule(c, d);
    }

    lines.next();
    lines.next();
    lines.next();
    lines.next();

    let mut x = 0;

    while x < rules.len() - 1 {
        let (mut x1, mut x2) = get!(rules, x);
        let mut y = x + 1;

        while y < rules.len() {
            let (y1, y2) = get!(rules, y);

            if x1 == y1 {
                if x2 < y2 {
                    rules.remove(x);
                    x1 = y1;
                    x2 = y2;
                } else {
                    rules.remove(y);
                }
            } else {
                y += 1;
            }
        }

        x += 1;
    }

    let check = |n: u16| rules.iter().any(|&(a, b)| a <= n && n <= b);

    let mut p1 = 0;

    for line in lines {
        let bytes = line.as_bytes();

        let mut n = 0;
        let mut i = 0;

        while i < bytes.len() {
            match unsafe { *bytes.get_unchecked(i) } {
                b'\n' => {
                    p1 += (!check(n) as u16) * n;
                    break;
                }
                b',' => {
                    p1 += (!check(n) as u16) * n;
                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        p1 += ((i == bytes.len() && !check(n)) as u16) * n;
    }

    p1
}

fn part2(input: &str) -> u64 {
    let mut rules = Vec::with_capacity(16);

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.len() <= 1 {
            break;
        }

        let bytes = line.as_bytes();
        let mut j = 0;

        while unsafe { *bytes.get_unchecked(j) } != b':' {
            j += 1;
        }

        let name = unsafe { bytes.get_unchecked(..j.min(12)) };

        let read_num = |j: &mut usize, end| {
            let mut n = 0;
            let mut byte;

            while *j < bytes.len() {
                byte = unsafe { *bytes.get_unchecked(*j) };

                if byte == end {
                    return n;
                }

                n = n * 10 + (byte & 0x0F) as u16;

                *j += 1;
            }

            n
        };

        j += 2;
        let a = read_num(&mut j, b'-');

        j += 1;
        let b = read_num(&mut j, b' ');

        j += 4;
        let c = read_num(&mut j, b'-');

        j += 1;
        let d = read_num(&mut j, b'\n');

        rules.push(Rule::new(name, a, b, c, d));
    }

    lines.next();

    let ticket = {
        let bytes = lines.next().unwrap().as_bytes();
        let mut i = 0;
        let mut n = 0;

        let mut nums = Vec::with_capacity(rules.len());

        while i < bytes.len() {
            match get!(bytes, i) {
                b'\n' => nums.push(n),
                b',' => {
                    nums.push(n);
                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        nums
    };

    lines.next();
    lines.next();

    let check = |n: u16| rules.iter().any(|rule| rule.contains(n));

    let mut possibs = HashMap::with_capacity(rules.len());

    for rule in rules.iter() {
        let mut set = HashSet::with_capacity_and_hasher(rules.len(), IntHasher);
        set.extend(0..rules.len() as u8);
        possibs.insert(rule, set);
    }

    let mut row = Vec::with_capacity(rules.len());

    'outer: for line in lines {
        let bytes = line.as_bytes();

        let mut n = 0;
        let mut i = 0;

        while i < bytes.len() {
            match get!(bytes, i) {
                b'\n' => {
                    if check(n) {
                        row.push(n);
                    } else {
                        row.clear();
                        continue 'outer;
                    }
                }
                b',' => {
                    if check(n) {
                        row.push(n);
                    } else {
                        row.clear();
                        continue 'outer;
                    }

                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        if i == bytes.len() {
            if check(n) {
                row.push(n);
            } else {
                row.clear();
                continue 'outer;
            }
        }

        let mut i = 0;

        while i < row.len() {
            for rule in rules.iter() {
                if !rule.contains(get!(row, i)) {
                    possibs
                        .get_mut(rule)
                        .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                        .remove(&(i as u8));
                }
            }

            i += 1;
        }

        row.clear();
    }

    let mut possibs: Vec<_> = possibs.into_iter().collect();
    possibs.sort_by_key(|(_, i)| std::cmp::Reverse(i.len()));

    let mut p2 = 1;

    while let Some((rule, rules)) = possibs.pop() {
        let val = rules
            .into_iter()
            .next()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        for (_, rules) in possibs.iter_mut() {
            rules.remove(&val);
        }

        if rule.name.starts_with("departure") {
            p2 *= unsafe { *ticket.get_unchecked(val as usize) } as u64;
        }
    }

    p2
}

struct Rule {
    name: String,
    a: u16,
    b: u16,
    c: u16,
    d: u16,
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
            && self.b == other.b
            && self.c == other.c
            && self.d == other.d
            && self.name.eq(&other.name)
    }
}

impl Eq for Rule {}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}-{} or {}-{}",
            self.name, self.a, self.b, self.c, self.d
        )
    }
}

impl Rule {
    fn new(name: &[u8], a: u16, b: u16, c: u16, d: u16) -> Self {
        let name = String::from_utf8_lossy(name).to_string();
        Self { name, a, b, c, d }
    }

    fn contains(&self, n: u16) -> bool {
        (n >= self.a && n <= self.b) || (n >= self.c && n <= self.d)
    }
}
