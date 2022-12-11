#[cfg(feature = "nightly")]
use core_simd::simd::{u8x4, u8x8, SimdUint};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.as_bytes();

    // let p1 = part1_const_lookup(input)?;
    // let p1 = part1_simd_rayon(input);
    // let p1 = part1_simd(input);
    let p1 = part1_naive(input)?;

    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

const fn gen_lookup_p1() -> [u16; 9] {
    let mut lookup = [0; 9];
    let mut opponent = 0;

    while opponent < 3 {
        let mut mine = 0;

        while mine < 3 {
            lookup[opponent * 3 + mine] = match (mine, opponent) {
                (0, 0) => 3,
                (0, 1) => 0,
                (0, 2) => 6,
                (1, 0) => 6,
                (1, 1) => 3,
                (1, 2) => 0,
                (2, 0) => 0,
                (2, 1) => 6,
                (2, 2) => 3,
                _ => unreachable!(),
            };

            mine += 1;
        }

        opponent += 1;
    }

    lookup
}

const LOOKUP_P1: [u16; 9] = gen_lookup_p1();

#[allow(unused)]
pub fn part1_const_lookup(input: &[u8]) -> u16 {
    input.chunks(4).fold(0, |score, chunk| {
        let opponent = get!(chunk[0]) - b'A';
        let mine = get!(chunk[2]) - b'X';
        let idx = (opponent * 3 + mine) as usize;

        score + get!(LOOKUP_P1[idx]) + mine as u16 + 1
    })
}

#[cfg(feature = "nightly")]
#[allow(unused)]
pub fn part1_simd_rayon(input: &[u8]) -> u16 {
    use std::ops::Add;

    use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

    input
        .par_chunks(32)
        .fold(u16::default, |score, chunk| {
            let mut opponent = u8x8::from_array([
                chunk.first().copied().unwrap_or(b'A'),
                chunk.get(4).copied().unwrap_or(b'A'),
                chunk.get(8).copied().unwrap_or(b'A'),
                chunk.get(12).copied().unwrap_or(b'A'),
                chunk.get(16).copied().unwrap_or(b'A'),
                chunk.get(20).copied().unwrap_or(b'A'),
                chunk.get(24).copied().unwrap_or(b'A'),
                chunk.get(28).copied().unwrap_or(b'A'),
            ]);

            let mut me = u8x8::from_array([
                chunk.get(2).copied().unwrap_or(b'W'),
                chunk.get(6).copied().unwrap_or(b'W'),
                chunk.get(10).copied().unwrap_or(b'W'),
                chunk.get(14).copied().unwrap_or(b'W'),
                chunk.get(18).copied().unwrap_or(b'W'),
                chunk.get(22).copied().unwrap_or(b'W'),
                chunk.get(26).copied().unwrap_or(b'W'),
                chunk.get(30).copied().unwrap_or(b'W'),
            ]);

            opponent -= u8x8::splat(b'A');
            me -= u8x8::splat(b'W');

            let three = u8x8::splat(3);
            let diff = (me - opponent + three) % three;
            let res = me + diff * three;

            score + res.reduce_sum() as u16
        })
        .reduce(u16::default, <u16 as Add>::add)
}

#[cfg(feature = "nightly")]
#[allow(unused)]
pub fn part1_simd(input: &[u8]) -> u16 {
    use std::ops::Add;

    use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

    let (aligned, suffix) = input.split_at((2500 * 4 / 32) * 32);

    let aligned_score = aligned.chunks(32).fold(0, |score, chunk| {
        let mut opponent = u8x8::from_array(unsafe {
            [
                *chunk.get_unchecked(0),
                *chunk.get_unchecked(4),
                *chunk.get_unchecked(8),
                *chunk.get_unchecked(12),
                *chunk.get_unchecked(16),
                *chunk.get_unchecked(20),
                *chunk.get_unchecked(24),
                *chunk.get_unchecked(28),
            ]
        });

        let mut me = u8x8::from_array(unsafe {
            [
                *chunk.get_unchecked(2),
                *chunk.get_unchecked(6),
                *chunk.get_unchecked(10),
                *chunk.get_unchecked(14),
                *chunk.get_unchecked(18),
                *chunk.get_unchecked(22),
                *chunk.get_unchecked(26),
                *chunk.get_unchecked(30),
            ]
        });

        opponent -= u8x8::splat(b'A');
        me -= u8x8::splat(b'W');

        let three = u8x8::splat(3);
        let diff = (me - opponent + three) % three;
        let res = me + diff * three;

        score + res.reduce_sum() as u16
    });

    let suffix_score = {
        let mut opponent = u8x4::from_array(unsafe {
            [
                *suffix.get_unchecked(0),
                *suffix.get_unchecked(4),
                *suffix.get_unchecked(8),
                *suffix.get_unchecked(12),
            ]
        });

        let mut me = u8x4::from_array(unsafe {
            [
                *suffix.get_unchecked(2),
                *suffix.get_unchecked(6),
                *suffix.get_unchecked(10),
                *suffix.get_unchecked(14),
            ]
        });

        opponent -= u8x4::splat(b'A');
        me -= u8x4::splat(b'W');

        let three = u8x4::splat(3);
        let diff = (me - opponent + three) % three;
        let res = me + diff * three;

        res.reduce_sum() as u16
    };

    aligned_score + suffix_score
}

#[allow(unused)]
pub fn part1_naive(input: &[u8]) -> Result<u16> {
    input
        .chunks(4)
        .try_fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => Ok(rps.draw(1)),
                [b'A', _, b'Y'] => Ok(rps.win(2)),
                [b'A', _, b'Z'] => Ok(rps.loss(3)),
                [b'B', _, b'X'] => Ok(rps.loss(1)),
                [b'B', _, b'Y'] => Ok(rps.draw(2)),
                [b'B', _, b'Z'] => Ok(rps.win(3)),
                [b'C', _, b'X'] => Ok(rps.win(1)),
                [b'C', _, b'Y'] => Ok(rps.loss(2)),
                [b'C', _, b'Z'] => Ok(rps.draw(3)),
                chunk => Err(Report::msg(format!("invalid chunk `{chunk:?}`"))),
            }
        })
        .map(|rps| rps.score)
}

fn part2(input: &[u8]) -> Result<u16> {
    input
        .chunks(4)
        .try_fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => Ok(rps.loss(3)),
                [b'A', _, b'Y'] => Ok(rps.draw(1)),
                [b'A', _, b'Z'] => Ok(rps.win(2)),
                [b'B', _, b'X'] => Ok(rps.loss(1)),
                [b'B', _, b'Y'] => Ok(rps.draw(2)),
                [b'B', _, b'Z'] => Ok(rps.win(3)),
                [b'C', _, b'X'] => Ok(rps.loss(2)),
                [b'C', _, b'Y'] => Ok(rps.draw(3)),
                [b'C', _, b'Z'] => Ok(rps.win(1)),
                chunk => Err(Report::msg(format!("invalid chunk `{chunk:?}`"))),
            }
        })
        .map(|rps| rps.score)
}

#[derive(Default)]
struct RockPaperScissors {
    score: u16,
}

impl RockPaperScissors {
    fn win(mut self, shape: u16) -> Self {
        self.score += shape + 6;

        self
    }

    fn draw(mut self, shape: u16) -> Self {
        self.score += shape + 3;

        self
    }

    fn loss(mut self, shape: u16) -> Self {
        self.score += shape;

        self
    }
}
