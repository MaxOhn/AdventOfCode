use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let start = Instant::now();
    let solution = aoc16_day16::run(&input).unwrap();
    let elapsed = start.elapsed();

    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
    println!("Elapsed: {elapsed:?}");

    assert_eq!(solution.part1, "11111000111110000");
    assert_eq!(solution.part2, "10111100110110100");
}
