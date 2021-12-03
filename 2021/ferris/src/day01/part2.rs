use std::mem::replace;

pub fn run(input: &[u8]) -> i64 {
    let mut bytes = input.iter();
    let mut prev = [0, 0, 0];
    let mut i = 0;
    let mut n = 0;

    for &byte in &mut bytes {
        if byte == b'\n' {
            *unsafe { prev.get_unchecked_mut(0) } = replace(&mut n, 0);
            break;
        }

        n = n * 10 + (byte & 0x0F) as u64;
    }

    for &byte in &mut bytes {
        if byte == b'\n' {
            *unsafe { prev.get_unchecked_mut(1) } = replace(&mut n, 0);
            break;
        }

        n = n * 10 + (byte & 0x0F) as u64;
    }

    for &byte in &mut bytes {
        if byte == b'\n' {
            *unsafe { prev.get_unchecked_mut(2) } = replace(&mut n, 0);
            break;
        }

        n = n * 10 + (byte & 0x0F) as u64;
    }

    let mut p2 = 0;

    for &byte in bytes {
        if byte == b'\n' {
            p2 += (replace(unsafe { prev.get_unchecked_mut(i) }, n) < replace(&mut n, 0)) as i64;
            i = (i + 1) % 3;
        } else {
            n = n * 10 + (byte & 0x0F) as u64;
        }
    }

    p2 + (prev[i] < n) as i64
}
