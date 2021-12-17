use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day17/input").as_bytes();

    let p1 = day17::part1::run(bytes);
    let p2 = day17::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 3916);
    assert_eq!(p2, 2986);
}
