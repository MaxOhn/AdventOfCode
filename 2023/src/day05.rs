use std::str::FromStr;

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let (seeds, maps) = parse_input(input)?;

    let p1 = part1(&seeds, &maps);
    let p2 = part2(&seeds, &maps);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .iter()
        .map(|seed| seed_to_location(*seed, maps))
        .min()
        .unwrap_or(u64::MAX)
}

fn part2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .par_chunks_exact(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| seed_to_location(seed, maps))
        .min()
        .unwrap_or(u64::MAX)
}

fn seed_to_location(seed: u64, maps: &[Map]) -> u64 {
    maps.iter().fold(seed, |curr, map| {
        map.find(curr)
            .map_or(curr, |entry| entry.dst + curr - entry.src)
    })
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<Map>)> {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .wrap_err("missing seeds line")?
        .strip_prefix("seeds: ")
        .wrap_err("invalid seeds prefix")?
        .split(' ')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .wrap_err("invalid seed value")?;

    let mut curr = "seed";

    let mut maps = Vec::new();

    lines.next();

    while let Some(line) = lines.next() {
        let mut split = line
            .strip_suffix(" map:")
            .wrap_err("invalid map header")?
            .split('-');

        let from = split.next().wrap_err("missing `from` part")?;
        ensure!(curr == from, "maps are not in order");
        curr = split.nth(1).wrap_err("missing `to` part")?;

        let mut entries = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            entries.push(line.parse()?);
        }

        maps.push(Map { entries });
    }

    Ok((seeds, maps))
}

struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn find(&self, value: u64) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.contains(value))
    }
}

struct Entry {
    dst: u64,
    src: u64,
    len: u64,
}

impl Entry {
    fn contains(&self, value: u64) -> bool {
        self.src <= value && self.src + self.len > value
    }
}

impl FromStr for Entry {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split(' ').map(str::parse).map(Result::ok).flatten();

        let ((dst, src), len) = split
            .next()
            .zip(split.next())
            .zip(split.next())
            .wrap_err("invalid map entry")?;

        Ok(Self { dst, src, len })
    }
}
