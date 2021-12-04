#![allow(dead_code, unused_attributes)]
#![feature(core_intrinsics, portable_simd)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let bytes = include_str!("../../day04/input").as_bytes();

    let p1 = day04::part1::run(bytes);
    let p2 = day04::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 39_902);
    assert_eq!(p2, 26_936);
}
