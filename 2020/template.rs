use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        line.clear();
    }

    println!("Setup: {:?}", start.elapsed()); //

    let start = Instant::now();
    let p1 = part1();
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); //

    let start = Instant::now();
    let p2 = part2();
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); //

    assert_eq!(p1, 0);
    assert_eq!(p2, 0);
}

fn part1() -> u32 {
    todo!()
}

fn part2() -> u32 {
    todo!()
}
