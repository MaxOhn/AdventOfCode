use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use util::Parse;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let (numbers, mut bingos) = parse_input()?;
    println!("Setup: {:?}", start.elapsed()); // 156µs

    let mut p1_bingos = bingos.clone();

    let start = Instant::now();
    let p1 = part1(&numbers, &mut p1_bingos);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 124µs

    let start = Instant::now();
    let p2 = part2(&numbers, &mut bingos);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 227µs

    assert_eq!(p1, 39_902);
    assert_eq!(p2, 26_936);

    Ok(())
}

fn parse_input() -> Result<(Vec<u8>, Vec<Bingo>), Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut numbers = String::new();
    input.read_line(&mut numbers)?;

    let numbers: Vec<u8> = numbers
        .split(',')
        .map(str::as_bytes)
        .map(|bytes| bytes.parse())
        .collect();

    let mut line = String::new();
    let mut bingos = Vec::new();

    while let Some(bingo) = Bingo::parse(&mut input, &mut line)? {
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
    fn parse(
        input: &mut BufReader<File>,
        line: &mut String,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let mut field = Vec::with_capacity(25);

        if input.read_line(line)? == 0 {
            return Ok(None);
        }

        for _ in 0..5 {
            line.clear();
            input.read_line(line)?;

            let row = line
                .trim_end()
                .split_whitespace()
                .map(str::as_bytes)
                .map(|bytes| <[u8] as Parse<u8>>::parse(bytes));

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
