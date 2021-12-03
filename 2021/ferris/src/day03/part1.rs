#![feature(portable_simd)]

use core_simd::{LaneCount, Simd, SupportedLaneCount};

pub fn run(input: &[u8]) -> i64 {
    let mut first_eight: MySimd<8> = MySimd::new();
    let mut last_four: MySimd<4> = MySimd::new();

    for chunk in ByteChunks::new(input) {
        first_eight.process(&chunk[..8]);
        last_four.process(&chunk[8..]);
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    let (count_a, wraps_a) = first_eight.into_array();
    let (count_b, wraps_b) = last_four.into_array();

    let iter = count_a
        .into_iter()
        .chain(count_b)
        .zip(wraps_a.into_iter().chain(wraps_b));

    for (count, wrap) in iter {
        let amount = (wrap as u32 * 256) + count as u32;
        gamma = gamma * 2 + (amount > 500) as i64;
        epsilon = epsilon * 2 + (amount < 500) as i64;
    }

    gamma * epsilon
}

struct MySimd<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Amount of '1's at an index
    count: Simd<u8, LANES>,
    /// Amount of times the u8 of `count` at the index wrapped around
    wraps: Simd<u8, LANES>,
}

impl<const LANES: usize> MySimd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn new() -> Self {
        Self {
            count: Simd::splat(0),
            wraps: Simd::splat(0),
        }
    }

    fn into_array(self) -> ([u8; LANES], [u8; LANES]) {
        (self.count.to_array(), self.wraps.to_array())
    }

    fn process(&mut self, slice: &[u8]) {
        let line = Simd::from_slice(slice) - Simd::splat(b'0');
        self.wraps = (line.lanes_ne(Simd::splat(0)) & self.count.lanes_eq(Simd::splat(u8::MAX)))
            .select(self.wraps + Simd::splat(1), self.wraps);
        self.count += line;
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
        let size = self.v.len() / 13;

        (size, Some(size))
    }
}
