use std::cmp::Ordering;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use util::Parse;

static mut PREV: [u64; 25] = [0; 25];

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut nums = Vec::with_capacity(768);
    let mut i = 0;

    while i < 25 {
        let _ = input.read_line(&mut line);
        let n = Parse::parse(line.as_bytes());
        unsafe { *PREV.get_unchecked_mut(i) = n }
        nums.push(n);
        i += 1;
        line.clear();
    }

    i = 0;

    let p1 = loop {
        let _ = input.read_line(&mut line);
        let n = Parse::parse(line.as_bytes());

        if part1(n) {
            unsafe { *PREV.get_unchecked_mut(i) = n }

            break n;
        }

        nums.push(n);
        unsafe { *PREV.get_unchecked_mut(i) = n }
        i = (i + 1) % 25;
        line.clear();
    };

    let p2 = match part2_preempt(&nums, p1) {
        Ok(n) => n,
        Err(i) => {
            line.clear();

            while input
                .read_line(&mut line)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
                != 0
            {
                nums.push(Parse::parse(line.as_bytes()));
                line.clear();
            }

            part2_continue(&nums, p1, i)
        }
    };

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 697µs

    assert_eq!(p1, 552_655_238);
    assert_eq!(p2, 70_672_245);
}

fn part1(target: u64) -> bool {
    let mut i = 0;

    while i < 24 {
        let mut j = i + 1;

        while j < 25 {
            if unsafe { PREV.get_unchecked(i) + PREV.get_unchecked(j) } == target {
                return false;
            }

            j += 1;
        }

        i += 1;
    }

    true
}

fn part2_preempt(nums: &[u64], limit: u64) -> Result<u64, usize> {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while sum < limit {
        sum += unsafe { *nums.get_unchecked(j) };
        j += 1;
    }

    loop {
        match sum.cmp(&limit) {
            Ordering::Less => {
                if j == nums.len() {
                    return Err(i);
                }

                sum += unsafe { *nums.get_unchecked(j) };
                j += 1;
            }
            Ordering::Greater => {
                sum -= unsafe { *nums.get_unchecked(i) };
                i += 1;

                if i == j {
                    sum += unsafe { *nums.get_unchecked(j) };
                    j += 1;
                }
            }
            Ordering::Equal => return Ok(min_max_sum(unsafe { nums.get_unchecked(i..j) })),
        }
    }
}

fn part2_continue(nums: &[u64], limit: u64, mut i: usize) -> u64 {
    let mut j = i;
    let mut sum = 0;

    while sum < limit {
        sum += unsafe { *nums.get_unchecked(j) };
        j += 1;
    }

    loop {
        match sum.cmp(&limit) {
            Ordering::Less => {
                sum += unsafe { *nums.get_unchecked(j) };
                j += 1;
            }
            Ordering::Greater => {
                sum -= unsafe { *nums.get_unchecked(i) };
                i += 1;
            }
            Ordering::Equal => return min_max_sum(unsafe { nums.get_unchecked(i..j) }),
        }
    }
}

fn min_max_sum(slice: &[u64]) -> u64 {
    let mut min = u64::MAX;
    let mut max = 0;
    let mut i = 0;

    while i < slice.len() {
        let n = unsafe { *slice.get_unchecked(i) };

        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }

        i += 1;
    }

    min + max
}
