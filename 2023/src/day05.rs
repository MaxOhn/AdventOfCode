use std::{cmp, mem, ops::Range, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let (seeds, maps) = parse_input(input)?;

    let p1 = part1(&seeds, &maps);
    let p2 = part2(&seeds, &maps);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<Map>)> {
    let mut split = input.split("\n\n");

    let seeds = split
        .next()
        .wrap_err("missing seeds line")?
        .strip_prefix("seeds: ")
        .wrap_err("invalid seeds prefix")?
        .split(' ')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .wrap_err("invalid seed value")?;

    let maps = split
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("invalid map")?;

    Ok((seeds, maps))
}

fn part1(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |curr, map| map.apply_one(curr)))
        .min()
        .unwrap_or(u64::MAX)
}

fn part2(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut bufs = Buffers::default();

    seeds
        .chunks_exact(2)
        .filter_map(|chunk| {
            let start = chunk[0];
            let end = start + chunk[1];

            #[allow(clippy::single_range_in_vec_init)]
            let ranges = vec![start..end];

            maps.iter()
                .fold(ranges, |ranges, map| map.apply_range(ranges, &mut bufs))
                .iter()
                .map(|range| range.start)
                .min()
        })
        .min()
        .unwrap_or(u64::MAX)
}

#[derive(Default)]
struct Buffers {
    new_ranges: Vec<Range<u64>>,
    final_ranges: Vec<Range<u64>>,
}

struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn apply_one(&self, value: u64) -> u64 {
        self.entries
            .iter()
            .find(|entry| entry.contains(value))
            .map_or(value, |entry| entry.dst + value - entry.src)
    }

    fn apply_range(&self, mut ranges: Vec<Range<u64>>, bufs: &mut Buffers) -> Vec<Range<u64>> {
        let Buffers {
            new_ranges,
            final_ranges,
        } = bufs;

        for &Entry { dst, src, len } in self.entries.iter() {
            let src_end = src + len;

            for range in ranges.drain(..) {
                let before = range.start..cmp::min(range.end, src);
                let inter = cmp::max(range.start, src)..cmp::min(src_end, range.end);
                let after = cmp::max(src_end, range.start)..range.end;

                if !before.is_empty() {
                    new_ranges.push(before);
                }

                if !inter.is_empty() {
                    let mapped = inter.start - src + dst..inter.end - src + dst;
                    final_ranges.push(mapped);
                }

                if !after.is_empty() {
                    new_ranges.push(after);
                }
            }

            mem::swap(&mut ranges, new_ranges);
        }

        ranges.append(final_ranges);

        ranges
    }
}

impl FromStr for Map {
    type Err = Report;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        section
            .lines()
            .skip(1)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(|entries| Self { entries })
            .wrap_err("invalid entry")
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
        let mut split = line.split(' ').map(str::parse).flat_map(Result::ok);

        let ((dst, src), len) = split
            .next()
            .zip(split.next())
            .zip(split.next())
            .wrap_err("invalid map entry")?;

        Ok(Self { dst, src, len })
    }
}
