use std::hint::unreachable_unchecked;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut split = line.split('-');
        let min = split.next().unwrap().parse().unwrap();
        let mut split = split
            .next()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
            .split(' ');
        let max = split.next().unwrap().parse().unwrap();
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
    }

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    let count = bytecount::count(password, letter);

    count >= min && count <= max
}

fn part2(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    unsafe { (*password.get_unchecked(min) == letter) ^ (*password.get_unchecked(max) == letter) }
}
