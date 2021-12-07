use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day07/input").as_bytes();

    let p1 = day07::part1::run(bytes);
    let p2 = day07::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 336_701);
    assert_eq!(p2, 95_167_302);
}
