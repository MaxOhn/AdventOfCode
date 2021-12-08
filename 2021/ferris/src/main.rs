use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day08/input").as_bytes();

    let p1 = day08::part1::run(bytes);
    let p2 = day08::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 409);
    assert_eq!(p2, 1_024_649);
}
