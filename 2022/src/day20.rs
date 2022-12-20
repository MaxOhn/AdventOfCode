use std::cmp::Ordering;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let original = input
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()?;

    let p1 = solve::<1, 1>(&original)?;
    let p2 = solve::<10, 811_589_153>(&original)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn solve<const ROUNDS: usize, const DECRYPTION_KEY: i64>(original: &[i64]) -> Result<i64> {
    let mut indices: Vec<_> = (0..).take(original.len()).collect();
    let wrap = original.len() as i64 - 1;

    for _ in 0..ROUNDS {
        for (orig_idx, n) in original.iter().map(|&n| n * DECRYPTION_KEY).enumerate() {
            let from = indices.iter().position(|&idx| idx == orig_idx).unwrap();
            let to = (from as i64 + n).rem_euclid(wrap) as usize;

            match from.cmp(&to) {
                Ordering::Less => indices[from..=to].rotate_left(1),
                Ordering::Greater => indices[to..=from].rotate_right(1),
                Ordering::Equal => {}
            }
        }
    }

    let zero_idx = indices
        .iter()
        .position(|&idx| original[idx] == 0)
        .wrap_err("missing 0")?;

    let sum = indices
        .iter()
        .cycle()
        .skip(zero_idx + 1000)
        .step_by(1000)
        .take(3)
        .map(|&idx| original[idx] * DECRYPTION_KEY)
        .sum();

    Ok(sum)
}
