use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day13/input").as_bytes();

    let p1 = day13::part1::run(bytes);
    let p2 = day13::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 716);
    // assert_eq!(p2, 144_603);
}
