use std::{collections::VecDeque, hash::Hash};

use aoc_rust::Solution;
use eyre::{ContextCompat, Result};
use fxhash::FxHashSet as HashSet;

pub fn run(input: &str) -> Result<Solution> {
    let grid = Grid::new(input)?;
    let (x, y) = grid.start().wrap_err("missing start")?;

    if x != y || x != grid.size() / 2 {
        eyre::bail!("start must be in the middle of a square grid");
    }

    let p1 = part1(&grid, (x, y));
    let p2 = part2(&grid, (x, y));

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(grid: &Grid, (x, y): (i32, i32)) -> usize {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    struct State {
        x: i32,
        y: i32,
        remaining: i32,
    }

    let mut counts = HashSet::default();
    let mut cache = HashSet::default();

    let mut stack = vec![State {
        x,
        y,
        remaining: 64,
    }];

    while let Some(state) = stack.pop() {
        if state.remaining == 0 {
            counts.insert((state.x, state.y));
            continue;
        } else if state.remaining % 2 == 0 && !cache.insert(state) {
            continue;
        }

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nx = state.x + dx;
            let ny = state.y + dy;

            if !grid.contains(nx, ny) || grid.is_rock(nx, ny) {
                continue;
            }

            let nstate = State {
                x: nx,
                y: ny,
                remaining: state.remaining - 1,
            };

            stack.push(nstate);
        }
    }

    counts.len()
}

fn part2(grid: &Grid, start: (i32, i32)) -> u64 {
    const STEPS: i32 = 26_501_365;

    let size = grid.size();

    let mut even_dists = Vec::with_capacity((size * size) as usize / 2);
    let mut odd_dists = Vec::with_capacity((size * size) as usize / 2);

    let mut seen = HashSet::with_capacity_and_hasher((size * size) as usize, Default::default());
    seen.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((dist, (x, y))) = queue.pop_front() {
        if dist % 2 == 0 {
            even_dists.push(dist);
        } else {
            odd_dists.push(dist);
        }

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            if !grid.contains(nx, ny) || grid.is_rock(nx, ny) {
                continue;
            }

            if seen.insert((nx, ny)) {
                queue.push_back((dist + 1, (nx, ny)));
            }
        }
    }

    let half = size / 2;

    let even_outer = even_dists.iter().filter(|&&dist| dist > half).count() as u64 - 1;
    let odd_outer = odd_dists.iter().filter(|&&dist| dist > half).count() as u64;

    let even_all = even_dists.len() as u64;
    let odd_all = odd_dists.len() as u64;

    let n = ((STEPS - (size / 2)) / size) as u64;

    let even = n * n * even_all + n * even_outer;
    let odd = (n + 1) * (n + 1) * odd_all - (n + 1) * odd_outer;

    even + odd
}

pub struct Grid {
    size: i32,
    grid: Vec<u8>,
}

impl Grid {
    pub fn new(input: &str) -> eyre::Result<Self> {
        let w = input.lines().next().unwrap().len();
        let grid: Vec<_> = input.lines().flat_map(str::bytes).collect();

        eyre::ensure!(grid.len() / w == w, "grid must be square");

        Ok(Self {
            size: w as i32,
            grid,
        })
    }

    pub fn start(&self) -> Option<(i32, i32)> {
        self.grid
            .chunks_exact(self.size() as usize)
            .enumerate()
            .find_map(|(y, chunk)| {
                chunk
                    .iter()
                    .position(|&b| b == b'S')
                    .map(|x| (x as i32, y as i32))
            })
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.size() && y >= 0 && y < self.size()
    }

    pub fn is_rock(&self, x: i32, y: i32) -> bool {
        self.grid[(y * self.size + x) as usize] == b'#'
    }
}
