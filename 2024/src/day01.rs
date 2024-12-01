use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip()
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |sum, (left, right)| sum + (right - left).abs())
}

fn part2(input: &str) -> i32 {
    let (left, mut right) = parse(input);

    right.sort_unstable();

    left.iter().fold(0, |sum, &left| {
        let Ok(idx) = right.binary_search(&left) else {
            return sum;
        };

        let (to_left, to_right) = right.split_at(idx);
        let to_left = to_left.iter().rev().take_while(|&&n| n == left).count();
        let to_right = to_right.iter().take_while(|&&n| n == left).count();

        sum + left * (to_left + to_right) as i32
    })
}
