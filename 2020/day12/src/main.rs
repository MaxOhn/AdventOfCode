use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const NORTH: i16 = 0;
const EAST: i16 = 1;
const SOUTH: i16 = 2;
const WEST: i16 = 3;

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 590);
    assert_eq!(p2, 42_013);
}

fn part1() -> i16 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut dir = EAST;
    let mut x = 0;
    let mut y = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();
        let n: i16 = util::Parse::parse(unsafe { bytes.get_unchecked(1..) });

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

        line.clear();
    }

    let p1 = x.abs() + y.abs();
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 128µs

    p1
}

fn part2() -> i32 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut wx = 10;
    let mut wy = -1;
    let mut x = 0;
    let mut y = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();
        let n: i32 = util::Parse::parse(unsafe { bytes.get_unchecked(1..) });

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

        line.clear();
    }

    let p2 = x.abs() + y.abs();
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 145µs

    p2
}

fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
    *a *= -1;
}
