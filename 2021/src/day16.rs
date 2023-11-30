use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut binary: Vec<u8> = Vec::with_capacity(input.len() * 4);

    for c in input.trim_end().chars() {
        match c {
            '0' => binary.extend([0, 0, 0, 0]),
            '1' => binary.extend([0, 0, 0, 1]),
            '2' => binary.extend([0, 0, 1, 0]),
            '3' => binary.extend([0, 0, 1, 1]),
            '4' => binary.extend([0, 1, 0, 0]),
            '5' => binary.extend([0, 1, 0, 1]),
            '6' => binary.extend([0, 1, 1, 0]),
            '7' => binary.extend([0, 1, 1, 1]),
            '8' => binary.extend([1, 0, 0, 0]),
            '9' => binary.extend([1, 0, 0, 1]),
            'A' => binary.extend([1, 0, 1, 0]),
            'B' => binary.extend([1, 0, 1, 1]),
            'C' => binary.extend([1, 1, 0, 0]),
            'D' => binary.extend([1, 1, 0, 1]),
            'E' => binary.extend([1, 1, 1, 0]),
            'F' => binary.extend([1, 1, 1, 1]),
            _ => unreachable!(),
        }
    }

    let mut p1 = 0;
    let mut len = 0;

    let p2 = process_packet(&binary, &mut p1, &mut len);

    Ok(Solution::new().part1(p1).part2(p2))
}

macro_rules! parse {
    ($bytes:ident[*$len:ident..$end:literal] => $ty:ty) => {
        $bytes[*$len..]
            .iter()
            .copied()
            .inspect(|_| *$len += 1)
            .take($end)
            .fold(0, |v, byte| v * 2 + byte as $ty)
    };
}

fn process_packet(bytes: &[u8], versions: &mut u32, len: &mut usize) -> u64 {
    *versions += parse!(bytes[*len..3] => u32);
    let type_id = parse!(bytes[*len..3] => u8);

    let process_subpackets = if bytes[*len] == 0 {
        process_subpackets_by_len
    } else {
        process_subpackets_by_count
    };

    match type_id {
        0 => process_subpackets(bytes, versions, len, |a, b| a + b),
        1 => process_subpackets(bytes, versions, len, |a, b| a * b),
        2 => process_subpackets(bytes, versions, len, std::cmp::min),
        3 => process_subpackets(bytes, versions, len, std::cmp::max),
        4 => process_literal(bytes, len),
        5 => process_two_subpackets(bytes, versions, len, |a, b| (a > b) as u64),
        6 => process_two_subpackets(bytes, versions, len, |a, b| (a < b) as u64),
        7 => process_two_subpackets(bytes, versions, len, |a, b| (a == b) as u64),
        _ => unreachable!(),
    }
}

fn process_literal(bytes: &[u8], len: &mut usize) -> u64 {
    let mut literal = 0;

    for chunk in bytes[*len..].chunks_exact(5) {
        *len += 5;

        for &byte in &chunk[1..] {
            literal = literal * 2 + byte as u64;
        }

        if chunk[0] == 0 {
            break;
        }
    }

    literal
}

fn process_subpackets_by_len(
    bytes: &[u8],
    versions: &mut u32,
    len: &mut usize,
    fold_op: fn(u64, u64) -> u64,
) -> u64 {
    *len += 1;
    let number = parse!(bytes[*len..15] => usize);
    let goal = *len + number;
    let mut value = process_packet(bytes, versions, len);

    while *len < goal {
        value = fold_op(value, process_packet(bytes, versions, len));
    }

    value
}

fn process_subpackets_by_count(
    bytes: &[u8],
    versions: &mut u32,
    len: &mut usize,
    fold_op: fn(u64, u64) -> u64,
) -> u64 {
    *len += 1;
    let count = parse!(bytes[*len..11] => usize);
    let value = process_packet(bytes, versions, len);

    (0..count - 1)
        .map(|_| process_packet(bytes, versions, len))
        .fold(value, fold_op)
}

fn process_two_subpackets(
    bytes: &[u8],
    versions: &mut u32,
    len: &mut usize,
    cmp_op: fn(u64, u64) -> u64,
) -> u64 {
    let len_type_id = bytes[*len];
    *len += 12 + 4 * (len_type_id == 0) as usize;

    let a = process_packet(bytes, versions, len);
    let b = process_packet(bytes, versions, len);

    cmp_op(a, b)
}
