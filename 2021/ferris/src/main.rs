#![allow(dead_code, unused_attributes)]
#![feature(portable_simd)]

mod day01;
mod day02;
mod day03;

fn main() {
    let bytes = include_str!("../../day03/input").as_bytes();

    let p1 = day03::part1::run(bytes);
    let p2 = day03::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 3_885_894);
    assert_eq!(p2, 4_375_225);
}
