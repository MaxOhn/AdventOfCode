use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> i64 {
    solve(input.lines().map(|line| line.split(' ')))
}

fn part2(input: &str) -> i64 {
    solve(input.lines().map(|line| line.rsplit(' ')))
}

fn solve<'i, I, L>(input: I) -> i64
where
    I: Iterator<Item = L>,
    L: Iterator<Item = &'i str>,
{
    let mut curr = Vec::new();
    let mut next = Vec::new();
    let mut sum = 0;

    for line in input {
        curr.clear();
        curr.extend(line.map(str::parse::<i64>).flat_map(Result::ok));

        sum += curr[curr.len() - 1];

        while curr.iter().any(|&n| n != 0) {
            diffs(&curr, &mut next);
            std::mem::swap(&mut curr, &mut next);
            sum += curr[curr.len() - 1];
            next.clear();
        }
    }

    sum
}

fn diffs(from: &[i64], to: &mut Vec<i64>) {
    for window in from.windows(2) {
        to.push(window[1] - window[0])
    }
}
