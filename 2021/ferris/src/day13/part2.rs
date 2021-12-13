use memchr::{memchr, memchr_iter};
use rustc_hash::FxHashSet;

const FOLD_COUNT: usize = 12;

pub fn run(input: &[u8]) -> i64 {
    without_branches_abs(input)
}

pub fn with_branches(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let fold_fns = [fold_x, fold_y];
    let mut folds = [(fold_x as fn(&mut u16, &mut u16, u16), 0); FOLD_COUNT];

    let axis = input[f + 11] - b'x';

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

    folds[0] = (fold_fns[axis as usize], n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12] - b'x';

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

        folds[i] = (fold_fns[axis as usize], n);
    }

    let mut set = FxHashSet::default();
    let mut bytes = input[..f - 2].iter().copied();

    while bytes.len() > 0 {
        let mut x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as u16);

        let mut y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as u16);

        for (fold_fn, n) in folds {
            fold_fn(&mut x, &mut y, n)
        }

        set.insert((x, y));
    }

    set.len() as i64
}

#[inline(always)]
fn fold_x(x: &mut u16, _y: &mut u16, n: u16) {
    if *x > n {
        *x = (n << 1) - *x;
    }
}

#[inline(always)]
fn fold_y(_x: &mut u16, y: &mut u16, n: u16) {
    if *y > n {
        *y = (n << 1) - *y;
    }
}

// --- branchless ---

pub fn without_branches(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let fold_fns = [fold_x_branchless, fold_y_branchless];
    let mut folds = [(fold_x_branchless as fn(&mut u16, &mut u16, u16), 0); FOLD_COUNT];

    let axis = input[f + 11] - b'x';

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

    folds[0] = (fold_fns[axis as usize], n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12] - b'x';

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as u16);

        folds[i] = (fold_fns[axis as usize], n);
    }

    let mut set = FxHashSet::default();
    let mut bytes = input[..f - 2].iter().copied();

    while bytes.len() > 0 {
        let mut x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as u16);

        let mut y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as u16);

        for (fold_fn, n) in folds {
            fold_fn(&mut x, &mut y, n)
        }

        set.insert((x, y));
    }

    set.len() as i64
}

#[inline(always)]
fn fold_x_branchless(x: &mut u16, _y: &mut u16, n: u16) {
    *x = (*x > n) as u16 * ((n << 1) - *x) + (*x <= n) as u16 * *x;
}

#[inline(always)]
fn fold_y_branchless(_x: &mut u16, y: &mut u16, n: u16) {
    *y = (*y > n) as u16 * ((n << 1) - *y) + (*y <= n) as u16 * *y;
}

// --- branchless abs ---

pub fn without_branches_abs(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let fold_fns = [fold_x_branchless_abs, fold_y_branchless_abs];
    let mut folds = [(fold_x_branchless_abs as fn(&mut i16, &mut i16, i16), 0); FOLD_COUNT];

    let axis = input[f + 11] - b'x';

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

    folds[0] = (fold_fns[axis as usize], n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12] - b'x';

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

        folds[i] = (fold_fns[axis as usize], n);
    }

    let mut set = FxHashSet::default();
    let mut bytes = input[..f - 2].iter().copied();

    while bytes.len() > 0 {
        let mut x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as i16);

        let mut y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as i16);

        for (fold_fn, n) in folds {
            fold_fn(&mut x, &mut y, n)
        }

        set.insert((x, y));
    }

    set.len() as i64
}

#[inline(always)]
fn fold_x_branchless_abs(x: &mut i16, _y: &mut i16, n: i16) {
    *x = n - (*x - n).abs()
}

#[inline(always)]
fn fold_y_branchless_abs(_x: &mut i16, y: &mut i16, n: i16) {
    *y = n - (*y - n).abs()
}
