use std::str::Lines;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let (numbers, mut bingos) = parse_input(input)?;

    let mut p1_bingos = bingos.clone();

    let p1 = part1(&numbers, &mut p1_bingos);
    let p2 = part2(&numbers, &mut bingos);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<Bingo>)> {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u8>, _>>()?;

    let mut bingos = Vec::new();

    while let Some(bingo) = Bingo::parse(&mut lines)? {
        bingos.push(bingo);
    }

    Ok((numbers, bingos))
}

fn part1(numbers: &[u8], bingos: &mut [Bingo]) -> u32 {
    for &n in numbers {
        for bingo in bingos.iter_mut() {
            if bingo.mark(n) {
                return bingo.sum() * n as u32;
            }
        }
    }

    unreachable!()
}

fn part2(numbers: &[u8], bingos: &mut Vec<Bingo>) -> u32 {
    let mut numbers = numbers.iter();

    for &n in &mut numbers {
        let mut marked = false;

        for bingo in bingos.iter_mut() {
            marked |= bingo.mark(n);
        }

        if marked {
            bingos.retain(|bingo| !bingo.is_done());

            if bingos.len() == 1 {
                break;
            }
        }
    }

    let bingo = &mut bingos[0];

    for &n in numbers {
        if bingo.mark(n) {
            return bingo.sum() * n as u32;
        }
    }

    unreachable!()
}

#[derive(Clone)]
struct Bingo {
    field: Box<[u8]>,
    marked: u32,
}

#[allow(clippy::unusual_byte_groupings)]
const DONE: [u32; 10] = [
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
];

impl Bingo {
    fn parse(lines: &mut Lines<'_>) -> Result<Option<Self>> {
        let mut field = Vec::with_capacity(25);

        if lines.next().is_none() {
            return Ok(None);
        }

        for line in lines.take(5) {
            let row = line
                .trim_end()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap());

            field.extend(row);
        }

        let field = field.into_boxed_slice();

        Ok(Some(Self { field, marked: 0 }))
    }

    fn mark(&mut self, n: u8) -> bool {
        if let Some(idx) = self.field.iter().position(|&elem| elem == n) {
            self.marked |= 1 << idx;

            self.is_done()
        } else {
            false
        }
    }

    fn is_done(&self) -> bool {
        DONE.iter().any(|&mask| self.marked & mask == mask)
    }

    fn sum(&self) -> u32 {
        self.field
            .iter()
            .enumerate()
            .filter_map(|(i, n)| (self.marked & (1 << i) == 0).then(|| *n as u32))
            .sum()
    }
}
