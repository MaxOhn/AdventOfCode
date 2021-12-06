use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

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

    let nums: Vec<u8> = line.split(',').map(|n| n.parse().unwrap()).collect();
    println!("Setup: {:?}", start.elapsed()); // 119µs

    let start = Instant::now();
    let p1 = solve(80, &nums);
    let elapsed = start.elapsed();
    println!("Part 1: {} [{:?}]", p1, elapsed); // 1µs

    let start = Instant::now();
    let p2 = solve(256, &nums);
    let elapsed = start.elapsed();
    println!("Part 2: {} [{:?}]", p2, elapsed); // 2.1µs

    assert_eq!(p1, 363_101);
    assert_eq!(p2, 1_644_286_074_024);

    Ok(())
}

fn solve(days: usize, nums: &[u8]) -> usize {
    let mut count = [0; 9];

    for &n in nums {
        count[n as usize] += 1;
    }

    for _ in 0..days {
        count.rotate_left(1);
        count[6] += count[8];
    }

    count.into_iter().sum()
}
