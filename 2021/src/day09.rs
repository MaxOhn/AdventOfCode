use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use aoc_rust::Solution;
use eyre::Result;

use crate::util::{Matrix, Pos2};

pub fn run(input: &str) -> Result<Solution> {
    let mut grid = Vec::new();
    let mut w = 0;

    for line in input.lines() {
        let line = line.trim_end().as_bytes();
        w = line.len();
        grid.extend(line.iter().map(|&b| b - b'0'));
    }

    let grid = Matrix::from_vec(grid, w);

    let (p1, _) = part1(&grid);

    let (_, lows) = part1(&grid);
    let p2 = part2(&grid, &lows);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(grid: &Matrix<u8>) -> (u32, Vec<Pos2<isize>>) {
    let mut answer = 0;
    let mut lows = Vec::new();

    let w = grid.width() as isize;
    let h = grid.height() as isize;

    for x in 0..w {
        'outer: for y in 0..h {
            let curr = grid[y as usize][x as usize];

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let nx = x + i;
                    let ny = y + j;

                    if nx < 0 || ny < 0 || nx >= w || ny >= h {
                        continue;
                    }

                    if grid[ny as usize][nx as usize] <= curr {
                        continue 'outer;
                    }
                }
            }

            answer += curr as u32 + 1;
            lows.push(Pos2::new(x, y));
        }
    }

    (answer, lows)
}

fn part2(grid: &Matrix<u8>, lows: &[Pos2<isize>]) -> usize {
    let mut basins = Vec::with_capacity(lows.len());

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let w = grid.width() as isize;
    let h = grid.height() as isize;

    for &low in lows {
        let mut basin = HashSet::new();

        visited.clear();
        queue.clear();
        queue.push_front(low);

        while let Some(curr) = queue.pop_back() {
            if grid[curr.y as usize][curr.x as usize] == 9 {
                continue;
            }

            basin.insert(curr);

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == j || i == -j {
                        continue;
                    }

                    let n = curr + Pos2::new(i, j);

                    if n.x < 0 || n.y < 0 || n.x >= w || n.y >= h {
                        continue;
                    }

                    if visited.insert(n) {
                        queue.push_front(n);
                    }
                }
            }
        }

        basins.push(basin.len());
    }

    basins.sort_unstable_by_key(|&len| Reverse(len));
    basins.truncate(3);

    basins.iter().fold(1, |prod, len| prod * len)
}
