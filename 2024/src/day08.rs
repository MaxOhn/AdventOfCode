use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxBuildHasher;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);

    map.n_antinodes(|antinodes, x, y| {
        let diff = x - y;

        let nx = x + diff;
        let ny = y - diff;

        if map.contains(nx) {
            antinodes.insert(nx);
        }

        if map.contains(ny) {
            antinodes.insert(ny);
        }
    })
}

fn part2(input: &str) -> usize {
    let map = Map::parse(input);

    map.n_antinodes(|antinodes, mut x, mut y| {
        let diff = x - y;

        while map.contains(x) {
            antinodes.insert(x);
            x += diff;
        }

        while map.contains(y) {
            antinodes.insert(y);
            y -= diff;
        }
    })
}

type Antinodes = HashSet<Pos, FxBuildHasher>;

struct Map {
    w: i16,
    h: i16,
    antennas: HashMap<u8, Vec<Pos>, FxBuildHasher>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut antennas = HashMap::with_hasher(FxBuildHasher::default());

        let mut find_antennas = |y, line| {
            for (x, byte) in str::bytes(line).enumerate() {
                if byte != b'.' {
                    Vec::push(antennas.entry(byte).or_default(), Pos::new(x, y));
                }
            }
        };

        let mut lines = input.lines().enumerate();

        let (_, line) = lines.next().unwrap();
        find_antennas(0, line);

        let w = line.len() as i16;
        let mut h = 1;

        for (y, line) in lines {
            h += 1;
            find_antennas(y, line);
        }

        Self { w, h, antennas }
    }

    fn n_antinodes(&self, f: impl Fn(&mut Antinodes, Pos, Pos)) -> usize {
        let mut antinodes = HashSet::with_hasher(FxBuildHasher::default());

        for signals in self.antennas.values() {
            for i in 0..signals.len() {
                for j in i + 1..signals.len() {
                    f(&mut antinodes, signals[i], signals[j]);
                }
            }
        }

        antinodes.len()
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i16,
            y: y as i16,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
