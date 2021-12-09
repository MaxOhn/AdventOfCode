use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day09/input").as_bytes();

    let p1 = day09::part1::run(bytes);
    let p2 = day09::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 575);
    assert_eq!(p2, 1_019_700);
}
