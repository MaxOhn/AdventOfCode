use aoc_rust::Solution;
use eyre::Result;
use itoa::Buffer;
use md5::Digest;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input, p1);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    solve(input, 1, |hash| hash.starts_with(&[0, 0]) && hash[2] < 8)
}

fn part2(input: &str, part1: u32) -> u32 {
    solve(input, part1, |hash| hash.starts_with(&[0, 0, 0]))
}

fn solve<F: Fn(&Digest) -> bool>(input: &str, start: u32, find: F) -> u32 {
    let mut int = Buffer::new();
    let mut buf = String::new();

    (start..)
        .find(|&n| {
            buf.clear();
            buf.push_str(input);
            buf.push_str(int.format(n));

            find(&md5::compute(&buf))
        })
        .unwrap()
}
