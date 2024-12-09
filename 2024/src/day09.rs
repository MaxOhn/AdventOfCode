use std::{iter, num::NonZeroU16};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut memory = Vec::new();

    for (chunk, i) in input.as_bytes().chunks(2).zip(1..) {
        let id = NonZeroU16::new(i);
        let blocks = chunk[0] & 0xF;
        let free = chunk.get(1).map_or(0, |&byte| byte & 0xF);

        memory.extend(iter::repeat(id).take(blocks as usize));
        memory.extend(iter::repeat(None).take(free as usize));
    }

    let mut slice = memory.as_slice();
    let mut checksum = 0;
    let mut idx = 0;

    loop {
        let from = slice
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, opt)| opt.map(|val| (i, val)))
            .next_back();

        let to = slice
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| match opt {
                Some(id) => {
                    checksum += (idx + i as u64) * (id.get() - 1) as u64;

                    None
                }
                None => Some(i),
            })
            .next();

        let Some(((from, from_val), to)) = from.zip(to) else {
            break;
        };

        if to > from {
            break;
        }

        idx += to as u64;
        checksum += idx * (from_val.get() - 1) as u64;
        idx += 1;

        slice = &slice[to + 1..from];
    }

    checksum
}

#[derive(Copy, Clone)]
struct FileMeta {
    idx: usize,
    len: usize,
}

fn part2(input: &str) -> u64 {
    let mut memory = Vec::new();
    let mut metas = Vec::new();

    for (chunk, i) in input.as_bytes().chunks(2).zip(1..) {
        let idx = NonZeroU16::new(i);
        let blocks = chunk[0] & 0xF;
        let free = chunk.get(1).map_or(0, |&byte| byte & 0xF);

        let meta = FileMeta {
            idx: memory.len(),
            len: blocks as usize,
        };

        metas.push(meta);
        memory.extend(iter::repeat(idx).take(blocks as usize));
        memory.extend(iter::repeat(None).take(free as usize));
    }

    while let Some(meta) = metas.pop() {
        let FileMeta {
            idx: from,
            len: from_len,
        } = meta;

        let (mut front, back) = memory.split_at_mut(from);

        loop {
            let Some(to) = front.iter().position(Option::is_none) else {
                break;
            };

            front = &mut front[to..];

            let to_len = front.iter().copied().take_while(Option::is_none).count();

            if to_len >= from_len {
                back[..from_len].swap_with_slice(&mut front[..from_len]);

                break;
            }

            front = &mut front[to_len..];
        }
    }

    memory
        .into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|n| (i, n)))
        .map(|(i, id)| i as u64 * (id.get() - 1) as u64)
        .sum()
}
