use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let ops: Vec<_> = input.lines().map(str::parse).map(Result::unwrap).collect();

    let start = Instant::now();
    let p1 = aoc16_day12::part1(&ops);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day12::part2(&ops);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 318_117);
    assert_eq!(p2, 9_227_771);
}
