use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use util::{NumHasherBuilder, Parse};

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 7_440_382_076_205);
    assert_eq!(p2, 4_200_656_704_538);
}

fn part1() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut mask: [u8; 36] = [b'X'; 36];
    let mut mem = HashMap::with_capacity_and_hasher(512, NumHasherBuilder);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
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
                let mut val: u64 = Parse::parse(unsafe { bytes.get_unchecked(i..) });
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

        line.clear();
    }

    let p1: u64 = mem.values().copied().sum();

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 227µs

    p1
}

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut mask_zeroed = 0;
    let mut mask_ones = 0;
    let mut xs = Vec::with_capacity(8);
    let mut mem = HashMap::with_capacity_and_hasher(100_000, NumHasherBuilder);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
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

                let val: u64 = Parse::parse(unsafe { bytes.get_unchecked(i + 4..) });

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

        line.clear();
    }

    let p2: u64 = mem.values().copied().sum();

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 12ms

    p2
}

#[allow(dead_code)]
fn part2_old() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut mask: [u8; 36] = [b'X'; 36];
    let mut xs = Vec::with_capacity(8);
    let mut mem = HashMap::with_capacity_and_hasher(512, NumHasherBuilder);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
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
                let val: u64 = util::Parse::parse(unsafe { bytes.get_unchecked(i..) });

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

        line.clear();
    }

    let p2: u64 = mem.values().copied().sum();

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 66ms

    p2
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
