use std::{mem, str::Bytes};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = solve::<Part1>(input);
    let p2 = solve::<Part2>(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn solve<C: Check>(input: &str) -> u64 {
    let split = memchr::memchr(b'\n', &input[input.len() / 2..].as_bytes()).unwrap();
    let (first, second) = input.split_at(input.len() / 2 + split + 1);

    let compute = |input: &str| {
        let mut sum = 0;
        let mut bytes = input.bytes();
        let mut buf = Vec::new();

        while let Some(equation) = Equation::parse(&mut bytes, &mut buf) {
            if equation.check::<C>() {
                sum += equation.value;
            }
        }

        sum
    };

    let (first, second) = rayon::join(|| compute(first), || compute(second));

    first + second
}

#[derive(Debug)]
struct Equation<'a> {
    value: u64,
    nums: &'a [u64],
}

impl<'a> Equation<'a> {
    fn parse(bytes: &mut Bytes<'_>, buf: &'a mut Vec<u64>) -> Option<Self> {
        let digit = |byte| (byte & 0xF) as u64;

        let mut value = 0;

        loop {
            match bytes.next() {
                Some(byte @ b'0'..=b'9') => value = value * 10 + digit(byte),
                Some(b':') => break,
                None | Some(_) => return None,
            }
        }

        debug_assert_eq!(bytes.next(), Some(b' '));
        buf.clear();
        let mut curr = 0;

        loop {
            match bytes.next() {
                Some(byte @ b'0'..=b'9') => curr = curr * 10 + digit(byte),
                Some(b' ') => buf.push(mem::replace(&mut curr, 0)),
                Some(b'\n') | None => break,
                Some(_) => return None,
            }
        }

        buf.push(curr);

        Some(Self { value, nums: &*buf })
    }

    fn check<C: Check>(&self) -> bool {
        let [curr, rest @ ..] = self.nums else {
            return false;
        };

        C::recurse(self.value, *curr, rest)
    }
}

trait Check {
    fn check(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool;

    fn recurse(target: u64, curr: u64, rest: &[u64]) -> bool {
        let [next, rest @ ..] = rest else {
            return curr == target;
        };

        if curr > target {
            return false;
        }

        Self::check(target, curr, *next, rest)
    }
}

struct Part1;

impl Check for Part1 {
    fn check(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
        Self::recurse(target, curr + next, rest) || Self::recurse(target, curr * next, rest)
    }
}

struct Part2;

impl Check for Part2 {
    fn check(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
        Self::recurse(target, curr + next, rest)
            || Self::recurse(target, curr * next, rest)
            || Self::recurse(target, concat(curr, next), rest)
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    if b == 0 {
        return a * 10;
    }

    let mut shrink = b;

    while shrink > 0 {
        a *= 10;
        shrink /= 10;
    }

    a + b
}
