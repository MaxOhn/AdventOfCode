use std::str::FromStr;
use std::time::Instant;

use util::Parse;

#[macro_use]
extern crate util;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let ops: Vec<_> = input
        .lines()
        .map(Op::from_str)
        .map(Result::unwrap)
        .collect();

    let start = Instant::now();
    let p1 = part1(&ops);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&ops);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 318_117);
    assert_eq!(p2, 9_227_771);
}

fn part1(ops: &[Op]) -> usize {
    run([0; 4], ops)
}

fn part2(ops: &[Op]) -> usize {
    let mut regs = [0; 4];
    set!(regs, 2, 1);

    run(regs, ops)
}

fn run(mut regs: [usize; 4], ops: &[Op]) -> usize {
    let mut pc = 0;

    while pc < ops.len() {
        match get!(ops, pc) {
            Op::Cpy(src, trg) => match src {
                Source::Value(val) => set!(regs, trg as usize, val),
                Source::Register(reg) => set!(regs, trg as usize, regs[reg as usize]),
            },
            Op::Inc(reg) => unsafe { *get_mut!(regs, reg as usize) += 1 },
            Op::Dec(reg) => unsafe { *get_mut!(regs, reg as usize) -= 1 },
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
enum Source {
    Value(usize),
    Register(u8),
}

#[derive(Copy, Clone, Debug)]
enum Op {
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
                    Source::Value(usize::parse(get_ref!(bytes, 4..bytes.len() - 2)))
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

                let val = isize::parse(get_ref!(bytes, i..));

                Op::Jnz(src, val)
            }
            _ => return Err(()),
        };

        Ok(op)
    }
}
