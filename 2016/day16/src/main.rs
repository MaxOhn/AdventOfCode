use std::time::Instant;

#[macro_use]
extern crate util;

const DISK_SIZE_PART1: usize = 272;
const DISK_SIZE_PART2: usize = 35_651_584;

fn main() {
    let mut input = std::fs::read("./input").unwrap();

    for byte in input.iter_mut() {
        *byte = *byte & 1;
    }

    let mut input_clone = input.clone();

    let start = Instant::now();
    input_clone.reserve(DISK_SIZE_PART1 * 2);
    let p1 = run(input_clone, DISK_SIZE_PART1);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    input.reserve(DISK_SIZE_PART2 * 2);
    let p2 = run(input, DISK_SIZE_PART2);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, "11111000111110000");
    assert_eq!(p2, "10111100110110100");
}

fn run(mut data: Vec<u8>, size: usize) -> String {
    let mut buf = vec![0; size * 2];

    while data.len() < size {
        for i in 0..data.len() {
            set!(buf, data.len() - 1 - i, 1 - get!(data, i));
        }

        data.push(0);
        data.extend(buf.iter().take(data.len() - 1));
    }

    data.truncate(size);
    buf.clear();

    loop {
        for chunk in data.chunks_exact(2) {
            buf.push((get!(chunk, 0) == get!(chunk, 1)) as u8);
        }

        if buf.len() % 2 == 1 {
            break;
        }

        data.clear();
        swap!(&mut data, &mut buf);
    }

    buf.into_iter().map(|byte| (byte + b'0') as char).collect()
}
