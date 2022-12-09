use std::{collections::HashSet, hash::Hash};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = simulate_rope::<2>(input)?;
    let p2 = simulate_rope::<10>(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn simulate_rope<const N: usize>(input: &str) -> Result<usize> {
    let mut rope = [Pos::default(); N];
    let mut seen = HashSet::new();

    for line in input.lines() {
        let (direction, num) = line.split_once(' ').wrap_err("invalid line")?;
        let count = num.parse().wrap_err("invalid num")?;

        for _ in 0..count {
            let head = get_mut!(rope[0]);

            match direction {
                "R" => head.x += 1,
                "U" => head.y += 1,
                "L" => head.x -= 1,
                "D" => head.y -= 1,
                _ => bail!("invalid direction `{direction}`"),
            }

            for i in 1..N {
                move_successor(get!(rope[i - 1]), get_mut!(rope[i]));
            }

            seen.insert(get!(rope[N - 1]));
        }
    }

    Ok(seen.len())
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

fn move_successor(curr: Pos, succ: &mut Pos) {
    let dx = curr.x - succ.x;
    let dy = curr.y - succ.y;

    if dx.abs() + dy.abs() == 4 {
        succ.x += dx.signum();
        succ.y += dy.signum();
    } else if dx.abs() > 1 {
        succ.x += dx.signum();
        succ.y += (dy != 0) as i32 * dy;
    } else if dy.abs() > 1 {
        succ.y += dy.signum();
        succ.x += (dx != 0) as i32 * dx;
    }
}
