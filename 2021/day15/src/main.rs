use std::{
    collections::BinaryHeap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    iter,
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
    let matrix = part1_matrix()?;
    let p1 = solve(matrix);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 895µs

    let start = Instant::now();
    let matrix = part2_matrix()?;
    let p2 = solve(matrix);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 22ms

    assert_eq!(p1, 498);
    assert_eq!(p2, 2901);

    Ok(())
}

fn part1_matrix() -> Result<Matrix<u8>, Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut grid = Vec::new();

    input.read_line(&mut line)?;

    let row = line.trim_end().as_bytes().iter().map(|&b| (b & 0x0F) as u8);

    grid.extend(row);
    let w = grid.len();
    line.clear();

    while input.read_line(&mut line)? != 0 {
        let row = line.trim_end().as_bytes().iter().map(|&b| (b & 0x0F) as u8);

        grid.extend(row);
        line.clear();
    }

    Ok(Matrix::from_vec(grid, w))
}

fn part2_matrix() -> Result<Matrix<u8>, Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut grid = Vec::new();

    input.read_line(&mut line)?;
    let line_ = line.trim_end().as_bytes();

    for i in 0..5 {
        let row = line_
            .iter()
            .map(|&b| (b & 0x0F) as u8 + i)
            .map(|n| n - (n > 9) as u8 * 9);

        grid.extend(row);
    }

    let w = grid.len();
    line.clear();

    while input.read_line(&mut line)? != 0 {
        let line_ = line.trim_end().as_bytes();

        for i in 0..5 {
            let row = line_
                .iter()
                .map(|&b| (b & 0x0F) as u8 + i)
                .map(|n| n - (n > 9) as u8 * 9);

            grid.extend(row);
        }

        line.clear();
    }

    grid.reserve(4 * grid.len());
    let original = grid.clone();

    for i in 1..=4 {
        let row = original
            .iter()
            .map(|&n| n + i)
            .map(|n| n - (n > 9) as u8 * 9);

        grid.extend(row);
    }

    Ok(Matrix::from_vec(grid, w))
}

fn solve(matrix: Matrix<u8>) -> u16 {
    let w = matrix.width();
    let h = matrix.height();

    let start = Pos2::new(0, 0);
    let end = Pos2::new(w - 1, h - 1);

    let dists: Vec<_> = iter::repeat(u16::MAX).take(w * h).collect();
    let mut dists = Matrix::from_vec(dists, w);
    dists[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State::new(start));

    while let Some(State { pos, cost }) = heap.pop() {
        if pos == end {
            break;
        } else if cost > dists[pos] {
            continue;
        }

        for n in matrix.neighbors_4(pos) {
            let cost = cost + matrix[n] as u16;

            if cost < dists[n] {
                dists[n] = cost;
                heap.push(State { pos: n, cost });
            }
        }
    }

    dists[end]
}

#[derive(Eq, PartialEq)]
struct State {
    pos: Pos2<usize>,
    cost: u16,
}

impl State {
    fn new(pos: Pos2<usize>) -> Self {
        Self { pos, cost: 0 }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
