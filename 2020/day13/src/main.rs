use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 138);
    assert_eq!(p2, 226_845_233_210_288);
}

fn part1() -> u32 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let _ = input.read_line(&mut line);

    let earliest: u32 = util::Parse::parse(line.as_bytes());
    line.clear();

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut min = u32::MAX;
    let mut p1 = 0;
    let mut i = 0;
    let mut n = 0;

    while i < bytes.len() {
        let byte = unsafe { *bytes.get_unchecked(i) };
        if byte == b',' {
            let candidate = n - earliest % n;

            if candidate < min {
                min = candidate;
                p1 = candidate * n;
            }

            n = 0;
        } else if byte == b'x' {
            i += 2;
            continue;
        } else {
            n = n * 10 + (byte & 0x0F) as u32;
        }

        i += 1;
    }

    if n > 0 {
        let candidate = n - earliest % n;

        if candidate < min {
            p1 = candidate * n;
        }
    }

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 82µs

    p1
}

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let _ = input.read_line(&mut line);
    line.clear();

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut ids = Vec::with_capacity(10);
    let mut idx = 0;
    let mut i = 0;
    let mut n = 0;

    while i < bytes.len() {
        let byte = unsafe { *bytes.get_unchecked(i) };
        if byte == b',' {
            ids.push((n, idx));
            idx += 1;
            n = 0;
        } else if byte == b'x' {
            idx += 1;
            i += 2;
            continue;
        } else {
            n = n * 10 + (byte & 0x0F) as u64;
        }

        i += 1;
    }

    if n > 0 {
        ids.push((n, idx));
    }

    let (first, _) = ids.swap_remove(0);
    ids.sort_by_key(|(a, _)| std::cmp::Reverse(*a));

    let mut i = 0;
    let mut t = 0;
    let mut step = first;

    'outer: while i < ids.len() {
        t += step;
        let mut j = 0;
        let mut next_step = first;

        while j <= i {
            let (n, offset) = unsafe { *ids.get_unchecked(j) };

            if (t + offset) % n != 0 {
                continue 'outer;
            }

            j += 1;
            next_step *= n;
        }

        i += 1;
        step = next_step;
    }

    println!("Part 2: {} [{:?}]", t, start.elapsed()); // 56µs

    t
}
