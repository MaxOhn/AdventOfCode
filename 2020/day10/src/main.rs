use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut adapters: Vec<u8> = Vec::with_capacity(128);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        adapters.push(util::Parse::parse(line.as_bytes()));
        line.clear();
    }

    adapters.sort_unstable_by_key(|k| std::cmp::Reverse(*k));
    adapters.insert(0, unsafe { adapters.get_unchecked(0) } + 3);
    adapters.push(0);

    println!("Setup: {:?}", start.elapsed()); // 122µs

    let start = Instant::now();
    let p1 = part1(&adapters);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 800ns

    let start = Instant::now();
    let p2 = part2(&adapters);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 800ns

    assert_eq!(p1, 2470);
    assert_eq!(p2, 1_973_822_685_184);
}

fn part1(adapters: &[u8]) -> u64 {
    static mut DIFFS: [u64; 3] = [0; 3];

    let mut i = adapters.len() - 1;

    while i > 0 {
        unsafe {
            let idx = adapters.get_unchecked(i - 1) - adapters.get_unchecked(i);
            *DIFFS.get_unchecked_mut(idx as usize - 1) += 1
        }

        i -= 1;
    }

    unsafe { DIFFS.get_unchecked(0) * DIFFS.get_unchecked(2) }
}

fn part2(adapters: &[u8]) -> u64 {
    static mut POSSIBS: [u64; 3] = [0; 3];

    unsafe {
        *POSSIBS.get_unchecked_mut(0) = 1;
        *POSSIBS.get_unchecked_mut(1) = 1;
        *POSSIBS.get_unchecked_mut(2) =
            (adapters.get_unchecked(0) - adapters.get_unchecked(2) <= 3) as u64 + 1
    }

    let mut i = 2;

    while i < adapters.len() {
        unsafe {
            let mut possibs = *POSSIBS.get_unchecked((i - 1) % 3);

            if adapters.get_unchecked(i - 2) - adapters.get_unchecked(i) <= 3 {
                possibs += POSSIBS.get_unchecked((i - 2) % 3)
                    + (adapters.get_unchecked(i - 3) - adapters.get_unchecked(i) <= 3) as u64
                        * POSSIBS.get_unchecked(i % 3)
            }

            *POSSIBS.get_unchecked_mut(i % 3) = possibs
        }

        i += 1;
    }

    unsafe { *POSSIBS.get_unchecked((adapters.len() - 1) % 3) }
}
