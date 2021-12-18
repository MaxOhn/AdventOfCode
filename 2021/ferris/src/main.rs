use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day18/input").as_bytes();

    let p1 = day18::part1::run(bytes);
    let p2 = day18::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 4480);
    assert_eq!(p2, 4676);
}
