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

    let (p1, _, p2) = process_packet(&binary);
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 156µs

    assert_eq!(p1, 886);
    assert_eq!(p2, 184_487_454_837);

    Ok(())
}

fn process_packet(bytes: &[u8]) -> (u32, usize, u64) {
    let mut version = 0;

    for &byte in &bytes[..3] {
        version = version * 2 + byte as u32;
    }

    let mut type_id = 0;

    for &byte in &bytes[3..6] {
        type_id = type_id * 2 + byte;
    }

    let mut len = 6;

    if type_id == 4 {
        let mut literal = 0;

        for chunk in bytes[6..].chunks_exact(5) {
            len += 5;

            for &byte in &chunk[1..] {
                literal = literal * 2 + byte as u64;
            }

            if chunk[0] == 0 {
                break;
            }
        }

        return (version, len, literal);
    }

    len += 1;
    let mut values = Vec::new();

    match bytes[6] {
        0 => {
            let mut number = 0;
            len += 15;

            for &byte in &bytes[7..22] {
                number = number * 2 + byte as usize;
            }

            let goal = len + number;

            while len < goal {
                let (version_, len_, value) = process_packet(&bytes[len..]);
                version += version_;
                len += len_;
                values.push(value);
            }
        }
        1 => {
            let mut number = 0;
            len += 11;

            for &byte in &bytes[7..18] {
                number = number * 2 + byte;
            }

            for _ in 0..number {
                let (version_, len_, value) = process_packet(&bytes[len..]);
                version += version_;
                len += len_;
                values.push(value);
            }
        }
        _ => unreachable!(),
    };

    let value = match type_id {
        0 => values.into_iter().sum(),
        1 => values.into_iter().product(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => (values[0] > values[1]) as u64,
        6 => (values[0] < values[1]) as u64,
        7 => (values[0] == values[1]) as u64,
        _ => unreachable!(),
    };

    (version, len, value)
}
