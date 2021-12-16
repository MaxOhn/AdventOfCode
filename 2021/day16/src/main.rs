use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    input.read_line(&mut line)?;

    let mut binary: Vec<u8> = Vec::with_capacity(line.len() * 4);

    for c in line.trim_end().chars() {
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
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 125µs

    assert_eq!(p1, 886);
    assert_eq!(p2, 184_487_454_837);

    Ok(())
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

    if type_id == 4 {
        return gather_literal(bytes, len);
    }

    let len_type_id = bytes[*len];
    *len += 1;

    let values = match len_type_id {
        0 => gather_subpackets_by_len(bytes, versions, len),
        1 => gather_subpackets_by_count(bytes, versions, len),
        _ => unreachable!(),
    };

    match type_id {
        0 => values.into_iter().sum(),
        1 => values.into_iter().product(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => (values[0] > values[1]) as u64,
        6 => (values[0] < values[1]) as u64,
        7 => (values[0] == values[1]) as u64,
        _ => unreachable!(),
    }
}

fn gather_literal(bytes: &[u8], len: &mut usize) -> u64 {
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

fn gather_subpackets_by_len(bytes: &[u8], versions: &mut u32, len: &mut usize) -> Vec<u64> {
    let mut values = Vec::new();
    let number = parse!(bytes[*len..15] => usize);
    let goal = *len + number;

    while *len < goal {
        values.push(process_packet(bytes, versions, len));
    }

    values
}

fn gather_subpackets_by_count(bytes: &[u8], versions: &mut u32, len: &mut usize) -> Vec<u64> {
    let count = parse!(bytes[*len..11] => usize);

    (0..count)
        .map(|_| process_packet(bytes, versions, len))
        .collect()
}
