use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day10/input").as_bytes();

    let p1 = day10::part1::run(bytes);
    let p2 = day10::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 339_477);
    assert_eq!(p2, 3_049_320_156);
}
