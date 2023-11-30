use std::{collections::BinaryHeap, iter};

use aoc_rust::Solution;
use eyre::Result;

use crate::util::{Matrix, Pos2};

pub fn run(input: &str) -> Result<Solution> {
    let matrix = part1_matrix(input)?;
    let p1 = solve(matrix);

    let matrix = part2_matrix(input)?;
    let p2 = solve(matrix);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1_matrix(input: &str) -> Result<Matrix<u8>> {
    let mut lines = input.lines();
    let mut grid = Vec::new();

    let row = lines
        .next()
        .unwrap()
        .trim_end()
        .as_bytes()
        .iter()
        .map(|&b| (b & 0x0F) as u8);

    grid.extend(row);
    let w = grid.len();

    for line in lines {
        let row = line.trim_end().as_bytes().iter().map(|&b| (b & 0x0F) as u8);
        grid.extend(row);
    }

    Ok(Matrix::from_vec(grid, w))
}

fn part2_matrix(input: &str) -> Result<Matrix<u8>> {
    let mut lines = input.lines();
    let mut grid = Vec::new();

    let line = lines.next().unwrap().trim_end().as_bytes();

    for i in 0..5 {
        let row = line
            .iter()
            .map(|&b| (b & 0x0F) as u8 + i)
            .map(|n| n - (n > 9) as u8 * 9);

        grid.extend(row);
    }

    let w = grid.len();

    for line in lines {
        let line = line.trim_end().as_bytes();

        for i in 0..5 {
            let row = line
                .iter()
                .map(|&b| (b & 0x0F) as u8 + i)
                .map(|n| n - (n > 9) as u8 * 9);

            grid.extend(row);
        }
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
