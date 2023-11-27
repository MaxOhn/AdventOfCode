use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
    str::{FromStr, Lines},
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let mut stacks = parse_stacks(&mut lines)?;

    lines.next();

    let instructions = lines
        .map(str::parse)
        .collect::<Result<Vec<Instruction>>>()?;

    let p1 = part1(&mut stacks.clone(), &instructions);
    let p2 = part2(&mut stacks, &instructions);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_stacks(lines: &mut Lines<'_>) -> Result<Vec<Vec<u8>>> {
    let Some(line) = lines.next() else {
        return Err(Report::msg("missing crates"));
    };

    let mut stacks: Vec<_> = line
        .bytes()
        .skip(1)
        .step_by(4)
        .map(|c| if c != b' ' { vec![c] } else { Vec::new() })
        .collect();

    for line in lines {
        let bytes = line.as_bytes();

        if bytes.get(1).filter(|c| c.is_ascii_digit()).is_some() {
            break;
        }

        bytes
            .iter()
            .skip(1)
            .step_by(4)
            .zip(stacks.iter_mut())
            .filter(|(&c, _)| c != b' ')
            .for_each(|(&c, stack)| stack.push(c));
    }

    stacks
        .iter_mut()
        .map(Vec::deref_mut)
        .for_each(<[u8]>::reverse);

    Ok(stacks)
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let rest = line.trim_start_matches("move ");
        let (count, rest) = rest.split_once(' ').wrap_err("invalid line")?;
        let rest = rest.trim_start_matches("from ");
        let (from, rest) = rest.split_once(' ').wrap_err("invalid line")?;
        let to = rest.trim_start_matches("to ");

        let count: usize = count
            .parse()
            .map_err(|_| eyre!("invalid number for `count`"))?;

        let from = from
            .parse()
            .map(|n: usize| n - 1)
            .map_err(|_| eyre!("invalid number for `from`"))?;

        let to = to
            .parse()
            .map(|n: usize| n - 1)
            .map_err(|_| eyre!("invalid number for `to`"))?;

        Ok(Self { from, to, count })
    }
}

fn part1(stacks: &mut [Vec<u8>], instructions: &[Instruction]) -> String {
    for instruction in instructions {
        let Instruction { from, to, count } = instruction;

        let (from, to) = from_to(stacks, *from, *to);
        let new_len = from.len() - count;
        let drain = from.drain(new_len..).rev();
        to.extend(drain);
    }

    read_top_crates(stacks)
}

fn part2(stacks: &mut [Vec<u8>], instructions: &[Instruction]) -> String {
    for instruction in instructions {
        let Instruction { from, to, count } = instruction;

        let (from, to) = from_to(stacks, *from, *to);
        let new_len = from.len() - count;
        to.extend_from_slice(&from[new_len..]);
        from.truncate(new_len);
    }

    read_top_crates(stacks)
}

fn read_top_crates(stacks: &[Vec<u8>]) -> String {
    stacks
        .iter()
        .map(Vec::deref)
        .filter_map(<[u8]>::last)
        .copied()
        .map(char::from)
        .collect()
}

fn from_to(stacks: &mut [Vec<u8>], from: usize, to: usize) -> (&mut Vec<u8>, &mut Vec<u8>) {
    match from.cmp(&to) {
        Ordering::Less => {
            let (front, back) = stacks.split_at_mut(to);

            (get_mut!(front, from), &mut back[0])
        }
        Ordering::Greater => {
            let (front, back) = stacks.split_at_mut(from);

            (&mut back[0], get_mut!(front, to))
        }
        Ordering::Equal => unreachable!("from == to"),
    }
}
