#![feature(core_intrinsics)]

use std::intrinsics::unlikely;

pub fn run(input: &[u8]) -> i64 {
    let mut grid = vec![[0_u8; 1000]; 1000];
    let mut i = 0;

    loop {
        let x1 = parse_num(input, &mut i, b',');
        let y1 = parse_num(input, &mut i, b' ');
        i += 3;
        let x2 = parse_num(input, &mut i, b',');
        let y2 = parse_num(input, &mut i, b'\n');

        if x1 == x2 {
            let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };

            for y in range {
                unsafe {
                    *grid.get_unchecked_mut(y).get_unchecked_mut(x1) += 1;
                }
            }
        } else if y1 == y2 {
            let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };

            for x in range {
                unsafe {
                    *grid.get_unchecked_mut(y1).get_unchecked_mut(x) += 1;
                }
            }
        }

        if unlikely(i == input.len()) {
            return grid
                .into_iter()
                .map(|row| row.into_iter())
                .flatten()
                .filter(|&count| count > 1)
                .count() as i64;
        }
    }
}

#[inline(always)]
fn parse_num(bytes: &[u8], i: &mut usize, delim: u8) -> usize {
    unsafe { bytes.get_unchecked(*i..) }
        .iter()
        .copied()
        .inspect(|_| *i += 1)
        .take_while(|&byte| byte != delim)
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as usize)
}
