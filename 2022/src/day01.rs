use std::iter;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut max = 0;
    let mut mid = 0;
    let mut min = 0;

    let mut sum = 0;

    for line in input.lines().chain(iter::once("")) {
        if line.is_empty() {
            if sum > max {
                min = mid;
                mid = max;
                max = sum;
            } else if sum > mid {
                min = mid;
                mid = sum;
            } else if sum > min {
                min = sum;
            }

            sum = 0;
        } else {
            sum += line.parse::<u32>().map_err(|_| eyre!("invalid number"))?;
        }
    }

    Ok(Solution::new().part1(max).part2(max + mid + min))
}
