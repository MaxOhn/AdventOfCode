use std::mem;

use memchr::memchr_iter;

use crate::prelude::Solution;

fn char_to_idx(c: &u8) -> usize {
    match *c {
        b'a'..=b'z' => (*c - b'a') as usize,
        c => (c - b'A' + 26) as usize,
    }
}

pub fn run(input: &[u8]) -> Solution {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut p1_seen = [false; 26 * 2];
    let mut p2_count1 = [0; 26 * 2];
    let mut p2_count2 = [0; 26 * 2];

    let lines = memchr_iter(b'\n', input).scan(0, |i, j| Some(&input[mem::replace(i, j + 1)..j]));

    for (i, line) in lines.enumerate() {
        let (front, back) = line.split_at(line.len() / 2);

        front
            .iter()
            .map(char_to_idx)
            .for_each(|i| set!(p1_seen[i] = true));

        let dup = back
            .iter()
            .map(char_to_idx)
            .find(|&i| get!(p1_seen[i]))
            .unwrap();

        p1_seen.iter_mut().for_each(|seen| *seen = false);

        p1 += 1 + dup as u32;

        match i % 3 {
            0 => line
                .iter()
                .map(char_to_idx)
                .for_each(|i| set!(p2_count1[i] += 1)),
            1 => line
                .iter()
                .map(char_to_idx)
                .for_each(|i| set!(p2_count2[i] += 1)),
            2 => {
                let dup = line
                    .iter()
                    .map(char_to_idx)
                    .find(|&i| (get!(p2_count1[i]) > 0) && (get!(p2_count2[i]) > 0))
                    .unwrap();

                p2_count1
                    .iter_mut()
                    .zip(p2_count2.iter_mut())
                    .for_each(|(c1, c2)| {
                        *c1 = 0;
                        *c2 = 0;
                    });

                p2 += 1 + dup as u32;
            }
            _ => unreachable!(),
        }
    }

    Solution::new().part1(p1).part2(p2)
}
