use aoc_rust::Solution;
use eyre::Result;

use self::map::Map;

pub fn run(input: &str) -> Result<Solution> {
    let map: Map = input.trim().parse()?;

    let p1 = part1(&map)?;
    let p2 = part2(&map)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(map: &Map) -> Result<u32> {
    map.dijkstra(1, 3)
}

fn part2(map: &Map) -> Result<u32> {
    map.dijkstra(4, 10)
}

mod map {
    use std::{collections::BinaryHeap, str::FromStr};

    use eyre::{ContextCompat, Report};
    pub struct Map {
        width: usize,
        map: Vec<u8>,
    }

    impl Map {
        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.map.len() / self.width
        }

        pub fn dijkstra(&self, min_steps: i32, max_steps: i32) -> Result<u32, Report> {
            #[derive(Clone, Copy, PartialEq, Eq)]
            struct State {
                x: i32,
                y: i32,
                dir: Direction,
                heat_loss: u32,
            }

            impl Ord for State {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    other.heat_loss.cmp(&self.heat_loss)
                }
            }

            impl PartialOrd for State {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }

            let w = self.width();
            let h = self.height();

            let dst_x = w as i32 - 1;
            let dst_y = h as i32 - 1;

            let mut queue = BinaryHeap::new();

            queue.push(State {
                x: 0,
                y: 0,
                dir: Direction::Right,
                heat_loss: 0,
            });
            queue.push(State {
                x: 0,
                y: 0,
                dir: Direction::Down,
                heat_loss: 0,
            });

            let mut dist = [0_u8; 4].map(|_| vec![u32::MAX; self.map.len()]);

            while let Some(State {
                x,
                y,
                dir,
                heat_loss,
            }) = queue.pop()
            {
                if x == dst_x && y == dst_y {
                    return Ok(heat_loss);
                }

                let ndirs = match dir {
                    Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
                    Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
                };

                for ndir in ndirs {
                    let mut nheat_loss = heat_loss;

                    for steps in 1..=max_steps {
                        let (nx, ny) = match ndir {
                            Direction::Up => (x, y - steps),
                            Direction::Down => (x, y + steps),
                            Direction::Left => (x - steps, y),
                            Direction::Right => (x + steps, y),
                        };

                        if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                            break;
                        }

                        nheat_loss += self.map[ny as usize * w + nx as usize] as u32;

                        if steps < min_steps {
                            continue;
                        }

                        let curr_best = dist[ndir as usize][ny as usize * w + nx as usize];

                        if nheat_loss < curr_best {
                            dist[ndir as usize][ny as usize * w + nx as usize] = nheat_loss;

                            let nstate = State {
                                x: nx,
                                y: ny,
                                dir: ndir,
                                heat_loss: nheat_loss,
                            };

                            queue.push(nstate);
                        }
                    }
                }
            }

            eyre::bail!("no path found")
        }
    }

    impl FromStr for Map {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map_or(0, str::len);

            let map = s
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).map(|d| d as u8)))
                .collect::<Option<Vec<_>>>()
                .wrap_err("invalid digit")?;

            Ok(Self { width, map })
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
}
