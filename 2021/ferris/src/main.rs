use aoc_ferris::*;

fn main() {
    let bytes = include_str!("../../day14/input").as_bytes();

    let p1 = day14::part1::run(bytes);
    let p2 = day14::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 2549);
    assert_eq!(p2, 2516901104210);
}
