use std::{ops::Index, str::FromStr};

use aoc_rust::Solution;
use eyre::{Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let image = input.trim().parse::<Image>()?;
    let galaxies = image.galaxies();

    let p1 = part1(&image, galaxies.clone());
    let p2 = part2(&image, galaxies);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(image: &Image, mut galaxies: Vec<Pos>) -> usize {
    solve(image, &mut galaxies, 1)
}

fn part2(image: &Image, mut galaxies: Vec<Pos>) -> usize {
    solve(image, &mut galaxies, 999_999)
}

fn solve(image: &Image, galaxies: &mut [Pos], expand: usize) -> usize {
    let mut h = image.height();
    let mut w = image.width();

    let mut x = 0;

    while x < w {
        let is_empty = galaxies.iter().all(|(gx, _)| *gx != x);

        if is_empty {
            for (gx, _) in galaxies.iter_mut() {
                if *gx > x {
                    *gx += expand;
                }
            }

            w += expand;
            x += expand;
        }

        x += 1;
    }

    let mut y = 0;

    while y < h {
        let is_empty = galaxies.iter().all(|(_, gy)| *gy != y);

        if is_empty {
            for (_, gy) in galaxies.iter_mut() {
                if *gy > y {
                    *gy += expand;
                }
            }

            h += expand;
            y += expand;
        }

        y += 1;
    }

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, (x, y))| {
            galaxies
                .iter()
                .skip(i)
                .map(|(u, v)| x.abs_diff(*u) + y.abs_diff(*v))
        })
        .sum()
}

type Pos = (usize, usize);

struct Image {
    width: usize,
    inner: Vec<Pixel>,
}

impl Image {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.inner
            .chunks_exact(self.width())
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, px)| matches!(px, Pixel::Galaxy).then_some((x, y)))
            })
            .collect()
    }
}

impl FromStr for Image {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map_or(0, str::len);

        let inner = s
            .lines()
            .flat_map(|line| line.bytes().map(Pixel::try_from))
            .collect::<Result<_>>()?;

        Ok(Self { width, inner })
    }
}

impl Index<Pos> for Image {
    type Output = Pixel;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        let idx = y * self.width + x;

        &self.inner[idx]
    }
}

#[derive(Copy, Clone)]
enum Pixel {
    Empty,
    Galaxy,
}

impl TryFrom<u8> for Pixel {
    type Error = Report;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'.' => Ok(Self::Empty),
            b'#' => Ok(Self::Galaxy),
            _ => eyre::bail!("invalid pixel byte `{byte}`"),
        }
    }
}
