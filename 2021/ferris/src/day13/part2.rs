use memchr::{memchr, memchr_iter};
use rustc_hash::FxHashSet;

const FOLD_COUNT: usize = 12;

pub fn run(input: &[u8]) -> i64 {
    without_branches_combined_pointers(input)
}

// --- version 1: branches + function pointers ---

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

// --- version 2: branchless + function pointers ---

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

// --- version 3: branchless through abs + function pointers ---

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

// --- version 4: branchless + combined cases ---

pub fn without_branches_combined(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let mut folds = [(0, 0); FOLD_COUNT];

    let axis = input[f + 11];

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

    folds[0] = ((axis - b'x') as i16, n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12];

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

        folds[i] = ((axis - b'x') as i16, n);
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

        for (is_y, n) in folds {
            x = (1 - is_y) * (x > n) as i16 * ((n << 1) - x) + (x <= n) as i16 * x + is_y * x;
            y = is_y * (y > n) as i16 * ((n << 1) - y) + (y <= n) as i16 * y + (1 - is_y) * y;
        }

        set.insert((x, y));
    }

    set.len() as i64
}

// --- version 5: branchless + combined pointers ---

pub fn without_branches_combined_pointers(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let mut folds = [(0, 0); FOLD_COUNT];

    let axis = input[f + 11];

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

    folds[0] = ((axis - b'x') as usize, n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12];

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

        folds[i] = ((axis - b'x') as usize, n);
    }

    let mut set = FxHashSet::default();
    let mut bytes = input[..f - 2].iter().copied();

    while bytes.len() > 0 {
        let x = (&mut bytes)
            .take_while(|&byte| byte != b',')
            .fold(0, |x, byte| x * 10 + (byte & 0x0F) as i16);

        let y = (&mut bytes)
            .take_while(|&byte| byte != b'\n')
            .fold(0, |y, byte| y * 10 + (byte & 0x0F) as i16);

        for (is_y, n) in folds {
            let adr = (1 - is_y) * (&x as *const _ as usize) + is_y * (&y as *const _ as usize);
            let ptr = adr as *mut i16;

            unsafe { *ptr = (*ptr > n) as i16 * ((n << 1) - *ptr) + (*ptr <= n) as i16 * *ptr };
        }

        set.insert((x, y));
    }

    set.len() as i64
}

// --- version 6: branches through references ---

pub fn branched_combined_pointers(mut input: &[u8]) -> i64 {
    let trim = (input[input.len() - 1] == b'\n') as usize;
    input = &input[..input.len() - trim];

    let f = memchr(b'f', input).unwrap();
    let mut folds = [(false, 0); FOLD_COUNT];

    let axis = input[f + 11];

    let n = input[f + 13..]
        .iter()
        .copied()
        .take_while(|&byte| byte != b'\n')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

    folds[0] = (axis == b'x', n);

    for (i, new_line) in (1..).zip(memchr_iter(b'\n', &input[f..])) {
        let axis = input[f + new_line + 12];

        let n = input[f + new_line + 14..]
            .iter()
            .copied()
            .take_while(|&byte| byte != b'\n')
            .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i16);

        folds[i] = (axis == b'x', n);
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

        for (is_x, n) in folds {
            let mut_ref = if is_x { &mut x } else { &mut y };

            *mut_ref = n - (*mut_ref - n).abs();
        }

        set.insert((x, y));
    }

    set.len() as i64
}
