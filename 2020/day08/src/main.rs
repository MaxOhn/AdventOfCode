use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

static mut SEEN: [bool; 1024] = [false; 1024];

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut instructions: Vec<Op> = Vec::with_capacity(640);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        instructions.push(parse_op(&line));
        line.clear();
    }

    println!("Setup: {:?}", start.elapsed()); // 143µs

    let start = Instant::now();
    let (p1_result, max) = part1(&instructions);
    let p1 = p1_result.unwrap_err();
    clear_seen(max);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 1.7µs

    let start = Instant::now();
    let p2 = part2(&mut instructions);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 79µs

    assert_eq!(p1, 2014);
    assert_eq!(p2, 2251);
}

fn part1(instructions: &[Op]) -> (Result<i32, i32>, usize) {
    let mut acc = 0;
    let mut pc = 0;
    let mut prev;
    let mut max = 0;

    while (pc as usize) < instructions.len() {
        prev = acc;

        match unsafe { instructions.get_unchecked(pc as usize) } {
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Op::Jmp(n) => pc += n,
            Op::Nop(_) => pc += 1,
        }

        if unsafe { *SEEN.get_unchecked(pc as usize) } {
            return (Err(prev), max as usize);
        }

        max = max.max(pc);
        unsafe { *SEEN.get_unchecked_mut(pc as usize) = true }
    }

    (Ok(acc), max as usize)
}

fn part2(instructions: &mut [Op]) -> i32 {
    let mut i = 0;

    loop {
        let replaced = loop {
            match unsafe { *instructions.get_unchecked(i) } {
                Op::Acc(_) => i += 1,
                Op::Jmp(n) => {
                    unsafe { *instructions.get_unchecked_mut(i) = Op::Nop(n) }
                    break Op::Jmp(n);
                }
                Op::Nop(n) => {
                    unsafe { *instructions.get_unchecked_mut(i) = Op::Jmp(n) }
                    break Op::Nop(n);
                }
            }
        };

        match part1(&instructions) {
            (Ok(n), _) => return n,
            (Err(_), max) => clear_seen(max),
        };

        unsafe { *instructions.get_unchecked_mut(i) = replaced }
        i += 1;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse_op(line: &str) -> Op {
    let n = parse(unsafe { line.trim_end().as_bytes().get_unchecked(4..) });

    match unsafe { line.as_bytes().get_unchecked(0) } {
        b'a' => Op::Acc(n),
        b'j' => Op::Jmp(n),
        b'n' => Op::Nop(n),
        _ => unsafe { unreachable_unchecked() },
    }
}

fn clear_seen(max: usize) {
    let mut j = 0;

    while j <= max {
        unsafe { *SEEN.get_unchecked_mut(j) = false }
        j += 1;
    }
}

fn parse(bytes: &[u8]) -> i32 {
    let mut n = 0;
    let mut i = 1;
    let sig = (unsafe { *bytes.get_unchecked(0) } == b'+') as i32 * 2 - 1;

    while i < bytes.len() {
        let c = unsafe { *bytes.get_unchecked(i) };

        if c == b'\r' {
            return n * sig;
        }

        n = n * 10 + (unsafe { *bytes.get_unchecked(i) } & 0x0F) as i32;
        i += 1;
    }

    n * sig
}
