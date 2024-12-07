use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = solve::<Part1>(input);
    let p2 = solve::<Part2>(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn solve<C: Check>(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let (a, b) = line.split_once(": ").unwrap();
        let a = a.parse().unwrap();
        let b: Vec<_> = b.split(' ').map(str::parse).map(Result::unwrap).collect();

        if C::check(a, &b) {
            sum += a;
        }
    }

    sum
}

trait Check {
    fn ops(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool;

    fn check(res: u64, vals: &[u64]) -> bool {
        let [init, rest @ ..] = vals else {
            return false;
        };

        Self::recurse(res, *init, rest)
    }

    fn recurse(target: u64, curr: u64, rest: &[u64]) -> bool {
        let [next, rest @ ..] = rest else {
            return curr == target;
        };

        if curr > target {
            return false;
        }

        Self::ops(target, curr, *next, rest)
    }
}

struct Part1;

impl Check for Part1 {
    fn ops(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
        Self::recurse(target, curr + next, rest) || Self::recurse(target, curr * next, rest)
    }
}

struct Part2;

impl Check for Part2 {
    fn ops(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
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
