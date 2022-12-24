use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Index, IndexMut},
};

use ahash::RandomState;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let (grid, start, end) = Grid::parse(input)?;
    let cycle = lcm(grid.width as u16 - 2, grid.height() as u16 - 2);
    let mut grids = vec![grid];
    let mut seen = HashSet::default();

    let p1 = dijkstra(start, end, &mut grids, cycle, 0, &mut seen);
    let back = dijkstra(end, start, &mut grids, cycle, p1, &mut seen);
    let p2 = dijkstra(start, end, &mut grids, cycle, back, &mut seen);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn dijkstra(
    start: Pos,
    end: Pos,
    grids: &mut Vec<Grid>,
    cycle: u16,
    offset: u16,
    seen: &mut HashSet<(Pos, u16), RandomState>,
) -> u16 {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct State {
        steps: u16,
        pos: Pos,
    }

    let mut heap = BinaryHeap::new();

    let state = State {
        pos: start,
        steps: offset,
    };

    let mut shortest = u16::MAX;
    heap.push((Reverse(start.dist(end)), state));
    seen.clear();

    while let Some((Reverse(dist), State { pos, steps })) = heap.pop() {
        if dist == 0 {
            shortest = shortest.min(steps);

            continue;
        } else if shortest < steps || !seen.insert((pos, steps % cycle)) {
            continue;
        }

        let grid_idx = (steps as usize + 1) % cycle as usize;

        let grid = match grids.get(grid_idx) {
            Some(grid) => grid,
            None => {
                let mut grid = grids[grids.len() - 1].clone();
                grid.simulate();
                grids.push(grid);

                &grids[grid_idx]
            }
        };

        const DELTAS: [Pos; 5] = [
            Pos { x: 0, y: -1 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 0 },
            Pos { x: -1, y: 0 },
            Pos { x: 0, y: 0 },
        ];

        for delta in DELTAS {
            let next = pos + delta;

            if !(grid.is_valid(next) && grid[next].is_empty()) {
                continue;
            }

            let state = State {
                pos: next,
                steps: steps + 1,
            };

            heap.push((Reverse(state.pos.dist(end)), state));
        }
    }

    shortest
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn dist(self, other: Self) -> u16 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    BlizzardUp,
    BlizzardDown,
    BlizzardRight,
    BlizzardLeft,
    Wall,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Space {
    tiles: [Tile; 4],
}

impl Default for Space {
    #[inline]
    fn default() -> Self {
        Self {
            tiles: [Tile::Empty; 4],
        }
    }
}

impl Space {
    fn is_empty(self) -> bool {
        self.tiles[0] == Tile::Empty
    }

    fn iter(self) -> impl Iterator<Item = Tile> {
        self.tiles
            .into_iter()
            .take_while(|&tile| tile != Tile::Empty)
    }

    fn push(&mut self, tile: Tile) {
        let Some(i) = self.tiles.iter().position(|&tile| tile == Tile::Empty) else {
            panic!("more than 5 tiles on a space")
        };

        self.tiles[i] = tile;
    }
}

#[derive(Clone)]
struct Grid {
    inner: Box<[Space]>,
    width: usize,
}

impl Grid {
    fn is_valid(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width as i16 && pos.y >= 0 && pos.y < self.height()
    }

    fn height(&self) -> i16 {
        (self.inner.len() / self.width) as i16
    }

    fn simulate(&mut self) {
        let mut next = vec![Space::default(); self.inner.len()].into_boxed_slice();

        for (from, space) in self.inner.iter().enumerate() {
            for tile in space.iter() {
                match tile {
                    Tile::BlizzardUp => {
                        let mut to = from - self.width;

                        if to < self.width {
                            to += self.inner.len() - 2 * self.width;
                        }

                        next[to].push(Tile::BlizzardUp);
                    }
                    Tile::BlizzardDown => {
                        let mut to = from + self.width;

                        if to >= self.inner.len() - self.width - 1 {
                            to %= self.width;
                            to += self.width;
                        }

                        next[to].push(Tile::BlizzardDown);
                    }
                    Tile::BlizzardRight => {
                        let mut to = from + 1;

                        if (to + 1) % self.width == 0 {
                            to -= self.width - 2;
                        }

                        next[to].push(Tile::BlizzardRight);
                    }
                    Tile::BlizzardLeft => {
                        let mut to = from - 1;

                        if to % self.width == 0 {
                            to += self.width - 2;
                        }

                        next[to].push(Tile::BlizzardLeft);
                    }
                    Tile::Empty => {}
                    Tile::Wall => next[from].push(Tile::Wall),
                }
            }
        }

        self.inner = next;
    }

    fn parse(input: &str) -> Result<(Self, Pos, Pos)> {
        let mut lines = input.lines().map(str::as_bytes);

        fn extend_from_line(line: &[u8], grid: &mut Vec<Space>) -> Result<()> {
            grid.reserve(line.len());

            for byte in line {
                let tile = match byte {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Empty,
                    b'>' => Tile::BlizzardRight,
                    b'<' => Tile::BlizzardLeft,
                    b'^' => Tile::BlizzardUp,
                    b'v' => Tile::BlizzardDown,
                    _ => bail!("unexpected tile `{}`", *byte as char),
                };

                let tiles = [tile, Tile::Empty, Tile::Empty, Tile::Empty];
                grid.push(Space { tiles });
            }

            Ok(())
        }

        let (mut grid, width) = if let Some(line) = lines.next() {
            let mut grid = Vec::new();
            extend_from_line(line, &mut grid)?;

            (grid, line.len())
        } else {
            (Vec::new(), 0)
        };

        let start = grid
            .iter()
            .position(|space| space.tiles[0] == Tile::Empty)
            .map(|x| Pos { x: x as i16, y: 0 })
            .wrap_err("missing empty tile in first line")?;

        for line in lines {
            extend_from_line(line, &mut grid)?;
        }

        let end = grid
            .iter()
            .rev()
            .take(width)
            .position(|space| space.tiles[0] == Tile::Empty)
            .map(|x| Pos {
                x: (width - x - 1) as i16,
                y: (grid.len() / width - 1) as i16,
            })
            .wrap_err("missing empty tile in last line")?;

        let grid = Self {
            inner: grid.into(),
            width,
        };

        Ok((grid, start, end))
    }
}

impl Index<Pos> for Grid {
    type Output = Space;

    #[inline]
    fn index(&self, Pos { x, y }: Pos) -> &Self::Output {
        self.inner.index((y as usize) * self.width + x as usize)
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, Pos { x, y }: Pos) -> &mut Self::Output {
        self.inner.index_mut((y as usize) * self.width + x as usize)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut rows = self.inner.chunks_exact(self.width);

        if let Some(row) = rows.next() {
            for space in row {
                space.fmt(f)?;
            }
        }

        for row in rows {
            f.write_str("\n")?;

            for space in row {
                space.fmt(f)?;
            }
        }

        Ok(())
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.tiles[1] == Tile::Empty {
            self.tiles[0].fmt(f)
        } else {
            self.iter().count().fmt(f)
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let tile = match self {
            Tile::BlizzardUp => "^",
            Tile::BlizzardDown => "v",
            Tile::BlizzardRight => ">",
            Tile::BlizzardLeft => "<",
            Tile::Wall => "#",
            Tile::Empty => ".",
        };

        f.write_str(tile)
    }
}

fn gcd(mut a: u16, mut b: u16) -> u16 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u16, b: u16) -> u16 {
    (a * b) / gcd(a, b)
}
