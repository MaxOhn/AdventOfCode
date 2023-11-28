use std::time::Instant;

fn main() {
    let start = Instant::now();
    let p1 = aoc16_day13::part1();
    println!("Part 1: {p1} [{:?}]", start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day13::part2();
    println!("Part 2: {p2} [{:?}]", start.elapsed());

    assert_eq!(p1, 96);
    assert_eq!(p2, 141);
}
