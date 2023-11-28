#[macro_use]
extern crate aoc_rust;

use std::str::{from_utf8 as str_from_utf8, FromStr};

use aoc_rust::Solution;

pub fn run(input: &str) -> eyre::Result<Solution> {
    let ops: Vec<_> = input
        .lines()
        .map(Op::from_str)
        .map(Result::unwrap)
        .collect();

    Ok(Solution::new().part1(part1(&ops)).part2(part2(&ops)))
}

pub fn part1(ops: &[Op]) -> usize {
    execute([0; 4], ops)
}

pub fn part2(ops: &[Op]) -> usize {
    let mut regs = [0; 4];
    set!(regs, 2, 1);

    execute(regs, ops)
}

fn execute(mut regs: [usize; 4], ops: &[Op]) -> usize {
    let mut pc = 0;

    while pc < ops.len() {
        match get!(ops, pc) {
            Op::Cpy(src, trg) => match src {
                Source::Value(val) => set!(regs, trg as usize, val),
                Source::Register(reg) => set!(regs, trg as usize, regs[reg as usize]),
            },
            Op::Inc(reg) => *get_mut!(regs, reg as usize) += 1,
            Op::Dec(reg) => *get_mut!(regs, reg as usize) -= 1,
            Op::Jnz(src, jmp) => {
                let cnd = match src {
                    Source::Value(val) => val != 0,
                    Source::Register(reg) => get!(regs, reg as usize) != 0,
                };

                if cnd {
                    pc = jmp.saturating_add(pc as isize) as usize;
                    continue;
                }
            }
        }

        pc += 1;
    }

    get!(regs, 0)
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Value(usize),
    Register(u8),
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Cpy(Source, u8),
    Inc(u8),
    Dec(u8),
    Jnz(Source, isize),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let op = match get_ref!(bytes, ..3) {
            b"cpy" => {
                let reg = get!(bytes, bytes.len() - 1) - b'a';
                let byte = get!(bytes, 4);

                let src = if byte.is_ascii_alphabetic() {
                    Source::Register(byte - b'a')
                } else {
                    Source::Value(
                        str_from_utf8(get_ref!(bytes, 4..bytes.len() - 2))
                            .unwrap()
                            .parse()
                            .unwrap(),
                    )
                };

                Op::Cpy(src, reg)
            }
            b"inc" => Op::Inc(get!(bytes, 4) - b'a'),
            b"dec" => Op::Dec(get!(bytes, 4) - b'a'),
            b"jnz" => {
                let mut i = 5;
                let byte = get!(bytes, 4);

                let src = if byte.is_ascii_alphabetic() {
                    i += 1;
                    Source::Register(byte - b'a')
                } else {
                    let mut n = (byte & 0xF) as usize;

                    loop {
                        match get!(bytes, i) {
                            b' ' => break,
                            b => n = n * 10 + (b & 0xF) as usize,
                        }

                        i += 1;
                    }

                    i += 1;

                    Source::Value(n)
                };

                let val = str_from_utf8(get_ref!(bytes, i..))
                    .unwrap()
                    .parse()
                    .unwrap();

                Op::Jnz(src, val)
            }
            _ => return Err(()),
        };

        Ok(op)
    }
}
