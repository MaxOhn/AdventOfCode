use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let start = Instant::now();
    let p1 = aoc16_day05::part1(&input);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day05::part2(&input);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, "c6697b55");
    assert_eq!(p2, "8c35d1ab");
}
