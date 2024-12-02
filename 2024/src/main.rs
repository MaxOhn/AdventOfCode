use std::{fs, time::Instant};

use eyre::Result;

fn main() -> Result<()> {
    macro_rules! load {
        ( $day:ident ) => {
            (
                concat!("./inputs/", stringify!($day), ".txt"),
                aoc24::$day::run,
            )
        };
    }

    let (path, run) = load!(day02);
    let input = fs::read_to_string(path)?;

    let start = Instant::now();
    let solution = run(&input)?;
    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
