use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    mem,
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

    let algorithm: Vec<_> = line.trim_end().as_bytes().iter().map(parse_byte).collect();

    input.read_line(&mut line)?;
    line.clear();

    let mut grid = Vec::new();

    input.read_line(&mut line)?;
    grid.extend(line.trim_end().as_bytes().iter().map(parse_byte));
    let w = line.len() - 1;
    line.clear();

    while input.read_line(&mut line)? != 0 {
        grid.extend(line.trim_end().as_bytes().iter().map(parse_byte));
        line.clear();
    }

    let mut map = HashMap::with_capacity(w * w);

    for (x, chunk) in grid.chunks_exact(w).enumerate() {
        for (y, &byte) in chunk.iter().enumerate() {
            map.insert((y as isize, x as isize), byte);
        }
    }

    println!("Setup: {:?}", start.elapsed()); // 896µs

    let mut next_map = HashMap::with_capacity(map.len());
    let mut p1_map = map.clone();
    let start = Instant::now();
    let p1 = solve(&mut p1_map, &mut next_map, &algorithm, w as isize, 2);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 6.9ms

    let start = Instant::now();
    let p2 = solve(&mut map, &mut next_map, &algorithm, w as isize, 50);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 351ms

    assert_eq!(p1, 5680);
    assert_eq!(p2, 19_766);

    Ok(())
}

type Map = HashMap<(isize, isize), u8>;

fn solve(map: &mut Map, next_map: &mut Map, algorithm: &[u8], w: isize, iterations: u8) -> usize {
    for i in 0..iterations {
        for x in -(i as isize) - 1..=w + i as isize {
            for y in -(i as isize) - 1..=w + i as isize {
                let pos = (x, y);

                let idx = OFFSETS
                    .iter()
                    .map(|(x, y)| (pos.0 + x, pos.1 + y))
                    .map(|pos| map.get(&pos).map_or(i as u8 % 2, |v| *v))
                    .fold(0, |binary, bit| binary * 2 + bit as usize);

                next_map.insert(pos, algorithm[idx]);
            }
        }

        mem::swap(map, next_map);
    }

    map.values().filter(|&value| *value == 1).count()
}

fn parse_byte(byte: &u8) -> u8 {
    match *byte {
        b'.' => 0,
        b'#' => 1,
        _ => unreachable!(),
    }
}

static OFFSETS: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
