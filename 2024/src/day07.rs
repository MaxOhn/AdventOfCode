use std::{
    cell::RefCell,
    iter, mem,
    ops::{Add, Mul},
};

use aoc_rust::Solution;
use eyre::Result;
use rayon::{prelude::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1_recursive(input);
    let p2 = part2_recursive(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn part1_recursive(input: &str) -> u64 {
    solve_recursive::<Part1>(input)
}

pub fn part2_recursive(input: &str) -> u64 {
    solve_recursive::<Part2>(input)
}

pub fn part1_dynamic(input: &str) -> u64 {
    solve_dynamic::<Part1>(input)
}

pub fn part2_dynamic(input: &str) -> u64 {
    solve_dynamic::<Part2>(input)
}

#[derive(Default)]
struct ThreadData {
    eq_buf: Vec<u64>,
    dp: Vec<u64>,
}

struct Dp<'a>(&'a mut Vec<u64>);

thread_local! {
    static DATA: RefCell<ThreadData> = RefCell::new(ThreadData::default());
}

fn solve_recursive<C: Check>(input: &str) -> u64 {
    solve(input, |eq, _| eq.check_recursive::<C>())
}

fn solve_dynamic<C: Check>(input: &str) -> u64 {
    solve(input, |eq, Dp(dp)| eq.check_dynamic::<C>(dp))
}

fn solve(input: &str, check_fn: fn(&Equation<'_>, Dp) -> bool) -> u64 {
    input
        .par_lines()
        .map(|line| {
            DATA.with_borrow_mut(|data| {
                let ThreadData { eq_buf, dp } = data;
                let eq = Equation::parse(line, eq_buf);

                if check_fn(&eq, Dp(dp)) {
                    eq.value
                } else {
                    0
                }
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

    fn check_recursive<C: Check>(&self) -> bool {
        let [curr, rest @ ..] = self.nums else {
            return false;
        };

        C::recurse(self.value, *curr, rest)
    }

    fn check_dynamic<C: Check>(&self, dp: &mut Vec<u64>) -> bool {
        C::check_dynamic(self.value, self.nums, dp)
    }
}

struct Part1;

impl Check for Part1 {
    const OPS: &[fn(u64, u64) -> u64] = &[u64::add, u64::mul];
}

struct Part2;

impl Check for Part2 {
    const OPS: &[fn(u64, u64) -> u64] = &[u64::add, u64::mul, concat];
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

trait Check {
    const OPS: &[fn(u64, u64) -> u64];
    const N: usize = Self::OPS.len();

    fn check_recursive(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
        Self::OPS
            .iter()
            .any(|op| Self::recurse(target, op(curr, next), rest))
    }

    fn recurse(target: u64, curr: u64, rest: &[u64]) -> bool {
        let [next, rest @ ..] = rest else {
            return curr == target;
        };

        if curr > target {
            return false;
        }

        Self::check_recursive(target, curr, *next, rest)
    }

    fn check_dynamic(eq_value: u64, eq_nums: &[u64], dp: &mut Vec<u64>) -> bool {
        let mut nums = eq_nums.iter().copied();

        let Some(first) = nums.next() else {
            return false;
        };

        let Some(last_n) = nums.next_back() else {
            return eq_value == first;
        };

        let required = (0..eq_nums.len() as u32 - 1)
            .map(|exp| Self::N.pow(exp))
            .sum();

        if dp.len() < required {
            dp.extend(iter::repeat(0).take(required - dp.len()));
        }

        dp[0] = first;

        let mut prev = 0;
        let mut curr = 1;
        let mut len = Self::N;

        let zip = |curr, len| (curr..curr + len).zip(Self::OPS.iter().cycle());
        let idx = |prev, j, curr| prev + (j - curr) / Self::N;

        for n in nums {
            for (j, op) in zip(curr, len) {
                dp[j] = op(dp[idx(prev, j, curr)], n);
            }

            prev = curr;
            curr += len;
            len *= Self::N;
        }

        zip(curr, len).any(|(j, op)| op(dp[idx(prev, j, curr)], last_n) == eq_value)
    }
}
