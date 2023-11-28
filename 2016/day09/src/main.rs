use std::time::Instant;

fn main() {
    let bytes = std::fs::read("./input.txt").unwrap();

    let start = Instant::now();
    let p1 = aoc16_day09::part1(&bytes);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day09::part2(&bytes);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 120_765);
    assert_eq!(p2, 11_658_395_076);
}
