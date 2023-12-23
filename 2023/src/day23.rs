use std::{cmp, collections::hash_map::Entry, ops::Index};

use aoc_rust::Solution;
use eyre::{ContextCompat, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> Result<Solution> {
    let grid = Grid::new(input.trim());

    let p1 = part1(&grid)?;
    let p2 = part2(&grid)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(grid: &Grid) -> Result<u32> {
    let w = grid.width();
    let h = grid.height();

    let (x, y) = grid.start()?;
    let (fx, fy) = grid.end()?;

    let mut stack = vec![(x, y, HashSet::default())];
    let mut max = 0;

    while let Some((x, y, seen)) = stack.pop() {
        if (x, y) == (fx, fy) {
            if seen.len() > max {
                max = seen.len();
            }

            continue;
        }

        let neighbors = match grid[(x, y)] {
            b'.' => &[(1, 0), (-1, 0), (0, 1), (0, -1)] as &[_],
            b'v' => &[(0, 1)],
            b'>' => &[(1, 0)],
            b'<' => &[(-1, 0)],
            b'^' => &[(0, -1)],
            byte => eyre::bail!("invalid byte `{byte}`"),
        };

        for (dx, dy) in neighbors {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[(nx, ny)] == b'#' {
                continue;
            }

            if !seen.contains(&(nx, ny)) {
                let mut nseen = seen.clone();
                nseen.insert((nx, ny));
                stack.push((nx, ny, nseen));
            }
        }
    }

    Ok(max as u32)
}

fn part2(grid: &Grid) -> Result<u32> {
    let neighbor_map = neighbor_map(grid);

    let (x, y) = grid.start()?;
    let (fx, fy) = grid.end()?;

    let mut seen = HashSet::default();
    seen.insert((x, y));
    let mut stack = vec![(x, y, seen, 0)];
    let mut max = 0;

    while let Some((x, y, seen, len)) = stack.pop() {
        if (x, y) == (fx, fy) {
            max = cmp::max(max, len);

            continue;
        }

        for &((nx, ny), dist) in neighbor_map[&(x, y)].iter() {
            if !seen.contains(&(nx, ny)) {
                let mut nseen = seen.clone();
                nseen.insert((nx, ny));
                stack.push((nx, ny, nseen, len + dist));
            }
        }
    }

    Ok(max as u32)
}

fn neighbor_map(grid: &Grid) -> HashMap<(i32, i32), Vec<((i32, i32), usize)>> {
    let w = grid.width();
    let h = grid.height();
    let mut direct_neighbors = HashMap::default();

    for x in 0..w {
        for y in 0..h {
            if grid[(x, y)] == b'#' {
                continue;
            }

            let mut neighbors = Vec::new();

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[(nx, ny)] == b'#' {
                    continue;
                }

                neighbors.push((nx, ny));
            }

            direct_neighbors.insert((x, y), neighbors);
        }
    }

    let mut neighbor_map = HashMap::default();

    for x in 0..w {
        for y in 0..h {
            let Some(ns) = direct_neighbors.get(&(x, y)) else {
                continue;
            };

            let mut final_neighbors: HashMap<_, _> = ns.iter().map(|n| (*n, 1)).collect();
            final_neighbors.insert((x, y), 0);
            let mut double_neighbors = HashSet::default();
            double_neighbors.insert((x, y));

            let mut stack = vec![(ns, 1)];

            while let Some((neighbors, dist)) = stack.pop() {
                for neighbor in neighbors {
                    let Some(nns) = direct_neighbors.get(neighbor) else {
                        continue;
                    };

                    if nns.len() != 2 {
                        continue;
                    }

                    let mut seen = true;

                    for &nn in nns {
                        match final_neighbors.entry(nn) {
                            Entry::Occupied(_) => {
                                double_neighbors.insert(nn);
                            }
                            Entry::Vacant(e) => {
                                e.insert(dist + 1);
                                seen = false;
                            }
                        }
                    }

                    if !seen {
                        stack.push((nns, dist + 1));
                    }
                }
            }

            let neighbors = final_neighbors
                .into_iter()
                .filter(|(key, _)| !double_neighbors.contains(key))
                .collect();
            neighbor_map.insert((x, y), neighbors);
        }
    }

    neighbor_map
}

struct Grid {
    width: i32,
    grid: Vec<u8>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().next().map_or(0, str::len) as i32;
        let grid = input.lines().flat_map(str::bytes).collect();

        Self { width, grid }
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32 / self.width
    }

    fn start(&self) -> Result<(i32, i32)> {
        self.grid
            .iter()
            .position(|&byte| byte == b'.')
            .map(|x| (x as i32, 0))
            .wrap_err("missing start in first row")
    }

    fn end(&self) -> Result<(i32, i32)> {
        self.grid
            .chunks_exact(self.width as usize)
            .last()
            .and_then(|row| row.iter().position(|&byte| byte == b'.'))
            .map(|x| (x as i32, self.height() - 1))
            .wrap_err("missing end in last row")
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = u8;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        self.grid.index((y * self.width + x) as usize)
    }
}
