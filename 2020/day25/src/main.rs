use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let _ = input.read_line(&mut line);
    let card_key: usize = util::Parse::parse(line.as_bytes());
    line.clear();
    let _ = input.read_line(&mut line);
    let door_key: usize = util::Parse::parse(line.as_bytes());

    let mut val = 1;
    let mut encryption = 1;
    let subject_number = 7;

    while val != card_key {
        val = (subject_number * val) % 20_201_227;
        encryption = (encryption * door_key) % 20_201_227;
    }

    println!("Part 1: {} {:?}", encryption, start.elapsed()); // 41ms

    assert_eq!(encryption, 1_890_859);
}
