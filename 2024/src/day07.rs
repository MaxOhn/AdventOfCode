use std::{cell::RefCell, mem};

use aoc_rust::Solution;
use eyre::Result;
use rayon::{prelude::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = solve::<Part1>(input);
    let p2 = solve::<Part2>(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

thread_local! {
    static BUF: RefCell<Vec<u64>> = RefCell::new(Vec::new());
}

fn solve<C: Check>(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| {
            BUF.with_borrow_mut(|buf| {
                let eq = Equation::parse(line, buf);

                eq.check::<C>().then_some(eq.value).unwrap_or(0)
            })
        })
        .sum()
}

struct Equation<'a> {
    value: u64,
    nums: &'a [u64],
}

impl<'a> Equation<'a> {
    fn parse(line: &str, buf: &'a mut Vec<u64>) -> Self {
        let digit = |byte| (byte & 0xF) as u64;

        let mut bytes = line.bytes();
        let mut value = 0;

        loop {
            match bytes.next() {
                Some(byte @ b'0'..=b'9') => value = value * 10 + digit(byte),
                Some(b':') => break,
                None | Some(_) => unreachable!(),
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
                Some(_) => unreachable!(),
            }
        }

        buf.push(curr);

        Self { value, nums: &*buf }
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
