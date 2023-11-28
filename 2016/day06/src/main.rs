use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let start = Instant::now();
    let mut columns = BTreeMap::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            *columns
                .entry(i)
                .or_insert_with(|| HashMap::with_capacity(26))
                .entry(c)
                .or_insert(0) += 1;
        }
    }

    println!("Setup: {:?}", start.elapsed());

    let start = Instant::now();
    let p1 = aoc16_day06::part1(&columns);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day06::part2(&columns);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, "qoclwvah");
    assert_eq!(p2, "ryrgviuv");
}
