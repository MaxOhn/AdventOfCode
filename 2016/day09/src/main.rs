#[macro_use]
extern crate util;

use std::time::Instant;

fn main() {
    let bytes = std::fs::read("./input").unwrap();

    let start = Instant::now();
    let p1 = part1(&bytes);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&bytes);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 120_765);
    assert_eq!(p2, 11_658_395_076);
}

fn part1(bytes: &[u8]) -> usize {
    let mut i = 0;
    let mut len = 0;

    while i < bytes.len() {
        match get!(bytes, i) {
            b' ' => i += 1,
            b'(' => match parse_marker(get_ref!(bytes, i + 1..)) {
                Some((size, reps, j)) => {
                    i += j + 2;
                    let spaces = bytecount::count(get_ref!(bytes, i..i + size), b' ');
                    len += reps * (size - spaces);
                    i += size;
                }
                None => {
                    len += 1;
                    i += 1
                }
            },
            _ => {
                len += 1;
                i += 1;
            }
        }
    }

    len
}

fn part2(bytes: &[u8]) -> usize {
    let mut i = 0;
    let mut len = 0;

    while i < bytes.len() {
        match get!(bytes, i) {
            b' ' => i += 1,
            b'(' => match parse_marker(get_ref!(bytes, i + 1..)) {
                Some((size, reps, j)) => {
                    i += j + 2;
                    let decompressed = part2(get_ref!(bytes, i..i + size));
                    len += reps * decompressed;
                    i += size;
                }
                None => {
                    len += 1;
                    i += 1
                }
            },
            _ => {
                len += 1;
                i += 1;
            }
        }
    }

    len
}

fn parse_marker(bytes: &[u8]) -> Option<(usize, usize, usize)> {
    let mut i = 0;
    let mut a = None;
    let mut b = None;

    loop {
        match get!(bytes, i) {
            n if n.is_ascii_digit() => {
                let val = a.get_or_insert(0);
                *val = *val * 10 + (n & 0xF) as usize;
            }
            b'x' => swap!(&mut a, &mut b),
            b')' => return Some((b?, a?, i)),
            _ => return None,
        }

        i += 1;
    }
}
