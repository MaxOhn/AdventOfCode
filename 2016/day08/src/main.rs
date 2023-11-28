use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let ops = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let start = Instant::now();
    let p1 = aoc16_day08::part1(&ops);
    println!("Part 1: {p1} [{:?}]", start.elapsed());

    assert_eq!(p1, 119);
}
