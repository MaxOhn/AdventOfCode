use std::ops::ControlFlow;

use aoc_rust::{util::numbers, Solution};
use eyre::{ContextCompat, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.trim().lines();

    let dirs = lines.next().wrap_err("missing instructions")?;

    lines.next().wrap_err("missing blank line")?;

    let mut instructions = lines
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()
        .wrap_err("invalid instruction")?;

    instructions.sort_unstable_by_key(|instr| instr.from);

    let p1 = part1(dirs.bytes().cycle(), &instructions)?;
    let p2 = part2(dirs.bytes().cycle(), &instructions)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(dirs: impl Iterator<Item = u8>, instructions: &[Instruction]) -> Result<u32> {
    let steps = dirs.zip(1..).try_fold("AAA", |mut curr, (dir, step)| {
        if let Err(err) = process_curr(&mut curr, dir, instructions) {
            return ControlFlow::Break(Err(err));
        }

        if curr.ends_with('Z') {
            ControlFlow::Break(Ok(step))
        } else {
            ControlFlow::Continue(curr)
        }
    });

    let ControlFlow::Break(steps) = steps else {
        unreachable!()
    };

    steps
}

fn part2(dirs: impl Iterator<Item = u8>, instructions: &[Instruction]) -> Result<u64> {
    let mut currs: Vec<_> = instructions
        .iter()
        .filter_map(|instr| instr.from.ends_with('A').then_some(instr.from))
        .collect();

    let steps = dirs.zip(1..).try_fold(1, |mut steps, (dir, step)| {
        currs.retain_mut(|curr| {
            process_curr(curr, dir, instructions).unwrap();

            if curr.ends_with('Z') {
                steps = numbers::lcm(steps, step);

                false
            } else {
                true
            }
        });

        if currs.is_empty() {
            ControlFlow::Break(steps)
        } else {
            ControlFlow::Continue(steps)
        }
    });

    let ControlFlow::Break(steps) = steps else {
        unreachable!()
    };

    Ok(steps)
}

fn process_curr<'a>(curr: &mut &'a str, dir: u8, instructions: &[Instruction<'a>]) -> Result<()> {
    let idx = instructions
        .binary_search_by_key(curr, |instr| instr.from)
        .map_err(|_| eyre::eyre!("missing instruction for `{curr}`"))?;

    match dir {
        b'L' => *curr = instructions[idx].left,
        b'R' => *curr = instructions[idx].right,
        _ => return Err(eyre::eyre!("invalid direction byte `{dir}`")),
    }

    Ok(())
}

struct Instruction<'a> {
    from: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Instruction<'a> {
    fn from_str(line: &'a str) -> Result<Self> {
        let (from, back) = line.split_once(" = ").wrap_err("missing equal")?;

        let (left, right) = back
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .wrap_err("missing comma")?;

        Ok(Self { from, left, right })
    }
}
