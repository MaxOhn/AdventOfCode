use std::{collections::HashMap, mem};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let algorithm: Vec<_> = lines
        .next()
        .unwrap()
        .trim_end()
        .as_bytes()
        .iter()
        .map(parse_byte)
        .collect();

    lines.next();

    let mut grid = Vec::new();

    let line = lines.next().unwrap();
    grid.extend(line.trim_end().as_bytes().iter().map(parse_byte));
    let w = line.len() - 1;

    for line in lines {
        grid.extend(line.trim_end().as_bytes().iter().map(parse_byte));
    }

    let mut map = HashMap::with_capacity(w * w);

    for (x, chunk) in grid.chunks_exact(w).enumerate() {
        for (y, &byte) in chunk.iter().enumerate() {
            map.insert((y as isize, x as isize), byte);
        }
    }

    let mut next_map = HashMap::with_capacity(map.len());
    let mut p1_map = map.clone();
    let p1 = solve(&mut p1_map, &mut next_map, &algorithm, w as isize, 2);
    let p2 = solve(&mut map, &mut next_map, &algorithm, w as isize, 50);

    Ok(Solution::new().part1(p1).part2(p2))
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
