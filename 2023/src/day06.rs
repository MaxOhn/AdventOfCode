use std::num::ParseIntError;

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u32> {
    let (time, dist) = input.split_once('\n').wrap_err("invalid input lines")?;

    fn parse_line<'l>(
        line: &'l str,
        prefix: &str,
    ) -> impl Iterator<Item = Result<u32, ParseIntError>> + 'l {
        line.trim_start_matches(prefix)
            .trim_start()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse::<u32>)
    }

    let times = parse_line(time, "Time:");
    let dists = parse_line(dist, "Distance:");

    times
        .zip(dists)
        .map(|(time, dist)| {
            let time = time?;
            let dist = dist?;

            let count = (1..(time + 1) / 2)
                .find_map(|i| (i * (time - i) > dist).then_some(time - 2 * i + 1))
                .unwrap_or(0);

            Ok(count)
        })
        .try_fold(1, |prod, n| Ok::<_, ParseIntError>(prod * n?))
        .wrap_err("invalid integer")
}

fn part2(input: &str) -> Result<u64> {
    let (time, dist) = input.split_once('\n').wrap_err("invalid input lines")?;

    fn parse_line<'l>(line: &'l str, prefix: &str) -> Result<u64> {
        line.trim_start_matches(prefix)
            .trim_start()
            .bytes()
            .filter_map(|byte| match byte {
                b' ' => None,
                b'0'..=b'9' => Some(Ok(byte)),
                _ => Some(Err(eyre::eyre!("invalid digit"))),
            })
            .try_fold(0, |n, byte| Ok::<_, Report>(n * 10 + (byte? & 0xF) as u64))
    }

    let time = parse_line(time, "Time:").wrap_err("invalid time")?;
    let dist = parse_line(dist, "Distance:").wrap_err("invalid dist")?;

    let count = (1..(time + 1) / 2)
        .find_map(|i| (i * (time - i) > dist).then_some(time - 2 * i + 1))
        .unwrap_or(0);

    Ok(count)
}
