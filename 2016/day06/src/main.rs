use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

type Columns = BTreeMap<usize, HashMap<char, usize>>;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

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
    let p1 = part1(&columns);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&columns);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, "qoclwvah");
    assert_eq!(p2, "ryrgviuv");
}

fn part1(columns: &Columns) -> String {
    columns
        .iter()
        .map(|(_, counts)| counts.iter().max_by_key(|(_, count)| *count).unwrap().0)
        .collect()
}

fn part2(columns: &Columns) -> String {
    columns
        .iter()
        .map(|(_, counts)| counts.iter().min_by_key(|(_, count)| *count).unwrap().0)
        .collect()
}
