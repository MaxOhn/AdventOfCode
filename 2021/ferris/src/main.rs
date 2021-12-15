use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day15/input").as_bytes();

    let p1 = day15::part1::run(bytes);
    let p2 = day15::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 498);
    assert_eq!(p2, 2901);
}
