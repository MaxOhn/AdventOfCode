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

const START_ID: u64 = 0;
const END_ID: u64 = 1;

fn part2(grid: &Grid) -> Result<u32> {
    let start = grid.start()?;
    let end = grid.end()?;

    let neighbor_map = neighbor_map(grid, start, end);

    eyre::ensure!(neighbor_map.len() <= 64, "can have at most 64 3-way forks");

    let mut seen = 0_u64;
    seen |= 1 << START_ID;
    let mut stack = vec![(START_ID, seen, 0)];
    let mut max = 0;

    while let Some((id, seen, len)) = stack.pop() {
        if id == END_ID {
            max = cmp::max(max, len);

            continue;
        }

        for &(nid, dist) in neighbor_map[&id].iter() {
            if seen & (1 << nid) == 0 {
                let mut nseen = seen;
                nseen |= 1 << nid;
                stack.push((nid, nseen, len + dist));
            }
        }
    }

    Ok(max as u32)
}

fn neighbor_map(grid: &Grid, start: Pos, end: Pos) -> HashMap<u64, Vec<(u64, usize)>> {
    let w = grid.width();
    let h = grid.height();

    // first collect all positions with >=3 neighbors

    let mut seen = HashSet::default();

    let mut forks = HashSet::default();
    forks.insert(start);
    forks.insert(end);

    let mut ids = HashMap::default();
    ids.insert(start, START_ID);
    ids.insert(end, END_ID);
    let mut next_id = END_ID + 1;

    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        if !seen.insert((x, y)) {
            continue;
        }

        let mut neighbors = 0;

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[(nx, ny)] == b'#' {
                continue;
            }

            neighbors += 1;
            stack.push((nx, ny));
        }

        if neighbors > 2 {
            forks.insert((x, y));

            if let Entry::Vacant(e) = ids.entry((x, y)) {
                e.insert(next_id);
                next_id += 1;
            }
        }
    }

    // then calculate the distances between the forks

    let mut neighbor_map = HashMap::<_, Vec<_>>::default();

    for &pos in forks.iter() {
        seen.clear();

        let mut stack = vec![(pos, 0)];

        while let Some(((x, y), dist)) = stack.pop() {
            if !seen.insert((x, y)) {
                continue;
            } else if (x, y) != pos && forks.contains(&(x, y)) {
                neighbor_map
                    .entry(ids[&pos])
                    .or_default()
                    .push((ids[&(x, y)], dist));
                continue;
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[(nx, ny)] == b'#' {
                    continue;
                }

                stack.push(((nx, ny), dist + 1));
            }
        }
    }

    neighbor_map
}

type Pos = (i32, i32);

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

    fn start(&self) -> Result<Pos> {
        self.grid
            .iter()
            .position(|&byte| byte == b'.')
            .map(|x| (x as i32, 0))
            .wrap_err("missing start in first row")
    }

    fn end(&self) -> Result<Pos> {
        self.grid
            .chunks_exact(self.width as usize)
            .last()
            .and_then(|row| row.iter().position(|&byte| byte == b'.'))
            .map(|x| (x as i32, self.height() - 1))
            .wrap_err("missing end in last row")
    }
}

impl Index<Pos> for Grid {
    type Output = u8;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        self.grid.index((y * self.width + x) as usize)
    }
}
