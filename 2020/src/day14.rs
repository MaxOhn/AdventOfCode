use std::collections::HashMap;
use std::hint::unreachable_unchecked;

use aoc_rust::util::int_hasher::IntHasher;
use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u64 {
    let mut mask: [u8; 36] = [b'X'; 36];
    let mut mem = HashMap::with_capacity_and_hasher(512, IntHasher);

    for line in input.lines() {
        let bytes = line.as_bytes();

        match unsafe { *bytes.get_unchecked(1) } {
            b'e' => {
                let mut i = 4;
                let mut adr = 0;

                loop {
                    let byte = unsafe { *bytes.get_unchecked(i) };

                    if byte == b']' {
                        break;
                    }

                    adr = adr * 10 + (byte & 0x0F) as usize;
                    i += 1;
                }

                i += 4;
                let mut val: u64 =
                    unsafe { std::str::from_utf8_unchecked(bytes.get_unchecked(i..)) }
                        .parse()
                        .unwrap();
                let mut j = 0;

                while j < 36 {
                    match unsafe { *mask.get_unchecked(j) } {
                        b'X' => {}
                        b'0' => val &= !(1 << (35 - j)),
                        b'1' => val |= 1 << (35 - j),
                        _ => unsafe { unreachable_unchecked() },
                    }

                    j += 1;
                }

                mem.insert(adr, val);
            }
            b'a' => mask.copy_from_slice(unsafe { bytes.get_unchecked(7..43) }),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    mem.values().copied().sum()
}

fn part2(input: &str) -> u64 {
    let mut mask_zeroed = 0;
    let mut mask_ones = 0;
    let mut xs = Vec::with_capacity(8);
    let mut mem = HashMap::with_capacity_and_hasher(100_000, IntHasher);

    for line in input.lines() {
        let bytes = line.as_bytes();

        match unsafe { *bytes.get_unchecked(1) } {
            b'e' => {
                let mut i = 4;
                let mut adr = 0;

                loop {
                    let byte = unsafe { *bytes.get_unchecked(i) };

                    if byte == b']' {
                        break;
                    }

                    adr = adr * 10 + (byte & 0x0F) as usize;
                    i += 1;
                }

                let val: u64 = std::str::from_utf8(unsafe { bytes.get_unchecked(i + 4..) })
                    .unwrap()
                    .parse()
                    .unwrap();

                adr = (adr | mask_zeroed) & mask_ones;

                for i in 0..(1 << xs.len()) {
                    let mut a = adr;
                    let mut j = 0;

                    while j < xs.len() {
                        a |= ((i >> j) & 1 as usize) << unsafe { *xs.get_unchecked(j) };
                        j += 1;
                    }

                    mem.insert(a, val);
                }
            }
            b'a' => {
                xs.clear();
                mask_zeroed = 0;
                mask_ones = 0;
                let mut i = 7;

                while i < 43 {
                    match unsafe { *bytes.get_unchecked(i) } {
                        b'0' => {
                            mask_zeroed <<= 1;
                            mask_ones = (mask_ones << 1) + 1;
                        }
                        b'1' => {
                            mask_zeroed = (mask_zeroed << 1) + 1;
                            mask_ones = (mask_ones << 1) + 1;
                        }
                        b'X' => {
                            mask_zeroed <<= 1;
                            mask_ones <<= 1;
                            xs.push(42 - i);
                        }
                        _ => unsafe { unreachable_unchecked() },
                    }

                    i += 1;
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    mem.values().copied().sum()
}

#[allow(dead_code)]
fn part2_old(input: &str) -> u64 {
    let mut mask: [u8; 36] = [b'X'; 36];
    let mut xs = Vec::with_capacity(8);
    let mut mem = HashMap::with_capacity_and_hasher(512, IntHasher);

    for line in input.lines() {
        let bytes = line.as_bytes();

        match unsafe { *bytes.get_unchecked(1) } {
            b'e' => {
                let mut i = 4;
                let mut adr = 0;

                loop {
                    let byte = unsafe { *bytes.get_unchecked(i) };

                    if byte == b']' {
                        break;
                    }

                    adr = adr * 10 + (byte & 0x0F) as usize;
                    i += 1;
                }

                i += 4;
                let val: u64 = std::str::from_utf8(unsafe { bytes.get_unchecked(i..) })
                    .unwrap()
                    .parse()
                    .unwrap();

                for a in nums(adr, &mask, &xs) {
                    mem.insert(a, val);
                }
            }
            b'a' => {
                xs.clear();
                mask.copy_from_slice(unsafe { bytes.get_unchecked(7..43) });

                let mut i = 7;

                while i < 43 {
                    if unsafe { *bytes.get_unchecked(i) == b'X' } {
                        xs.push(i - 7);
                    }

                    i += 1;
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    mem.values().copied().sum()
}

fn nums<'a>(mut num: usize, mask: &'a [u8], xs: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
    let mut mask_num = 0;
    let mut i = 0;

    while i < 36 {
        match unsafe { *mask.get_unchecked(i) } {
            b'X' => {
                num &= usize::MAX - (1 << (35 - i));
                mask_num = (mask_num << 1) + 1;
            }
            b'0' => {}
            b'1' => num |= 1 << (35 - i),
            _ => unsafe { unreachable_unchecked() },
        }

        i += 1;
    }

    (0..)
        .scan(mask_num, move |mask_num, _| {
            if *mask_num == 0 {
                return None;
            }

            let mut n = num;
            let mut i = 0;

            while i < xs.len() {
                let bit = (*mask_num >> (xs.len() - i - 1)) & 1;
                n |= bit << unsafe { 35 - *xs.get_unchecked(i) };
                i += 1;
            }

            *mask_num -= 1;

            Some(n)
        })
        .chain(std::iter::once(num))
}
