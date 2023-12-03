use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./inputs/day03.txt").unwrap();

    let start = Instant::now();
    let solution = aoc15::day03::run(&input);
    let elapsed = start.elapsed();

    print!("{}", solution.unwrap());
    println!("Elapsed: {elapsed:?}");
}
