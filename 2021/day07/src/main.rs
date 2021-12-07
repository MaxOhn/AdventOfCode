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
    let mut min = i32::MAX;
    let mut max = 0;

    let mut nums: Vec<i32> = line
        .split(',')
        .map(str::as_bytes)
        .map(|bytes| bytes.parse())
        .inspect(|&n| {
            min = min.min(n);
            max = max.max(n);
        })
        .collect();

    println!("Setup: [{:?}]", start.elapsed());

    let start = Instant::now();
    let p2 = part2(&nums, min, max);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); //

    let start = Instant::now();
    let p1 = part1(&mut nums);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); //

    assert_eq!(p1, 336_701);
    assert_eq!(p2, 95_167_302);

    Ok(())
}

fn part1(input: &mut [i32]) -> i32 {
    input.sort_unstable();
    let pos = input[input.len() / 2];

    input.iter().map(|&n| (n - pos).abs()).sum()

    // (min..=max)
    //     .map(|pos| input.iter().map(|&n| (n - pos).abs()).sum())
    //     .min()
    //     .unwrap_or(0)
}

fn part2(input: &[i32], min: i32, max: i32) -> i32 {
    (min..=max)
        .map(|pos| {
            input
                .iter()
                .map(|&n| (n - pos).abs())
                .fold(0, |fuel, diff| fuel + (diff * (diff + 1)) / 2)
        })
        .min()
        .unwrap_or(0)
}
