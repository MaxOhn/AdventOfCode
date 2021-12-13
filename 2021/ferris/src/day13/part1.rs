use std::collections::HashSet;

use memchr::memchr;
use rustc_hash::FxHashSet;

pub fn run(input: &[u8]) -> i64 {
    fxhash(input)
}

pub fn stdhash(input: &[u8]) -> i64 {
    let f = memchr(b'f', input).unwrap();
    let axis = input[f + 11];
    let fold = if axis == b'x' { fold_x } else { fold_y };

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

    let mut set = HashSet::new();
    let mut bytes = input[..f - 2].iter().copied();
    let m = n + n;

    while bytes.len() > 0 {
        let x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as u16);

        let y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as u16);

        set.insert(fold(x, y, n, m));
    }

    set.len() as i64
}

pub fn fxhash(input: &[u8]) -> i64 {
    let f = memchr(b'f', input).unwrap();
    let axis = input[f + 11];
    let fold = if axis == b'x' { fold_x } else { fold_y };

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

    let mut set = FxHashSet::default();
    let mut bytes = input[..f - 2].iter().copied();
    let m = n + n;

    while bytes.len() > 0 {
        let x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as u16);

        let y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as u16);

        set.insert(fold(x, y, n, m));
    }

    set.len() as i64
}

#[inline(always)]
fn fold_x(x: u16, y: u16, n: u16, m: u16) -> (u16, u16) {
    (if x > n { m - x } else { x }, y)
}

#[inline(always)]
fn fold_y(x: u16, y: u16, n: u16, m: u16) -> (u16, u16) {
    (x, if y > n { m - y } else { y })
}
