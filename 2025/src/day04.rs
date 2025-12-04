use std::cmp;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    let mut grid: Vec<_> = input.lines().map(str::as_bytes).map(Box::from).collect();

    scan_diagram::<false>(&mut grid)
}

fn part2(input: &str) -> u32 {
    let mut grid: Vec<Box<[_]>> = input.lines().map(str::as_bytes).map(Box::from).collect();

    let mut total = 0;

    loop {
        let next = scan_diagram::<true>(&mut grid);

        if next == 0 {
            break;
        }

        total += next;
    }

    total
}

fn scan_diagram<const REMOVE: bool>(grid: &mut [Box<[u8]>]) -> u32 {
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut count = 0;

            for ny in y.saturating_sub(1)..=cmp::min(grid.len() - 1, y + 1) {
                for nx in x.saturating_sub(1)..=cmp::min(grid[y].len() - 1, x + 1) {
                    if ny == y && nx == x {
                        continue;
                    }

                    if grid[ny][nx] == b'@' {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                if REMOVE {
                    grid[y][x] = b'.';
                }

                total += 1;
            }
        }
    }

    total
}
