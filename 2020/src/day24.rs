use std::collections::HashSet;
use std::hint::unreachable_unchecked;
use std::ops::{Add, AddAssign};

use aoc_rust::Solution;
use eyre::Result;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn neighbors<'s>(&'s self) -> impl Iterator<Item = Pos> + 's {
        OFFSETS.iter().copied().map(move |offset| *self + offset)
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<(i32, i32, i32)> for Pos {
    fn add_assign(&mut self, (x, y, z): (i32, i32, i32)) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

static OFFSETS: [Pos; 6] = [
    Pos { x: -1, y: -1, z: 0 },
    Pos { x: 0, y: -1, z: -1 },
    Pos { x: 1, y: 0, z: -1 },
    Pos { x: 1, y: 1, z: 0 },
    Pos { x: 0, y: 1, z: 1 },
    Pos { x: -1, y: 0, z: 1 },
];

pub fn run(input: &str) -> Result<Solution> {
    let mut blacks = HashSet::with_capacity(2048);

    for line in input.lines() {
        let mut pos = Pos::default();
        let bytes = line.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match get!(bytes, i) {
                b'e' => pos += (-1, -1, 0),
                b'w' => pos += (1, 1, 0),
                b'n' => {
                    i += 1;
                    match get!(bytes, i) {
                        b'e' => pos += (-1, 0, 1),
                        b'w' => pos += (0, 1, 1),
                        _ => unsafe { unreachable_unchecked() },
                    }
                }
                b's' => {
                    i += 1;
                    match get!(bytes, i) {
                        b'e' => pos += (0, -1, -1),
                        b'w' => pos += (1, 0, -1),
                        _ => unsafe { unreachable_unchecked() },
                    }
                }
                b'\n' => break,
                _ => unsafe { unreachable_unchecked() },
            }

            i += 1;
        }

        if !blacks.insert(pos) {
            blacks.remove(&pos);
        }
    }

    let p1 = blacks.len();

    let mut next_blacks = HashSet::with_capacity(blacks.len());
    let mut seen = HashSet::with_capacity(2048);

    for _ in 0..100 {
        for init in blacks.iter() {
            for curr in init.neighbors() {
                if !seen.insert(curr) {
                    continue;
                }

                let black = blacks.contains(&curr);
                let black_neighbors = curr.neighbors().filter(|n| blacks.contains(n)).count();

                if black_neighbors == 2 || (black_neighbors == 1 && black) {
                    next_blacks.insert(curr);
                }
            }
        }

        seen.clear();
        blacks.clear();
        std::mem::swap(&mut blacks, &mut next_blacks);
    }

    let p2 = blacks.len();

    Ok(Solution::new().part1(p1).part2(p2))
}
