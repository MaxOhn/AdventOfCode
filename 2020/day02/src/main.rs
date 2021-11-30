use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use util::Parse;

#[cfg(not(feature = "reg"))]
fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let mut split = line.split('-');
        let min = Parse::parse(
            split
                .next()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                .as_bytes(),
        );
        let mut split = split
            .next()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
            .split(' ');
        let max = Parse::parse(
            split
                .next()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                .as_bytes(),
        );
        let letter = unsafe {
            *split
                .next()
                .unwrap_or_else(|| unreachable_unchecked())
                .as_bytes()
                .get_unchecked(0)
        };
        let password = split
            .next()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
            .as_bytes();

        p1 += part1(min, max, letter, password) as u16;
        p2 += part2(min - 1, max - 1, letter, password) as u16;

        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 692µs

    assert_eq!(p1, 582);
    assert_eq!(p2, 729);
}

fn part1(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    let count = bytecount::count(password, letter);

    count >= min && count <= max
}

fn part2(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    unsafe { (*password.get_unchecked(min) == letter) ^ (*password.get_unchecked(max) == letter) }
}

#[cfg(feature = "rgx")]
fn main() {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref LINE_MATCHER: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)\n?").unwrap();
    }

    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    while input.read_line(&mut line).unwrap() != 0 {
        let caps = LINE_MATCHER.captures(&line).unwrap();

        let min = caps[1].parse().unwrap();
        let max = caps[2].parse().unwrap();
        let letter = unsafe { *caps[3].as_bytes().get_unchecked(0) };
        let password = caps[4].as_bytes();

        p1 += part1(min, max, letter, password) as u16;
        p2 += part2(min - 1, max - 1, letter, password) as u16;

        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 6ms

    assert_eq!(p1, 582);
    assert_eq!(p2, 729);
}
