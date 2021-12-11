use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use util::{Matrix, Pos2};

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
    let mut grid = parse_grid()?;
    println!("Setup: {:?}", start.elapsed()); // 21µs

    let start = Instant::now();
    let p1 = part1(&mut grid);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 311µs

    let start = Instant::now();
    let p2 = part2(&mut grid);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 368µs

    assert_eq!(p1, 1665);
    assert_eq!(p2, 235);

    Ok(())
}

fn parse_grid() -> Result<Matrix<u8>, Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut grid = Vec::new();

    input.read_line(&mut line)?;
    let line_ = line.trim_end().as_bytes();
    let w = line_.len();
    grid.extend(line_.iter().map(|byte| byte & 0x0F));
    line.clear();

    while input.read_line(&mut line)? != 0 {
        let row = line.trim_end().as_bytes().iter().map(|byte| byte & 0x0F);
        grid.extend(row);
        line.clear();
    }

    Ok(Matrix::from_vec(grid, w))
}

fn part1(grid: &mut Matrix<u8>) -> usize {
    (0..100).map(|_| step(grid)).sum()
}

fn part2(grid: &mut Matrix<u8>) -> usize {
    (101..).find(|_| step(grid) == grid.len()).unwrap()
}

fn step(grid: &mut Matrix<u8>) -> usize {
    let mut flashes = 0;

    for pos in grid.pos_iter() {
        let value = &mut grid[pos];
        *value += 1;

        if *value == 10 {
            flashes += 1;
            increment_neighbors(grid, pos, &mut flashes);
        }
    }

    if flashes > 0 {
        grid.iter_mut().filter(|v| **v >= 10).for_each(|v| *v = 0);
    }

    flashes
}

fn increment_neighbors(grid: &mut Matrix<u8>, pos: Pos2<usize>, flashes: &mut usize) {
    for n in grid.neighbors_8(pos) {
        let value = &mut grid[n];
        *value += 1;

        if *value == 10 {
            *flashes += 1;
            increment_neighbors(grid, n, flashes);
        }
    }
}
