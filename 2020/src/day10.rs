use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut adapters: Vec<u8> = Vec::with_capacity(128);

    for line in input.lines() {
        adapters.push(line.parse().unwrap());
    }

    adapters.sort_unstable_by_key(|k| std::cmp::Reverse(*k));
    adapters.insert(0, unsafe { adapters.get_unchecked(0) } + 3);
    adapters.push(0);

    Ok(Solution::new()
        .part1(part1(&adapters))
        .part2(part2(&adapters)))
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
