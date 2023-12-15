use std::collections::HashMap;

use aoc_rust::Solution;
use eyre::{Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<usize> {
    let mut sum = 0;

    for x in input.split(',') {
        sum += hash(x);
    }

    Ok(sum)
}

fn hash(s: &str) -> usize {
    let mut curr = 0;

    for byte in s.bytes() {
        curr += byte as usize;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn part2<'a>(input: &'a str) -> Result<usize> {
    let mut boxes = [0_u8; 256].map(|_| Vec::<(&str, usize)>::new());

    for x in input.split(',') {
        match x.split_once('=') {
            Some((label, value)) => {
                let h = hash(label);

                if let Some((_, lense)) = boxes[h].iter_mut().find(|(a, _)| *a == label) {
                    *lense = value.parse().unwrap();
                } else {
                    boxes[h].push((label, value.parse().unwrap()));
                }
            }
            None => {
                let label = &x[..x.len() - 1];
                let h = hash(label);

                if let Some(i) = boxes[h].iter().position(|(a, _)| *a == label) {
                    boxes[h].remove(i);
                }
            }
        }
    }

    let mut sum = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, f)) in b.iter().enumerate() {
            sum += (1 + i) * (j + 1) * f;
        }
    }

    Ok(sum)
}
