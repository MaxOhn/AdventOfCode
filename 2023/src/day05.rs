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
            // false positive, a vec of a single range really is what's required
            #[allow(clippy::single_range_in_vec_init)]
            let ranges = vec![chunk[0]..chunk[0] + chunk[1]];

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
            .find_map(|entry| entry.try_map(value))
            .unwrap_or(value)
    }

    fn apply_range(&self, mut ranges: Vec<Range<u64>>, bufs: &mut Buffers) -> Vec<Range<u64>> {
        let Buffers {
            new_ranges,
            final_ranges,
        } = bufs;

        for Entry { dst, src } in self.entries.iter() {
            for range in ranges.drain(..) {
                let before = range.start..cmp::min(range.end, src.start);
                let inter = cmp::max(range.start, src.start)..cmp::min(src.end, range.end);
                let after = cmp::max(src.end, range.start)..range.end;

                if !before.is_empty() {
                    new_ranges.push(before);
                }

                if !inter.is_empty() {
                    let from = inter.start - src.start + dst;
                    let to = inter.end - src.start + dst;
                    final_ranges.push(from..to);
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
    src: Range<u64>,
}

impl Entry {
    fn try_map(&self, value: u64) -> Option<u64> {
        // the expression can underflow if evaluated eagerly
        #[allow(clippy::unnecessary_lazy_evaluations)]
        self.src
            .contains(&value)
            .then(|| self.dst + value - self.src.start)
    }
}

impl FromStr for Entry {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split(' ').map(str::parse).flat_map(Result::ok);

        split
            .next()
            .zip(split.next())
            .zip(split.next())
            .map(|((dst, src), len)| Self {
                dst,
                src: src..src + len,
            })
            .wrap_err("invalid entry value")
    }
}
