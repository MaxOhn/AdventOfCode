use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new()
        .part1(execute(input, 2020))
        .part2(execute(input, 30_000_000)))
}

fn execute(input: &str, size: usize) -> u32 {
    let bytes = input.lines().next().unwrap().as_bytes();

    let mut nums = vec![u32::MAX; size];

    let mut i = 0;
    let mut idx = 0;
    let mut last = 0;
    let end = size as u32 - 1;

    while i < bytes.len() {
        let byte = unsafe { *bytes.get_unchecked(i) };
        if byte == b',' {
            unsafe { *nums.get_unchecked_mut(last as usize) = idx }
            last = 0;
            idx += 1;
        } else if byte == b'\n' {
            break;
        } else {
            last = last * 10 + (byte & 0x0F) as u32;
        }

        i += 1;
    }

    while idx < end {
        let last_idx = unsafe { *nums.get_unchecked(last as usize) };
        unsafe { *nums.get_unchecked_mut(last as usize) = idx }
        last = idx.saturating_sub(last_idx);
        idx += 1;
    }

    last
}
