#![allow(clippy::derive_hash_xor_eq)]

use std::hash::{Hash, Hasher};

use aoc_rust::Solution;
use eyre::Result;
use hashbrown::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Pos3 {
    fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    fn neighbors(self) -> Pos3Neighbors {
        Pos3Neighbors::new(self)
    }
}

impl Hash for Pos3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = ((self.x as u32) << 16) + ((self.y as u32) << 8) + self.z as u32;

        n.hash(state);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pos4 {
    w: i8,
    x: i8,
    y: i8,
    z: i8,
}

impl Pos4 {
    fn new(w: i8, x: i8, y: i8, z: i8) -> Self {
        Self { w, x, y, z }
    }

    fn neighbors(self) -> Pos4Neighbors {
        Pos4Neighbors::new(self)
    }
}

impl Hash for Pos4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = ((self.w as u32) << 24)
            + ((self.x as u32) << 16)
            + ((self.y as u32) << 8)
            + self.z as u32;

        n.hash(state);
    }
}

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    let mut layers = HashSet::with_capacity(256);
    let mut y = 0;

    for line in input.lines() {
        let coords = line
            .trim_end()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| Pos3::new(x as i8, y, 0));

        layers.extend(coords);
        y += 1;
    }

    let mut next_layers = HashSet::with_capacity(256);
    let mut checked = HashSet::with_capacity(512);

    for _ in 0..6 {
        for coord in layers.iter().copied() {
            for candidate in coord.neighbors().filter(|&c| checked.insert(c)) {
                let mut n = -(layers.contains(&candidate) as i16);

                for neighbor in candidate.neighbors() {
                    n += layers.contains(&neighbor) as i16;
                }

                if n == 3 || (n == 2 && layers.contains(&candidate)) {
                    next_layers.insert(candidate);
                }
            }
        }

        std::mem::swap(&mut layers, &mut next_layers);
        next_layers.clear();
        checked.clear();
    }

    layers.len()
}

fn part2(input: &str) -> usize {
    let mut layers = HashSet::with_capacity(4096);
    let mut y = 0;

    for line in input.lines() {
        let coords = line
            .trim_end()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| Pos4::new(0, x as i8, y, 0));

        layers.extend(coords);
        y += 1;
    }

    let mut next_layers = HashSet::with_capacity(4096);
    let mut checked = HashSet::with_capacity(8192);

    for _ in 0..6 {
        for coord in layers.iter().copied() {
            for candidate in coord.neighbors().filter(|&c| checked.insert(c)) {
                let mut n = -(layers.contains(&candidate) as i16);

                for neighbor in candidate.neighbors() {
                    n += layers.contains(&neighbor) as i16;
                }

                if n == 3 || (n == 2 && layers.contains(&candidate)) {
                    next_layers.insert(candidate);
                }
            }
        }

        std::mem::swap(&mut layers, &mut next_layers);
        next_layers.clear();
        checked.clear();
    }

    layers.len()
}

pub struct Pos3Neighbors {
    base: Pos3,
    next: Option<Pos3>,
}

impl Pos3Neighbors {
    fn new(base: Pos3) -> Self {
        let mut next = base;
        next.x -= 1;
        next.y -= 1;
        next.z -= 1;

        Pos3Neighbors {
            base,
            next: Some(next),
        }
    }
}

impl Iterator for Pos3Neighbors {
    type Item = Pos3;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.next.take();

        if let Some(ref c) = to_return {
            let mut next = Pos3::new(c.x + 1, c.y, c.z);

            if next.x > self.base.x + 1 {
                next.x = self.base.x - 1;
                next.y += 1;
            }

            if next.y > self.base.y + 1 {
                next.y = self.base.y - 1;
                next.z += 1;
            }

            if next.z <= self.base.z + 1 {
                self.next.replace(next);
            }
        }

        to_return
    }
}

pub struct Pos4Neighbors {
    base: Pos4,
    next: Option<Pos4>,
}

impl Pos4Neighbors {
    fn new(base: Pos4) -> Self {
        let mut next = base;
        next.w -= 1;
        next.x -= 1;
        next.y -= 1;
        next.z -= 1;

        Pos4Neighbors {
            base,
            next: Some(next),
        }
    }
}

impl Iterator for Pos4Neighbors {
    type Item = Pos4;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.next.take();

        if let Some(ref c) = to_return {
            let mut next = Pos4::new(c.w + 1, c.x, c.y, c.z);

            if next.w > self.base.w + 1 {
                next.w = self.base.w - 1;
                next.x += 1;
            }

            if next.x > self.base.x + 1 {
                next.x = self.base.x - 1;
                next.y += 1;
            }

            if next.y > self.base.y + 1 {
                next.y = self.base.y - 1;
                next.z += 1;
            }

            if next.z <= self.base.z + 1 {
                self.next.replace(next);
            }
        }

        to_return
    }
}
