use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::with_capacity(5);
    let mut numbers = Vec::with_capacity(200);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let n = util::Parse::parse(line.as_bytes());
        numbers.push(n);
        line.clear();
    }

    numbers.sort_unstable();

    println!("Setup: {:?}", start.elapsed()); // 103µs

    let start = Instant::now();
    let p1 = part1(&numbers);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 600ns

    let start = Instant::now();
    let p2 = part2(&numbers);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 2.5µs

    assert_eq!(p1, 326_211);
    assert_eq!(p2, 131_347_190);
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
