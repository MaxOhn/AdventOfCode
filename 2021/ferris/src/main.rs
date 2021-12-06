#![allow(dead_code, unused_attributes)]
#![feature(core_intrinsics, portable_simd)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let bytes = include_str!("../../day06/input").as_bytes();

    let p1 = day06::part1::run(bytes);
    let p2 = day06::part2::run(bytes);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    assert_eq!(p1, 363_101);
    assert_eq!(p2, 1_644_286_074_024);
}
