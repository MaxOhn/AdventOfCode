use std::ops::Index;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

struct Schematic<'i> {
    width: usize,
    height: usize,
    input: &'i [u8],
}

impl<'i> Schematic<'i> {
    fn new(input: &'i str) -> Self {
        Self {
            width: input.find('\n').unwrap_or(input.len()),
            height: input.lines().count(),
            input: input.as_bytes(),
        }
    }
}

impl Index<usize> for Schematic<'_> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let y = index * (self.width + 1);

        &self.input[y..][..self.width]
    }
}

fn part1(input: &str) -> u32 {
    let schematic = Schematic::new(input);

    let mut sum = 0;

    for (i, line) in input.lines().map(str::as_bytes).enumerate() {
        let mut start = 0;

        while start < line.len() - 1 {
            if !line[start].is_ascii_digit() {
                start += 1;

                continue;
            }

            let mut end = start + 1;

            while end < line.len() && line[end].is_ascii_digit() {
                end += 1;
            }

            let num = parse_num(&line[start..end]);

            let x_range_start = start.saturating_sub(1);
            let x_range_end = if end == line.len() { end - 1 } else { end };

            let y_range_start = i.saturating_sub(1);
            let y_range_end = if i == schematic.height - 1 { i } else { i + 1 };

            'neighbors: for ny in y_range_start..=y_range_end {
                for nx in x_range_start..=x_range_end {
                    let n = schematic[ny][nx];

                    if !n.is_ascii_digit() && n != b'.' {
                        sum += num;

                        break 'neighbors;
                    }
                }
            }

            start = end + 1;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let schematic = Schematic::new(input);

    let mut sum = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'*' {
                sum += handle_gear(&schematic, x, y);
            }
        }
    }

    sum
}

fn handle_gear(schematic: &Schematic<'_>, x: usize, y: usize) -> u32 {
    let mut first = None;
    let mut second = None;

    let x_range_start = x.saturating_sub(1);
    let x_range_end = if x == schematic.width - 2 { x } else { x + 1 };

    let y_range_start = y.saturating_sub(1);
    let y_range_end = if y == schematic.height - 1 { y } else { y + 1 };

    for ny in y_range_start..=y_range_end {
        let mut nx = x_range_start;

        while nx <= x_range_end {
            let bytes = &schematic[ny];
            let n = bytes[nx];

            if !n.is_ascii_digit() {
                nx += 1;

                continue;
            }

            let mut start = nx;
            let end = &mut nx;
            *end += 1;

            while start > 0 && bytes[start - 1].is_ascii_digit() {
                start -= 1;
            }

            while *end < bytes.len() && bytes[*end].is_ascii_digit() {
                *end += 1;
            }

            let num = parse_num(&bytes[start..*end]);

            if first.is_none() {
                first = Some(num);
            } else if second.is_none() {
                second = Some(num);
            } else {
                return 0;
            }
        }
    }

    first.zip(second).map_or(0, |(a, b)| a * b)
}

fn parse_num(slice: &[u8]) -> u32 {
    slice
        .iter()
        .fold(0, |n, &byte| n * 10 + (byte & 0xF) as u32)
}
