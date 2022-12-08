use std::slice::ChunksExact;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let trees = Matrix::parse(input);

    let p1 = part1(&trees);
    let p2 = part2(&trees);

    Ok(Solution::new().part1(p1).part2(p2))
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn part1(trees: &Matrix) -> u16 {
    trees.iter().fold(0, |total_visible, (x, y, tree)| {
        let curr_visible = DIRECTIONS.iter().any(|&(dx, dy)| {
            let mut cx = x + dx;
            let mut cy = y + dy;

            loop {
                match trees.get(cx, cy) {
                    Some(h) if h >= tree => return false,
                    Some(_) => {
                        cx += dx;
                        cy += dy;
                    }
                    None => return true,
                }
            }
        });

        total_visible + curr_visible as u16
    })
}

fn part2(trees: &Matrix) -> u32 {
    trees.iter().fold(0, |max, (x, y, tree)| {
        let score = DIRECTIONS
            .iter()
            .map(|&(dx, dy)| {
                let mut cx = x + dx;
                let mut cy = y + dy;
                let mut score = 0;

                loop {
                    match trees.get(cx, cy) {
                        Some(h) if h >= tree => return score + 1,
                        Some(_) => {
                            cx += dx;
                            cy += dy;
                            score += 1;
                        }
                        None => return score,
                    }
                }
            })
            .product();

        max.max(score)
    })
}

pub struct Matrix {
    inner: Box<[u8]>,
    width: usize,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let mut inner = if let Some(line) = lines.next() {
            line.bytes().map(|byte| byte - b'0').collect()
        } else {
            Vec::new()
        };

        let width = inner.len();
        inner.reserve(width * width);

        for line in lines {
            inner.extend(line.bytes().map(|byte| byte - b'0'));
        }

        Self {
            inner: inner.into(),
            width,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    fn iter(&self) -> Iter<'_> {
        let mut chunks = self.inner.chunks_exact(self.width);
        let chunk = chunks.next();

        Iter {
            x: 0,
            y: 0,
            chunks,
            chunk,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        (x >= 0 && y >= 0 && y < self.height() as isize && x < self.width() as isize)
            .then(|| self.get_unchecked(x, y))
    }

    fn get_unchecked(&self, x: isize, y: isize) -> u8 {
        get!(self.inner[y as usize * self.width + x as usize])
    }
}

struct Iter<'m> {
    x: isize,
    y: isize,
    chunks: ChunksExact<'m, u8>,
    chunk: Option<&'m [u8]>,
}

impl Iterator for Iter<'_> {
    type Item = (isize, isize, u8);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chunk?.split_first() {
                Some((elem, rest)) => {
                    self.chunk = Some(rest);
                    let x = self.x;
                    self.x += 1;

                    return Some((x, self.y, *elem));
                }
                None => {
                    self.chunk = self.chunks.next();
                    self.y += 1;
                    self.x = 0;
                }
            }
        }
    }
}
