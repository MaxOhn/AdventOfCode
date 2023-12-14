use std::{fs, time::Instant};

use eyre::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day21.txt")?;

    let start = Instant::now();
    let solution = aoc17::day21::run(&input)?;
    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
