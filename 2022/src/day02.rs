use std::ops::Add;

use core_simd::simd::{u8x4, u8x8, SimdUint};
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

use crate::prelude::Solution;

pub fn run(input: &[u8]) -> Solution {
    // let p1 = part1_simd_rayon(input);
    let p1 = part1_simd(input);
    // let p1 = part1_naive(input);

    let p2 = part2(input);

    Solution::new().part1(p1).part2(p2)
}

#[allow(unused)]
pub fn part1_simd_rayon(input: &[u8]) -> u16 {
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

#[allow(unused)]
pub fn part1_simd(input: &[u8]) -> u16 {
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
pub fn part1_naive(input: &[u8]) -> u16 {
    input
        .chunks(4)
        .fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => rps.draw(1),
                [b'A', _, b'Y'] => rps.win(2),
                [b'A', _, b'Z'] => rps.loss(3),
                [b'B', _, b'X'] => rps.loss(1),
                [b'B', _, b'Y'] => rps.draw(2),
                [b'B', _, b'Z'] => rps.win(3),
                [b'C', _, b'X'] => rps.win(1),
                [b'C', _, b'Y'] => rps.loss(2),
                [b'C', _, b'Z'] => rps.draw(3),
                _ => unreachable!("{:?}", chunk),
            }
        })
        .score
}

fn part2(input: &[u8]) -> u16 {
    input
        .chunks(4)
        .fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => rps.loss(3),
                [b'A', _, b'Y'] => rps.draw(1),
                [b'A', _, b'Z'] => rps.win(2),
                [b'B', _, b'X'] => rps.loss(1),
                [b'B', _, b'Y'] => rps.draw(2),
                [b'B', _, b'Z'] => rps.win(3),
                [b'C', _, b'X'] => rps.loss(2),
                [b'C', _, b'Y'] => rps.draw(3),
                [b'C', _, b'Z'] => rps.win(1),
                _ => unreachable!("{:?}", chunk),
            }
        })
        .score
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
