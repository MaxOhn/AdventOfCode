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

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    while input.read_line(&mut line)? != 0 {
        let mut split = line.split(' ').map(str::as_bytes);
        let dir = split.next().unwrap()[0];
        let val: i64 = split.next().unwrap().parse();

        match dir {
            b'f' => {
                horizontal += val;
                depth += aim * val;
            }
            b'd' => aim += val,
            b'u' => aim -= val,
            _ => unreachable!(),
        }

        line.clear();
    }

    let p1 = horizontal * aim;
    let p2 = horizontal * depth;
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 135µs

    assert_eq!(p1, 1_694_130);
    assert_eq!(p2, 1_698_850_445);

    Ok(())
}
