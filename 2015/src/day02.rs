use std::str::FromStr;

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let dims = input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Dimensions>>>()?;

    let p1 = part1(&dims);
    let p2 = part2(&dims);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(dims: &[Dimensions]) -> u32 {
    let mut sum = 0;

    for Dimensions { l, w, h } in dims {
        let a = l * w;
        let b = w * h;
        let c = l * h;

        let min = a.min(b).min(c);

        sum += 2 * (a + b + c) + min;
    }

    sum
}

fn part2(dims: &[Dimensions]) -> u32 {
    let mut sum = 0;

    for Dimensions { l, w, h } in dims {
        let a = 2 * (l + w);
        let b = 2 * (w + h);
        let c = 2 * (l + h);

        let min = a.min(b).min(c);

        sum += min + l * w * h;
    }

    sum
}

struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Dimensions {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut dims = line.split('x').map(str::parse).flat_map(Result::ok);

        Ok(Self {
            l: dims.next().wrap_err("invalid length")?,
            w: dims.next().wrap_err("invalid width")?,
            h: dims.next().wrap_err("invalid height")?,
        })
    }
}
