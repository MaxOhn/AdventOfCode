use aoc_rust::Solution;
use eyre::Result;

use self::grid::{Bufs, Direction, Grid};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u64> {
    let grid: Grid = input.parse()?;

    Ok(grid.run_beam(0, 0, Direction::Right, &mut Bufs::default()))
}

fn part2(input: &str) -> Result<u64> {
    let grid: Grid = input.parse()?;

    Ok(grid.part2())
}

mod grid {
    use std::{cmp, collections::VecDeque, str::FromStr};

    use eyre::Report;
    use fxhash::FxHashSet as HashSet;

    #[derive(Default)]
    pub struct Bufs {
        beams: VecDeque<Beam>,
        energized: HashSet<Beam>,
        count: HashSet<(usize, usize)>,
    }

    pub struct Grid {
        width: usize,
        grid: Vec<Cell>,
    }

    impl Grid {
        pub fn part2(&self) -> u64 {
            let w = self.width;
            let h = self.grid.len() / w;

            let mut bufs = Bufs::default();

            let mut max = 0;

            for x in 0..w {
                max = cmp::max(max, self.run_beam(x, 0, Direction::Down, &mut bufs));
                max = cmp::max(max, self.run_beam(x, h - 1, Direction::Up, &mut bufs));
            }

            for y in 0..h {
                max = cmp::max(max, self.run_beam(0, y, Direction::Right, &mut bufs));
                max = cmp::max(max, self.run_beam(w - 1, y, Direction::Left, &mut bufs));
            }

            max
        }

        pub fn run_beam(&self, x: usize, y: usize, dir: Direction, bufs: &mut Bufs) -> u64 {
            let w = self.width;
            let h = self.grid.len() / w;

            let beam = Beam::new(x, y, dir);

            let Bufs {
                beams,
                energized,
                count,
            } = bufs;

            energized.clear();
            count.clear();

            energized.insert(beam);
            beams.push_back(beam);

            while let Some(Beam { x, y, dir }) = beams.pop_front() {
                let cell = self.grid[y * w + x];

                let [a, b] = match (cell.0, dir) {
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
                        let a = if x > 0 {
                            Some(Beam::new(x - 1, y, Direction::Left))
                        } else {
                            None
                        };

                        let b = if x < w - 1 {
                            Some(Beam::new(x + 1, y, Direction::Right))
                        } else {
                            None
                        };

                        [a, b]
                    }
                    (b'|', Direction::Left | Direction::Right) => {
                        let a = if y > 0 {
                            Some(Beam::new(x, y - 1, Direction::Up))
                        } else {
                            None
                        };

                        let b = if y < h - 1 {
                            Some(Beam::new(x, y + 1, Direction::Down))
                        } else {
                            None
                        };

                        [a, b]
                    }
                    _ => continue,
                };

                if let Some(a) = a {
                    if energized.insert(a) {
                        beams.push_back(a);
                    }
                }

                if let Some(b) = b {
                    if energized.insert(b) {
                        beams.push_back(b);
                    }
                }
            }

            for beam in energized.iter() {
                count.insert((beam.x, beam.y));
            }

            count.len() as u64
        }

        // fn print(&self, beams: &[Beam], energized: &HashSet<Beam>) {
        //     for (y, chunk) in self.grid.chunks_exact(self.width).enumerate() {
        //         for (x, cell) in chunk.iter().enumerate() {
        //             if energized.iter().any(|beam| (beam.x, beam.y) == (x, y)) {
        //                 print!("#");
        //                 continue;
        //             }

        //             let beam_count = beams
        //                 .iter()
        //                 .filter(|beam| beam.x == x && beam.y == y)
        //                 .count();

        //             if cell.0 == b'.' {
        //                 match beam_count {
        //                     0 => print!("."),
        //                     1 => {
        //                         let beam = beams
        //                             .iter()
        //                             .find(|beam| beam.x == x && beam.y == y)
        //                             .unwrap();

        //                         match beam.dir {
        //                             Direction::Left => print!("<"),
        //                             Direction::Right => print!(">"),
        //                             Direction::Up => print!("^"),
        //                             Direction::Down => print!("v"),
        //                         }
        //                     }
        //                     _ => print!("{beam_count}"),
        //                 }
        //             } else {
        //                 print!("{}", cell.0 as char);
        //             }
        //         }

        //         println!();
        //     }

        //     println!("---------------------------------------------------");
        // }
    }

    impl FromStr for Grid {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map_or(0, str::len);

            let grid = s
                .lines()
                .flat_map(|line| line.bytes().map(Cell::from))
                .collect();

            Ok(Self { width, grid })
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct Cell(u8);

    impl From<u8> for Cell {
        fn from(byte: u8) -> Self {
            Self(byte)
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
}
