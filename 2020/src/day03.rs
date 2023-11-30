use aoc_rust::Solution;
use eyre::Result;

#[rustfmt::skip]
static mut COUNTERS: [Counter; 4] = [
    Counter { step: 1, x: 0, count: 0 },
    Counter { step: 3, x: 0, count: 0 },
    Counter { step: 5, x: 0, count: 0 },
    Counter { step: 7, x: 0, count: 0 },
];

pub fn run(input: &str) -> Result<Solution> {
    let mut skipper = 0;
    let mut skipper_x = 0;
    let mut y = 0;

    for line in input.lines() {
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
    }

    let p1 = unsafe { COUNTERS.get_unchecked(1).count };
    let p2 = unsafe { COUNTERS.iter() }
        .map(|counter| counter.count)
        .fold(skipper, |product, count| product * count);

    Ok(Solution::new().part1(p1).part2(p2))
}

struct Counter {
    step: usize,
    x: usize,
    count: u32,
}
