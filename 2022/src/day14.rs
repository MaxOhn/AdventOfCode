use std::ops::{Add, AddAssign, Index, IndexMut};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    // run_naive(input)
    run_dfs(input)
}

pub fn run_naive(input: &str) -> Result<Solution> {
    let p1 = fill_cave_naive(input, Part::One)?;
    let p2 = fill_cave_naive(input, Part::Two)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn run_dfs(input: &str) -> Result<Solution> {
    let p1 = fill_cave_dfs(input, Part::One)?;
    let p2 = fill_cave_dfs(input, Part::Two)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

fn fill_cave_dfs(input: &str, part: Part) -> Result<usize> {
    let mut cave = Cave::parse(input, part)?;

    let mut history = Vec::with_capacity(cave.height() as usize);
    history.push(SOURCE);

    let mut sand = 0;

    while let Some(prev) = history.last().copied() {
        let down = prev + DOWN;
        let left = prev + LEFT;
        let right = prev + RIGHT;

        match cave.get(down) {
            None => break,
            Some(false) => history.push(down),
            Some(true) => match cave.get(left) {
                None => break,
                Some(false) => history.push(left),
                Some(true) => match cave.get(right) {
                    None => break,
                    Some(false) => history.push(right),
                    Some(true) => {
                        let Some(pos) = history.pop() else { unreachable!() };
                        cave[pos] = true;
                        sand += 1;
                    }
                },
            },
        }
    }

    Ok(sand)
}

fn fill_cave_naive(input: &str, part: Part) -> Result<usize> {
    let mut cave = Cave::parse(input, part)?;
    let mut sand = 0;

    while cave.fill_one() {
        sand += 1;
    }

    Ok(sand)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn manhatten_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

const SOURCE: Pos = Pos { x: 500, y: 0 };

const DOWN: Pos = Pos { x: 0, y: 1 };
const LEFT: Pos = Pos { x: -1, y: 1 };
const RIGHT: Pos = Pos { x: 1, y: 1 };

struct Cave {
    inner: Box<[bool]>,
    w: i32,
    x_off: i32,
}

impl Cave {
    fn fill_one(&mut self) -> bool {
        if self[SOURCE] {
            return false;
        }

        let mut curr = SOURCE;

        loop {
            match self.get(curr + DOWN) {
                Some(false) => curr += DOWN,
                Some(true) => match self[curr + LEFT] {
                    false => curr += LEFT,
                    true => match self[curr + RIGHT] {
                        false => curr += RIGHT,
                        true => {
                            self[curr] = true;

                            return true;
                        }
                    },
                },
                None => return false,
            }
        }
    }

    fn parse(input: &str, part: Part) -> Result<Self> {
        struct Edge {
            pos: Pos,
            dist: i32,
        }

        impl Edge {
            fn new(dst_x: i32) -> Self {
                let dist = SOURCE.manhatten_dist(Pos { x: dst_x, y: 0 });

                Self { pos: SOURCE, dist }
            }
        }

        let mut paths = Vec::new();

        let mut min_x = i32::MAX;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        let mut left = Edge::new(0);

        for line in input.lines() {
            let mut path = Vec::new();

            for coord in line.split(" -> ") {
                let (x, y) = coord.split_once(',').wrap_err("invalid coord")?;

                let pos = Pos {
                    x: x.parse().wrap_err("invalid x")?,
                    y: y.parse().wrap_err("invalid y")?,
                };

                min_x = min_x.min(pos.x);
                max_x = max_x.max(pos.x);

                min_y = min_y.min(pos.y);
                max_y = max_y.max(pos.y);

                let left_dist = pos.manhatten_dist(Pos { x: 0, y: 0 });

                if left_dist < left.dist {
                    left.pos = pos;
                    left.dist = left_dist;
                }

                path.push(pos);
            }

            paths.push(path);
        }

        let mut w = max_x - min_x + 1;
        let mut h = max_y - min_y + 1;
        let mut x_off = min_x;

        if part == Part::Two {
            h += 2;

            let right =
                paths
                    .iter()
                    .flat_map(|path| path.iter())
                    .fold(Edge::new(max_x), |right, point| {
                        let right_dist = point.manhatten_dist(Pos { x: max_x, y: 0 });

                        if right_dist < right.dist {
                            Edge {
                                pos: *point,
                                dist: right_dist,
                            }
                        } else {
                            right
                        }
                    });

            let left_pad = h - left.pos.y - (left.pos.x - x_off + 1);
            let right_pad = h - right.pos.y - (w - (right.pos.x - x_off));

            w += left_pad + right_pad;
            x_off -= left_pad;
        }

        let mut inner = vec![false; (w * h) as usize].into_boxed_slice();

        for path in paths {
            for (start, end) in path.iter().zip(path.iter().skip(1)) {
                if start.x == end.x {
                    let min = start.y.min(end.y);
                    let max = start.y.max(end.y);
                    let x = start.x - x_off;

                    for y in min..=max {
                        let idx = y * w + x;
                        inner[idx as usize] = true;
                    }
                } else if start.y == end.y {
                    let min = (start.y * w + start.x.min(end.x) - x_off) as usize;
                    let max = (start.y * w + start.x.max(end.x) - x_off) as usize;

                    for square in get_mut!(inner[min..=max]) {
                        *square = true;
                    }
                } else {
                    bail!("cannot do diagonal line of rocks");
                }
            }
        }

        if part == Part::Two {
            for x in 0..w {
                let idx = (h - 1) * w + x;
                inner[idx as usize] = true;
            }
        }

        Ok(Self { inner, w, x_off })
    }

    fn height(&self) -> i32 {
        self.inner.len() as i32 / self.w
    }

    fn is_valid_pos(&self, pos: Pos) -> bool {
        let x = pos.x - self.x_off;

        x >= 0 && x < self.w && pos.y >= 0 && pos.y < self.height()
    }

    fn get(&self, pos: Pos) -> Option<bool> {
        self.is_valid_pos(pos).then(|| self[pos])
    }
}

impl Index<Pos> for Cave {
    type Output = bool;

    #[inline]
    fn index(&self, pos: Pos) -> &Self::Output {
        let idx = pos.y * self.w + pos.x - self.x_off;

        self.inner.index(idx as usize)
    }
}

impl IndexMut<Pos> for Cave {
    #[inline]
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let idx = pos.y * self.w + pos.x - self.x_off;

        self.inner.index_mut(idx as usize)
    }
}
