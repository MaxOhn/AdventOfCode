use std::hint::unreachable_unchecked;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut numbers = Vec::with_capacity(200);

    for line in input.lines() {
        let n = line.parse().unwrap();
        numbers.push(n);
    }

    numbers.sort_unstable();

    let p1 = part1(&numbers);
    let p2 = part2(&numbers);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(numbers: &[u32]) -> u32 {
    for i in 0..numbers.len() {
        let res = unsafe {
            numbers
                .get_unchecked(i + 1..)
                .binary_search(&(2020 - numbers.get_unchecked(i)))
        };
        if let Ok(j) = res {
            return unsafe { numbers.get_unchecked(i) * numbers.get_unchecked(j + i + 1) };
        }
    }
    unsafe { unreachable_unchecked() }
}

fn part2(numbers: &[u32]) -> u32 {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if unsafe { numbers.get_unchecked(i) + numbers.get_unchecked(j) } > 2020 {
                break;
            }
            let res = unsafe {
                numbers
                    .get_unchecked(j + 1..)
                    .binary_search(&(2020 - numbers.get_unchecked(i) - numbers.get_unchecked(j)))
            };
            if let Ok(k) = res {
                return unsafe {
                    numbers.get_unchecked(i)
                        * numbers.get_unchecked(j)
                        * numbers.get_unchecked(k + j + 1)
                };
            }
        }
    }
    unsafe { unreachable_unchecked() }
}
