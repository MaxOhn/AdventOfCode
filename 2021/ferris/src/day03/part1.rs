#![feature(portable_simd)]

use std::simd::{u8x16, Simd, SimdPartialOrd};

pub fn run(input: &[u8]) -> i64 {
    let mut simd = MySimd::new();
    let mut count = 0;

    for chunk in ByteChunks::new(input) {
        simd.process(chunk);
        count += 1;
    }

    let mut gamma = 0;
    let (count, wraps) = simd.finish(count);

    for (count, wrap) in count.into_iter().zip(wraps).take(12) {
        let amount = (wrap as u32 * 256) + count as u32;
        gamma = gamma * 2 + (amount > 500) as i64;
    }

    gamma * (!gamma & 0b1111_1111_1111)
}

struct MySimd {
    /// Amount of '1's at an index
    count: u8x16,
    /// Amount of times the u8 of `count` at the index wrapped around
    wraps: u8x16,
    ones: u8x16,
}

impl MySimd {
    fn new() -> Self {
        Self {
            count: Simd::splat(0),
            wraps: Simd::splat(0),
            ones: Simd::splat(1),
        }
    }

    fn process(&mut self, slice: &[u8]) {
        let line = unsafe {
            u8x16::from_array([
                *slice.get_unchecked(0),
                *slice.get_unchecked(1),
                *slice.get_unchecked(2),
                *slice.get_unchecked(3),
                *slice.get_unchecked(4),
                *slice.get_unchecked(5),
                *slice.get_unchecked(6),
                *slice.get_unchecked(7),
                *slice.get_unchecked(8),
                *slice.get_unchecked(9),
                *slice.get_unchecked(10),
                *slice.get_unchecked(11),
                0,
                0,
                0,
                0,
            ])
        };

        self.count += line;
        let mask = self.count.simd_lt(line);
        self.wraps = mask.select(self.wraps + self.ones, self.wraps);
    }

    fn finish(&mut self, count: usize) -> ([u8; 16], [u8; 16]) {
        let factor = count * b'0' as usize;
        let wraps = Simd::splat((factor / 256) as u8);
        let remaining = Simd::splat((factor % 256) as u8);

        let mask = self.count.simd_lt(remaining);
        self.wraps -= mask.select(wraps + self.ones, wraps);
        self.count -= remaining;

        (self.count.to_array(), self.wraps.to_array())
    }
}

struct ByteChunks<'a> {
    v: &'a [u8],
}

impl<'a> ByteChunks<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { v: slice }
    }
}

impl<'a> Iterator for ByteChunks<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.v.is_empty() {
            None
        } else {
            let ret = Some(&self.v[..12]);
            self.v = self.v.get(13..).unwrap_or_default();

            ret
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.v.len() + 1) / 13;

        (size, Some(size))
    }
}
