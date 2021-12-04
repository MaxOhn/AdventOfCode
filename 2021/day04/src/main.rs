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

    println!("Setup: {:?}", start.elapsed()); // 195µs

    let mut p1_bingos = bingos.clone();

    let start = Instant::now();
    let p1 = part1(&numbers, &mut p1_bingos);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 200µs

    let start = Instant::now();
    let p2 = part2(&numbers, &mut bingos);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 570µs

    assert_eq!(p1, 39_902);
    assert_eq!(p2, 26_936);

    Ok(())
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
            bingos.retain(|bingo| !bingo.done());

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
    field: Box<[Option<u8>]>,
}

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
                .map(|bytes| bytes.parse())
                .map(Some);

            field.extend(row);
        }

        let field = field.into_boxed_slice();

        Ok(Some(Self { field }))
    }

    fn mark(&mut self, n: u8) -> bool {
        let mut found = false;

        for elem in self.field.iter_mut() {
            if *elem == Some(n) {
                elem.take();
                found = true;
            }
        }

        if found {
            self.done()
        } else {
            false
        }
    }

    fn done(&self) -> bool {
        for chunk in self.field.chunks_exact(5) {
            if chunk.iter().all(Option::is_none) {
                return true;
            }
        }

        'outer: for col in 0..5 {
            for row in 0..5 {
                if self.field[row * 5 + col].is_some() {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }

    fn sum(&self) -> u32 {
        self.field.iter().flatten().map(|&n| n as u32).sum()
    }
}
