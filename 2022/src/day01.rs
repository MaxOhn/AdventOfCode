use std::{iter, mem};

use aoc22::prelude::*;

pub fn run(input: &[u8]) -> Solution {
    let [p1, b, c] = input
        .split(|&byte| byte == b'\n')
        .map(<u32 as Parseable>::parse)
        .chain(iter::once(0))
        .scan(0, |sum, n| {
            *sum += n;

            Some((n == 0).then(|| mem::replace(sum, 0)))
        })
        .flatten()
        .fold([0, 0, 0], |[max, mid, min], sum| {
            if sum > max {
                [sum, max, mid]
            } else if sum > mid {
                [max, sum, mid]
            } else if sum > min {
                [max, mid, sum]
            } else {
                [max, mid, min]
            }
        });

    Solution::new().part1(p1).part2(p1 + b + c)
}
