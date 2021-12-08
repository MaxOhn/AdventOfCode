#![feature(core_intrinsics)]

use std::intrinsics::unlikely;

use memchr::{memchr, memrchr};

pub fn run(input: &[u8]) -> i64 {
    let mut i = 0;
    let mut sum = 0;

    let last = memrchr(b'|', input).unwrap() - 55;

    loop {
        let line = memchr(b'|', &input[i..]).unwrap();
        i += line + 2;

        for _ in 0..3 {
            let len = memchr(b' ', &input[i..]).unwrap();
            sum += (len <= 4 || len == 7) as i64;
            i += len + 1;
        }

        if unlikely(i > last) {
            let len = input.len() - i;

            return sum + (len <= 4 || len == 7) as i64;
        } else {
            let white = memchr(b'\n', &input[i..]).unwrap();
            sum += (white <= 4 || white == 7) as i64;
            i += white + 2;
        }
    }
}
