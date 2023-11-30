use aoc_rust::Solution;
use eyre::Result;

use crate::util::{Matrix, Pos2};

pub fn run(input: &str) -> Result<Solution> {
    let mut grid = parse_grid(input)?;

    let p1 = part1(&mut grid);
    let p2 = part2(&mut grid);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_grid(input: &str) -> Result<Matrix<u8>> {
    let mut lines = input.lines();
    let mut grid = Vec::new();

    let line = lines.next().unwrap().trim_end().as_bytes();
    let w = line.len();
    grid.extend(line.iter().map(|byte| byte & 0x0F));

    for line in lines {
        let row = line.trim_end().as_bytes().iter().map(|byte| byte & 0x0F);
        grid.extend(row);
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
