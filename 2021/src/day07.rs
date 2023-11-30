use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let mut min = i32::MAX;
    let mut max = 0;

    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .inspect(|&n| {
            min = min.min(n);
            max = max.max(n);
        })
        .collect();

    let p2 = part2(&nums, min, max);
    let p1 = part1(&mut nums);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &mut [i32]) -> i32 {
    input.sort_unstable();
    let pos = input[input.len() / 2];

    input.iter().map(|&n| (n - pos).abs()).sum()
}

fn part2(input: &[i32], min: i32, max: i32) -> i32 {
    (min..=max)
        .map(|pos| {
            input
                .iter()
                .map(|&n| (n - pos).abs())
                .fold(0, |fuel, diff| fuel + (diff * (diff + 1)) / 2)
        })
        .min()
        .unwrap_or(0)
}
