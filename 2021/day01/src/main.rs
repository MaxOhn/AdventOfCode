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

    let mut line = String::new();

    input.read_line(&mut line)?;
    let a = line.as_bytes().parse();
    line.clear();

    let mut p1 = Part1::new(a);

    input.read_line(&mut line)?;
    let b = line.as_bytes().parse();
    line.clear();

    p1.process(b);

    input.read_line(&mut line)?;
    let c = line.as_bytes().parse();
    line.clear();

    p1.process(c);
    let mut p2 = Part2::new(a, b, c);

    while input.read_line(&mut line)? != 0 {
        let curr = line.as_bytes().parse();
        p1.process(curr);
        p2.process(curr);
        line.clear();
    }

    println!("Part 1: {}", p1.count);
    println!("Part 2: {}", p2.count);
    println!("Elapsed: {:?}", start.elapsed()); // 700µs

    assert_eq!(p1.count, 1559);
    assert_eq!(p2.count, 1600);

    Ok(())
}

struct Part1 {
    prev: u16,
    count: usize,
}

impl Part1 {
    fn new(prev: u16) -> Self {
        Self { prev, count: 0 }
    }

    fn process(&mut self, curr: u16) {
        self.count += (curr > self.prev) as usize;
        self.prev = curr;
    }
}

struct Part2 {
    a: u16,
    b: u16,
    c: u16,
    count: usize,
}

impl Part2 {
    fn new(a: u16, b: u16, c: u16) -> Self {
        Self { a, b, c, count: 0 }
    }

    fn process(&mut self, next: u16) {
        let prev = self.a + self.b + self.c;
        let curr = self.b + self.c + next;

        self.count += (curr > prev) as usize;

        self.a = self.b;
        self.b = self.c;
        self.c = next;
    }
}
