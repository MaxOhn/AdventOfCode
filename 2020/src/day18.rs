use std::hint::unreachable_unchecked;

use aoc_rust::Solution;
use eyre::Result;

enum Op {
    Add,
    Mul,
}

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let (n, _) = eval_part1(line.as_bytes());
        sum += n;
    }

    sum
}

fn eval_part1(bytes: &[u8]) -> (u64, usize) {
    let mut eval = 0;
    let mut buf = 0;
    let mut idx = 0;

    loop {
        match get!(bytes, idx) {
            b' ' => break,
            b'(' => {
                let (n, i) = eval_part1(unsafe { bytes.get_unchecked(idx + 1..) });
                eval = n;
                idx += i;
            }
            digit => eval = eval * 10 + (digit & 0x0F) as u64,
        }

        idx += 1;
    }

    idx += 1;

    let mut op = match get!(bytes, idx) {
        b'+' => Op::Add,
        b'*' => Op::Mul,
        _ => unsafe { unreachable_unchecked() },
    };

    idx += 2;

    while idx < bytes.len() {
        match get!(bytes, idx) {
            b' ' | b'\n' => {}
            b'+' => {
                match op {
                    Op::Add => eval += buf,
                    Op::Mul => eval *= buf,
                }

                buf = 0;
                idx += 1;
                op = Op::Add
            }
            b'*' => {
                match op {
                    Op::Add => eval += buf,
                    Op::Mul => eval *= buf,
                }

                buf = 0;
                idx += 1;
                op = Op::Mul
            }
            b'(' => {
                let (n, i) = eval_part1(unsafe { bytes.get_unchecked(idx + 1..) });
                buf = n;
                idx += i;
            }
            b')' => {
                idx += 1;
                break;
            }
            digit => buf = buf * 10 + (digit & 0x0F) as u64,
        }

        idx += 1;
    }

    match op {
        Op::Add => eval += buf,
        Op::Mul => eval *= buf,
    }

    (eval, idx)
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let (n, _) = eval_part2(line.as_bytes());
        sum += n;
    }

    sum
}

fn eval_part2(bytes: &[u8]) -> (u64, usize) {
    let mut eval = 0;
    let mut buf = 0;
    let mut idx = 0;

    loop {
        match get!(bytes, idx) {
            b' ' => break,
            b'(' => {
                let (n, i) = eval_part2(unsafe { bytes.get_unchecked(idx + 1..) });
                eval = n;
                idx += i;
            }
            digit => eval = eval * 10 + (digit & 0x0F) as u64,
        }

        idx += 1;
    }

    idx += 1;
    let mut op = match get!(bytes, idx) {
        b'+' => Op::Add,
        b'*' => Op::Mul,
        _ => unsafe { unreachable_unchecked() },
    };

    idx += 2;

    while idx < bytes.len() {
        match get!(bytes, idx) {
            b' ' | b'\n' => {}
            b'+' => match op {
                Op::Add => {
                    eval += buf;
                    idx += 1;
                    buf = 0;
                }
                Op::Mul => 'outer: loop {
                    idx += 2;
                    let (n, i) = eval_single_part2(unsafe { bytes.get_unchecked(idx..) });
                    idx += i;
                    buf += n;

                    while idx < bytes.len() {
                        match get!(bytes, idx) {
                            b'+' => continue 'outer,
                            b'*' | b')' => {
                                idx -= 1;
                                break 'outer;
                            }
                            _ => idx += 1,
                        }
                    }

                    break;
                },
            },
            b'*' => {
                match op {
                    Op::Add => eval += buf,
                    Op::Mul => eval *= buf,
                }

                buf = 0;
                idx += 1;
                op = Op::Mul
            }
            b'(' => {
                let (n, i) = eval_part2(unsafe { bytes.get_unchecked(idx + 1..) });
                buf = n;
                idx += i;
            }
            b')' => {
                idx += 1;
                break;
            }
            digit => buf = buf * 10 + (digit & 0x0F) as u64,
        }

        idx += 1;
    }

    match op {
        Op::Add => eval += buf,
        Op::Mul => eval *= buf,
    }

    (eval, idx)
}

fn eval_single_part2(bytes: &[u8]) -> (u64, usize) {
    let mut eval = 0;
    let mut idx = 0;

    while idx < bytes.len() {
        match get!(bytes, idx) {
            b' ' | b'\n' => break,
            b'(' => {
                let (n, i) = eval_part2(unsafe { bytes.get_unchecked(idx + 1..) });
                eval = n;
                idx += i;
            }
            b')' => return (eval, idx),
            digit => eval = eval * 10 + (digit & 0x0F) as u64,
        }

        idx += 1;
    }

    (eval, idx + 1)
}
