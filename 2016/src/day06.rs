use std::collections::{BTreeMap, HashMap};

use aoc_rust::Solution;

pub fn run(input: &str) -> eyre::Result<Solution> {
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

    Ok(Solution::new()
        .part1(part1(&columns))
        .part2(part2(&columns)))
}

type Columns = BTreeMap<usize, HashMap<char, usize>>;

pub fn part1(columns: &Columns) -> String {
    columns
        .iter()
        .map(|(_, counts)| counts.iter().max_by_key(|(_, count)| *count).unwrap().0)
        .collect()
}

pub fn part2(columns: &Columns) -> String {
    columns
        .iter()
        .map(|(_, counts)| counts.iter().min_by_key(|(_, count)| *count).unwrap().0)
        .collect()
}
