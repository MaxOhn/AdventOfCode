use aoc_rust::Solution;
use eyre::Result;

use self::grid::{Bufs, Direction, Grid};

pub fn run(input: &str) -> Result<Solution> {
    let grid = Grid::new(input.trim());

    let p1 = part1(&grid);
    let p2 = part2(&grid);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(grid: &Grid) -> usize {
    grid.run_beam(0, 0, Direction::Right, &mut Bufs::default())
}

fn part2(grid: &Grid) -> usize {
    let w = grid.width();
    let h = grid.height();

    let mut bufs = Bufs::default();
    let mut max = 0;

    for x in 0..w {
        max = max
            .max(grid.run_beam(x, 0, Direction::Down, &mut bufs))
            .max(grid.run_beam(x, h - 1, Direction::Up, &mut bufs));
    }

    for y in 0..h {
        max = max
            .max(grid.run_beam(0, y, Direction::Right, &mut bufs))
            .max(grid.run_beam(w - 1, y, Direction::Left, &mut bufs));
    }

    max
}

mod grid {
    use std::collections::VecDeque;

    use fxhash::FxHashSet as HashSet;

    pub struct Grid {
        width: usize,
        grid: Vec<u8>,
    }

    impl Grid {
        pub fn new(input: &str) -> Self {
            let width = input.lines().next().map_or(0, str::len);
            let grid = input.lines().flat_map(str::bytes).collect();

            Self { width, grid }
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.grid.len() / self.width
        }

        pub fn run_beam(&self, x: usize, y: usize, dir: Direction, bufs: &mut Bufs) -> usize {
            let w = self.width;
            let h = self.grid.len() / w;

            let beam = Beam::new(x, y, dir);

            let Bufs {
                beams,
                energized,
                count,
            } = bufs;

            energized.clear();

            energized.insert(beam);
            beams.push_back(beam);

            while let Some(Beam { x, y, dir }) = beams.pop_front() {
                let cell = self.grid[y * w + x];

                let [a, b] = match (cell, dir) {
                    (b'.' | b'-', Direction::Left) if x > 0 => {
                        [Some(Beam::new(x - 1, y, dir)), None]
                    }
                    (b'.' | b'-', Direction::Right) if x < w - 1 => {
                        [Some(Beam::new(x + 1, y, dir)), None]
                    }
                    (b'.' | b'|', Direction::Up) if y > 0 => [Some(Beam::new(x, y - 1, dir)), None],
                    (b'.' | b'|', Direction::Down) if y < h - 1 => {
                        [Some(Beam::new(x, y + 1, dir)), None]
                    }
                    (b'/', Direction::Left) if y < h - 1 => {
                        [Some(Beam::new(x, y + 1, Direction::Down)), None]
                    }
                    (b'/', Direction::Right) if y > 0 => {
                        [Some(Beam::new(x, y - 1, Direction::Up)), None]
                    }
                    (b'/', Direction::Up) if x < w - 1 => {
                        [Some(Beam::new(x + 1, y, Direction::Right)), None]
                    }
                    (b'/', Direction::Down) if x > 0 => {
                        [Some(Beam::new(x - 1, y, Direction::Left)), None]
                    }
                    (b'\\', Direction::Left) if y > 0 => {
                        [Some(Beam::new(x, y - 1, Direction::Up)), None]
                    }
                    (b'\\', Direction::Right) if y < h - 1 => {
                        [Some(Beam::new(x, y + 1, Direction::Down)), None]
                    }
                    (b'\\', Direction::Up) if x > 0 => {
                        [Some(Beam::new(x - 1, y, Direction::Left)), None]
                    }
                    (b'\\', Direction::Down) if x < w - 1 => {
                        [Some(Beam::new(x + 1, y, Direction::Right)), None]
                    }
                    (b'-', Direction::Up | Direction::Down) => {
                        let a = (x > 0).then(|| Beam::new(x - 1, y, Direction::Left));
                        let b = (x < w - 1).then(|| Beam::new(x + 1, y, Direction::Right));

                        [a, b]
                    }
                    (b'|', Direction::Left | Direction::Right) => {
                        let a = (y > 0).then(|| Beam::new(x, y - 1, Direction::Up));
                        let b = (y < h - 1).then(|| Beam::new(x, y + 1, Direction::Down));

                        [a, b]
                    }
                    _ => continue,
                };

                if let Some(beam) = a {
                    if energized.insert(beam) {
                        beams.push_back(beam);
                    }
                }

                if let Some(beam) = b {
                    if energized.insert(beam) {
                        beams.push_back(beam);
                    }
                }
            }

            count.clear();

            for beam in energized.iter() {
                count.insert((beam.x, beam.y));
            }

            count.len()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    struct Beam {
        x: usize,
        y: usize,
        dir: Direction,
    }

    impl Beam {
        fn new(x: usize, y: usize, dir: Direction) -> Self {
            Self { x, y, dir }
        }
    }

    #[derive(Default)]
    pub struct Bufs {
        beams: VecDeque<Beam>,
        energized: HashSet<Beam>,
        count: HashSet<(usize, usize)>,
    }
}
