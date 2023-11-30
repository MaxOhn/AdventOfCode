use std::hint::unreachable_unchecked;

use aoc_rust::Solution;
use eyre::Result;

const NORTH: i16 = 0;
const EAST: i16 = 1;
const SOUTH: i16 = 2;
const WEST: i16 = 3;

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> i16 {
    let mut dir = EAST;
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let n: i16 = line[1..].parse().unwrap();

        match unsafe { *bytes.get_unchecked(0) } {
            b'N' => y -= n,
            b'W' => x -= n,
            b'S' => y += n,
            b'E' => x += n,
            b'L' => dir = (dir + n / 30) % 4,
            b'R' => dir = (dir + n / 90) % 4,
            b'F' => match dir {
                NORTH => y -= n,
                EAST => x += n,
                SOUTH => y += n,
                WEST => x -= n,
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }

    x.abs() + y.abs()
}

fn part2(input: &str) -> i32 {
    let mut wx = 10;
    let mut wy = -1;
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let n: i32 = line[1..].parse().unwrap();

        match unsafe { *bytes.get_unchecked(0) } {
            b'N' => wy -= n,
            b'W' => wx -= n,
            b'S' => wy += n,
            b'E' => wx += n,
            b'L' => match n {
                90 => swap(&mut wy, &mut wx),
                180 => {
                    wx *= -1;
                    wy *= -1;
                }
                270 => swap(&mut wx, &mut wy),
                _ => unsafe { unreachable_unchecked() },
            },
            b'R' => match n {
                90 => swap(&mut wx, &mut wy),
                180 => {
                    wx *= -1;
                    wy *= -1;
                }
                270 => swap(&mut wy, &mut wx),
                _ => unsafe { unreachable_unchecked() },
            },
            b'F' => {
                x += n * wx;
                y += n * wy;
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    x.abs() + y.abs()
}

fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
    *a *= -1;
}
