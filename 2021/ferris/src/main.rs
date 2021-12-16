use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day16/input").as_bytes();

    let p1 = day16::part1::run(bytes);
    let p2 = day16::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 886);
    assert_eq!(p2, 184_487_454_837);
}
