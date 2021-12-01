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

    let mut p1 = 0;
    let mut p2 = 0;

    input.read_line(&mut line)?;
    let mut a: u16 = line.as_bytes().parse();
    line.clear();

    input.read_line(&mut line)?;
    let mut b: u16 = line.as_bytes().parse();
    line.clear();

    p1 += (b > a) as usize;

    input.read_line(&mut line)?;
    let mut c: u16 = line.as_bytes().parse();
    line.clear();

    p1 += (c > b) as usize;

    while input.read_line(&mut line)? != 0 {
        let curr = line.as_bytes().parse();
        p1 += (curr > c) as usize;
        p2 += (curr > a) as usize;
        a = b;
        b = c;
        c = curr;
        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 650µs

    assert_eq!(p1, 1559);
    assert_eq!(p2, 1600);

    Ok(())
}
