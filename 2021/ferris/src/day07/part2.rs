#![feature(core_intrinsics)]

use std::intrinsics::unlikely;

pub fn run(input: &[u8]) -> i64 {
    average(input)
}

pub fn average(input: &[u8]) -> i64 {
    let len = input.len();
    let input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut i = 0;
    let mut sum = 0;

    let numbers: Vec<_> = (0..1000)
        .map(|_| {
            let n = parse_number(input, &mut i);
            sum += n;

            n
        })
        .collect();

    let avg_min = (sum as f32 / 1000.0).floor() as i64;
    let avg_max = (sum as f32 / 1000.0).ceil() as i64;

    let val_min = numbers
        .iter()
        .map(|&n| (n - avg_min).abs())
        .fold(0, |fuel, diff| fuel + (diff * (diff + 1)) / 2);

    let avg_max = numbers
        .iter()
        .map(|&n| (n - avg_max).abs())
        .fold(0, |fuel, diff| fuel + (diff * (diff + 1)) / 2);

    val_min.min(avg_max)
}

pub fn regular(input: &[u8]) -> i64 {
    let len = input.len();
    let input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut i = 0;
    let mut min = i64::MAX;
    let mut max = 0;

    let numbers: Vec<_> = (0..1000)
        .map(|_| {
            let n = parse_number(input, &mut i);

            if n < min {
                min = n;

                if unlikely(n > max) {
                    max = n;
                }
            } else if n > max {
                max = n;
            }

            n
        })
        .collect();

    (min..max)
        .map(|pos| {
            numbers
                .iter()
                .map(|&n| (n - pos).abs())
                .fold(0, |fuel, diff| fuel + (diff * (diff + 1)) / 2)
        })
        .min()
        .unwrap_or(0)
}

#[inline(always)]
fn parse_number(bytes: &[u8], start: &mut usize) -> i64 {
    bytes[*start..]
        .iter()
        .copied()
        .inspect(|_| *start += 1)
        .take_while(|&byte| byte != b',')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i64)
}
