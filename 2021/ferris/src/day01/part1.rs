use std::mem::replace;

pub fn run(input: &[u8]) -> i64 {
    let mut bytes = input.iter();
    let mut prev = 0;

    for &byte in &mut bytes {
        if byte == b'\n' {
            break;
        }

        prev = prev * 10 + (byte & 0x0F) as u64;
    }

    let mut p1 = 0;
    let mut n = 0;

    for &byte in bytes {
        if byte == b'\n' {
            p1 += (replace(&mut prev, n) < replace(&mut n, 0)) as i64;
        } else {
            n = n * 10 + (byte & 0x0F) as u64;
        }
    }

    p1 + (prev < n) as i64
}
