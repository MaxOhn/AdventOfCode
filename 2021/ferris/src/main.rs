use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day11/input").as_bytes();

    let p1 = day11::part1::run(bytes);
    let p2 = day11::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 1665);
    assert_eq!(p2, 235);
}
