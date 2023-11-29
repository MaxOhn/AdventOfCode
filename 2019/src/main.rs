use aoc19::*;
use std::time::Instant;
use std::{env, fs, process::exit};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("  - caused by: {}", source);
            e = source;
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let days = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];

    let day: i32 = if let Some(arg) = env::args().nth(1) {
        arg.parse().unwrap_or(*days.last().unwrap())
    } else {
        *days.last().unwrap()
    };
    let input =
        fs::read_to_string(format!("inputs/day{:02}.txt", day)).expect("Error: Invalid day");
    let input = input.as_str();

    let start = Instant::now();
    let solution = match day {
        1 => day01::solve(input)?.to_string(),
        2 => day02::solve(input)?.to_string(),
        3 => day03::solve(input)?.to_string(),
        4 => day04::solve(input)?.to_string(),
        5 => day05::solve(input)?.to_string(),
        6 => day06::solve(input)?.to_string(),
        7 => day07::solve(input)?.to_string(),
        8 => day08::solve(input)?.to_string(),
        9 => day09::solve(input)?.to_string(),
        10 => day10::solve(input)?.to_string(),
        11 => day11::solve(input)?.to_string(),
        12 => day12::solve(input)?.to_string(),
        13 => day13::solve(input)?.to_string(),
        14 => day14::solve(input)?.to_string(),
        15 => day15::solve(input)?.to_string(),
        16 => day16::solve(input)?.to_string(),
        17 => day17::solve(input)?.to_string(),
        18 => day18::solve(input)?.to_string(),
        19 => day19::solve(input)?.to_string(),
        20 => day20::solve(input)?.to_string(),
        21 => day21::solve(input)?.to_string(),
        22 => day22::solve(input)?.to_string(),
        23 => day23::solve(input)?.to_string(),
        24 => day24::solve(input)?.to_string(),
        25 => day25::solve(input)?.to_string(),
        d if 0 < d && d < 26 => bail!("Day {} not yet implemented", d),
        _ => bail!("Invalid day"),
    };
    println!(
        "[Day {}] Elapsed time: {:?}\n{}",
        day,
        Instant::now().checked_duration_since(start).unwrap(),
        solution,
    );
    Ok(())
}
