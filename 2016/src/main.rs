use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./inputs/day03.txt").unwrap();

    let start = Instant::now();
    let p1 = aoc16::day03::part1(&input);
    println!("Part 1: {p1} [{:?}]", start.elapsed());

    let start = Instant::now();
    let p2 = aoc16::day03::part2(&input);
    println!("Part 2: {p2} [{:?}]", start.elapsed());

    assert_eq!(p1, 983);
    assert_eq!(p2, 1836);
}
