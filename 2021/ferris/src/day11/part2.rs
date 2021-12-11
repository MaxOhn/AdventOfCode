#![feature(core_intrinsics)]

use std::intrinsics::unlikely;

pub fn run(input: &[u8]) -> i64 {
    let mut grid = [[0; 10]; 10];

    for y in 0..10 {
        for x in 0..10 {
            unsafe {
                *grid.get_unchecked_mut(y).get_unchecked_mut(x) =
                    *input.get_unchecked(y * 11 + x) & 0x0F;
            }
        }
    }

    let mut i = 1;

    loop {
        if unlikely(step(&mut grid) == 100) {
            return i;
        }

        i += 1;
    }
}

#[inline(always)]
fn step(grid: &mut [[u8; 10]; 10]) -> i64 {
    let mut flashes = 0;

    for y in 0..10 {
        for x in 0..10 {
            let value = &mut grid[y][x];
            *value += 1;

            if *value == 10 {
                flashes += 1;
                increment_neighbors(grid, x as isize, y as isize, &mut flashes);
            }
        }
    }

    if flashes > 0 {
        grid.iter_mut()
            .map(|row| row.iter_mut())
            .flatten()
            .filter(|v| **v >= 10)
            .for_each(|v| *v = 0);
    }

    flashes
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn increment_neighbors(grid: &mut [[u8; 10]; 10], x: isize, y: isize, flashes: &mut i64) {
    for &(i, j) in &OFFSETS {
        let cx = x + i;
        let cy = y + j;

        if cx < 0 || cy < 0 || cx == 10 || cy == 10 {
            continue;
        }

        let value = &mut grid[cy as usize][cx as usize];
        *value += 1;

        if *value == 10 {
            *flashes += 1;
            increment_neighbors(grid, cx, cy, flashes);
        }
    }
}
