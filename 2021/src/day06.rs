use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let nums: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u8>, _>>()?;

    let p1 = solve(80, &nums);
    let p2 = solve(256, &nums);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn solve(days: usize, nums: &[u8]) -> usize {
    let mut count = [0; 9];

    for &n in nums {
        count[n as usize] += 1;
    }

    for _ in 0..days {
        count.rotate_left(1);
        count[6] += count[8];
    }

    count.into_iter().sum()
}
