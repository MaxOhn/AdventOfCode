use aoc_rust::Solution;
use eyre::{ContextCompat, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u32> {
    let mut sum = 0;

    for line in input.lines() {
        if let Some(matches) = parse_matches(line)?.checked_sub(1) {
            sum += 1 << matches;
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let matches = input
        .lines()
        .map(parse_matches)
        .collect::<Result<Vec<_>>>()
        .wrap_err("invalid cards")?;

    let mut counts = vec![1; matches.len()];

    for (i, matches) in matches.iter().enumerate() {
        for j in 1..=*matches as usize {
            counts[i + j] += counts[i];
        }
    }

    Ok(counts.iter().sum())
}

fn parse_matches(line: &str) -> Result<u8> {
    let (_, suffix) = line.split_once(':').wrap_err("missing colon")?;
    let (winning, owned) = suffix.split_once(" | ").wrap_err("missing split")?;

    let winning = winning
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<u8>, _>>()
        .wrap_err("invalid winning numbers")?;

    let owned = owned.split(' ').filter(|n| !n.is_empty()).map(str::parse);

    let mut matches = 0;

    for n in owned {
        matches += winning.contains(&n?) as u8;
    }

    Ok(matches)
}
