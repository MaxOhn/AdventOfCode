use std::{cmp::Reverse, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Result};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn parse_all(input: &str) -> Result<Vec<Self>> {
        let mut boxes = Vec::new();

        for line in input.lines() {
            boxes.push(line.parse()?);
        }

        Ok(boxes)
    }

    fn all_pairs(boxes: &[Self]) -> Vec<(usize, usize)> {
        let mut pairs = Vec::with_capacity((boxes.len() * (boxes.len() - 1)) / 2);

        for i in 0..boxes.len() - 1 {
            for j in i + 1..boxes.len() {
                pairs.push((i, j));
            }
        }

        pairs.sort_by_cached_key(|(i, j)| boxes[*i].dist_sq(boxes[*j]));

        pairs
    }

    fn dist_sq(&self, other: Self) -> i64 {
        // skipping sqrt because it's not necessary for sorting purposes
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(str::parse);

        Ok(Self {
            x: nums.next().wrap_err("missing x")??,
            y: nums.next().wrap_err("missing y")??,
            z: nums.next().wrap_err("missing z")??,
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    let boxes = JunctionBox::parse_all(input)?;
    let pairs = JunctionBox::all_pairs(&boxes);

    let mut circuits = vec![None; boxes.len()];
    let mut next_circuit = 0_u16;

    const CABLES: usize = 1000;
    let mut cables_left = CABLES;

    for (i, j) in pairs {
        match (circuits[i], circuits[j]) {
            (None, None) => {
                circuits[i] = Some(next_circuit);
                circuits[j] = Some(next_circuit);
                next_circuit += 1;
            }
            (None, c @ Some(_)) => circuits[i] = c,
            (c @ Some(_), None) => circuits[j] = c,
            (Some(a), Some(b)) if a == b => {}
            (Some(a), Some(b)) => overwrite_circuit_idx(&mut circuits, a, b),
        }

        cables_left -= 1;

        if cables_left == 0 {
            break;
        }
    }

    let mut indices: Vec<_> = circuits.into_iter().flatten().collect();
    indices.sort_unstable();

    let mut lengths = Vec::new();
    let mut len = 0;
    let mut curr = indices[0];

    for i in indices {
        if i == curr {
            len += 1;
        } else {
            lengths.push(len);
            len = 1;
            curr = i;
        }
    }

    lengths.push(len);

    let (biggest, _, _) = lengths.select_nth_unstable_by_key(3, |n| Reverse(*n));

    println!("{biggest:?}");

    Ok(biggest.into_iter().map(|a| *a).product())
}

fn part2(input: &str) -> Result<i64> {
    let boxes = JunctionBox::parse_all(input)?;
    let pairs = JunctionBox::all_pairs(&boxes);

    let mut circuits = vec![None; boxes.len()];
    let mut next_circuit = 0_u16;
    let mut n_circuits = boxes.len();

    for (i, j) in pairs {
        match (circuits[i], circuits[j]) {
            (None, None) => {
                circuits[i] = Some(next_circuit);
                circuits[j] = Some(next_circuit);
                next_circuit += 1;
            }
            (None, c @ Some(_)) => circuits[i] = c,
            (c @ Some(_), None) => circuits[j] = c,
            (Some(a), Some(b)) if a == b => continue,
            (Some(a), Some(b)) => overwrite_circuit_idx(&mut circuits, a, b),
        }

        n_circuits -= 1;

        if n_circuits == 1 {
            return Ok(boxes[i].x * boxes[j].x);
        }
    }

    unreachable!()
}

fn overwrite_circuit_idx(circuits: &mut [Option<u16>], from: u16, to: u16) {
    for circuit in circuits {
        if *circuit == Some(from) {
            *circuit = Some(to);
        }
    }
}
