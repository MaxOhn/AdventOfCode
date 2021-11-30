use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[rustfmt::skip]
static mut COUNTERS: [Counter; 4] = [
    Counter { step: 1, x: 0, count: 0 },
    Counter { step: 3, x: 0, count: 0 },
    Counter { step: 5, x: 0, count: 0 },
    Counter { step: 7, x: 0, count: 0 },
];

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut skipper = 0;
    let mut skipper_x = 0;
    let mut y = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let trimmed = line.trim_end().as_bytes();

        for counter in unsafe { COUNTERS.iter_mut() } {
            counter.count += (unsafe { *trimmed.get_unchecked(counter.x) } == b'#') as u32;
            counter.x = (counter.x + counter.step) % trimmed.len();
        }

        if y % 2 == 0 {
            skipper += (unsafe { *trimmed.get_unchecked(skipper_x) } == b'#') as u32;
            skipper_x = (skipper_x + 1) % trimmed.len();
        }
        y += 1;

        line.clear();
    }

    let p1 = unsafe { COUNTERS.get_unchecked(1).count };
    let p2 = unsafe { COUNTERS.iter() }
        .map(|counter| counter.count)
        .fold(skipper, |product, count| product * count);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 650µs

    assert_eq!(p1, 156);
    assert_eq!(p2, 3_521_829_480);
}

struct Counter {
    step: usize,
    x: usize,
    count: u32,
}
