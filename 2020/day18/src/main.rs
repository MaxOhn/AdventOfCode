use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

macro_rules! get {
    ($bytes:ident, $i:ident) => {
        unsafe { *$bytes.get_unchecked($i) }
    };
}

enum Op {
    Add,
    Mul,
}

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 16_332_191_652_452);
    assert_eq!(p2, 351_175_492_232_654);
}

fn part1() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut sum = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let (n, _) = eval_part1(line.as_bytes());
        sum += n;
        line.clear();
    }

    println!("Part 1: {} [{:?}]", sum, start.elapsed()); // 191µs

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

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut sum = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let (n, _) = eval_part2(line.as_bytes());
        sum += n;
        line.clear();
    }

    println!("Part 2: {} [{:?}]", sum, start.elapsed()); // 141µs

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
