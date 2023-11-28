#[macro_use]
extern crate aoc_rust;

use aoc_rust::Solution;

const DISK_SIZE_PART1: usize = 272;
const DISK_SIZE_PART2: usize = 35_651_584;

pub fn run(input: &str) -> eyre::Result<Solution> {
    let mut input = input.as_bytes().to_owned();

    for byte in input.iter_mut() {
        *byte = *byte & 1;
    }

    let mut input_clone = input.clone();

    input_clone.reserve(DISK_SIZE_PART1 * 2);
    let p1 = calc_checksum(input_clone, DISK_SIZE_PART1);

    input.reserve(DISK_SIZE_PART2 * 2);
    let p2 = calc_checksum(input, DISK_SIZE_PART2);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn calc_checksum(mut data: Vec<u8>, size: usize) -> String {
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
