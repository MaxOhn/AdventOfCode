use std::{fs, time::Instant};

use eyre::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day03.txt")?;

    let start = Instant::now();
    let solution = aoc23::day03::run(&input)?;
    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
