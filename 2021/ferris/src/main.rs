use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day12/input").as_bytes();

    let p1 = day12::part1::run(bytes);
    let p2 = day12::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 5756);
    assert_eq!(p2, 144_603);
}
