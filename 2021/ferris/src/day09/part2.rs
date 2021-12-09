use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use memchr::memchr;

pub fn run(input: &[u8]) -> i64 {
    let len = input.len();
    let mut input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut grid = Vec::with_capacity(100 * 100);

    while let Some(end) = memchr(b'\n', input) {
        grid.extend(input[..end].iter().map(|&b| b & 0x0F));
        input = &input[end + 1..];
    }

    let w = input.len();
    grid.extend(input.iter().map(|&b| b & 0x0F));
    let h = grid.len() / w;
    let mut lows = Vec::with_capacity(64);

    // First row - first column
    let curr = grid[0];
    if !((grid[1] <= curr) || grid[w] <= curr) {
        lows.push((0, 0));
    }

    // First row - last column
    let curr = grid[w - 1];
    if !((grid[w - 2] <= curr) || grid[2 * w - 1] <= curr) {
        lows.push((w - 1, 0));
    }

    // First row
    for x in 1..w - 1 {
        let curr = grid[x];

        if !((grid[x + 1] <= curr) || (grid[x - 1] <= curr) || grid[w + x] <= curr) {
            lows.push((x, 0));
        }
    }

    // Last row - first column
    let idx = (h - 1) * w;
    let curr = grid[idx];
    if !((grid[idx + 1] <= curr) || grid[idx - w] <= curr) {
        lows.push((0, h - 1));
    }

    // Last row - last column
    let curr = *grid.last().unwrap();
    if !((grid[w * h - 2] <= curr) || grid[idx - 1] <= curr) {
        lows.push((w - 1, h - 1));
    }

    // Last row
    for x in 1..w - 1 {
        let curr = grid[idx + x];

        if !((grid[idx + x + 1] <= curr)
            || (grid[idx + x - 1] <= curr)
            || grid[(w * (h - 2)) + x] <= curr)
        {
            lows.push((x, h - 1));
        }
    }

    // First column
    for y in (w..w * (h - 1)).step_by(w) {
        let curr = grid[y];

        if !((grid[y - w] <= curr) || (grid[y + w] <= curr) || (grid[y + 1] <= curr)) {
            lows.push((0, y / w));
        }
    }

    // Last column
    for y in (2 * w - 1..w * (h - 1) - 1).step_by(w) {
        let curr = grid[y];

        if !((grid[y - w] <= curr) || (grid[y + w] <= curr) || (grid[y - 1] <= curr)) {
            lows.push((w - 1, y / w));
        }
    }

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let curr = grid[(y * w + x)];

            if !(grid[(y * w + (x - 1))] <= curr
                || grid[(y * w + (x + 1))] <= curr
                || grid[((y - 1) * w + x)] <= curr
                || grid[((y + 1) * w + x)] <= curr)
            {
                lows.push((x, y));
            }
        }
    }

    let mut basins = Vec::with_capacity(lows.len());

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for low in lows {
        let mut basin = HashSet::new();

        visited.clear();
        queue.clear();
        queue.push_front(low);

        while let Some((cx, cy)) = queue.pop_back() {
            if grid[cy * w + cx] == 9 {
                continue;
            }

            basin.insert((cx, cy));

            if let Some(nx) = cx.checked_sub(1).filter(|&nx| visited.insert((nx, cy))) {
                queue.push_front((nx, cy));
            }

            if let Some(ny) = cy.checked_sub(1).filter(|&ny| visited.insert((cx, ny))) {
                queue.push_front((cx, ny));
            }

            let nx = cx + 1;

            if nx < w && visited.insert((nx, cy)) {
                queue.push_front((nx, cy));
            }

            let ny = cy + 1;

            if ny < h && visited.insert((cx, ny)) {
                queue.push_front((cx, ny));
            }
        }

        basins.push(basin.len());
    }

    basins.sort_unstable_by_key(|&len| Reverse(len));
    basins.truncate(3);

    basins.iter().fold(1, |prod, &len| prod * len) as i64
}
