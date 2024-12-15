use std::{
    collections::{HashMap, HashSet},
    mem,
    ops::{Index, IndexMut},
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
    let (mut grid, movements) = Grid::new(input, false);
    let mut pos = grid.start();

    for movement in movements.bytes() {
        let delta = match movement {
            b'>' => 1,
            b'<' => -1,
            b'v' => grid.w,
            b'^' => -grid.w,
            b'\n' => continue,
            _ => unreachable!(),
        };

        grid.step(&mut pos, delta);
    }

    grid.gps_sum()
}

fn part2(input: &str) -> usize {
    let (mut grid, movements) = Grid::new(input, true);
    let mut pos = grid.start();
    let mut bufs = Buffers::default();

    for movement in movements.bytes() {
        match movement {
            b'>' => grid.step_horizontal(&mut pos, 1),
            b'<' => grid.step_horizontal(&mut pos, -1),
            b'v' => grid.step_vertical(&mut pos, grid.w, &mut bufs),
            b'^' => grid.step_vertical(&mut pos, -grid.w, &mut bufs),
            b'\n' => continue,
            _ => unreachable!(),
        }
    }

    grid.gps_sum()
}

struct Grid {
    bytes: Vec<u8>,
    w: i32,
}

impl Grid {
    fn new(map: &str, double_width: bool) -> (Self, &str) {
        fn extend(dst: &mut Vec<u8>, src: &[u8], double_width: bool) {
            if double_width {
                dst.extend(src.iter().flat_map(|byte| match byte {
                    b'#' => [b'#', b'#'],
                    b'.' => [b'.', b'.'],
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                }));
            } else {
                dst.extend_from_slice(src);
            }
        }

        let mut bytes = Vec::new();
        let map_bytes = map.as_bytes();
        let mut iter = memchr::memchr_iter(b'\n', map_bytes);

        let mut w = iter.next().unwrap_or(map.len());
        extend(&mut bytes, &map_bytes[..w], double_width);
        let mut start = w + 1;

        if double_width {
            w *= 2;
        }

        for i in iter {
            if start == i {
                return (Self { bytes, w: w as i32 }, &map[i + 1..]);
            }

            extend(&mut bytes, &map_bytes[start..i], double_width);
            start = i + 1;
        }

        panic!("missing double newline")
    }

    fn start(&self) -> i32 {
        memchr::memchr(b'@', &self.bytes).expect("missing @") as i32
    }

    fn gps_sum(&self) -> usize {
        memchr::memchr2_iter(b'O', b'[', &self.bytes)
            .map(|i| {
                let x = i % self.w as usize;
                let y = i / self.w as usize;

                x + 100 * y
            })
            .sum()
    }

    fn step(&mut self, pos: &mut i32, delta: i32) {
        let mut next = *pos + delta;

        loop {
            match self[next] {
                b'#' => return,
                b'.' => break,
                _ => next += delta,
            }
        }

        while next != *pos {
            self[next] = self[next - delta];
            next -= delta;
        }

        self[*pos] = b'.';
        *pos += delta;
    }

    fn step_horizontal(&mut self, pos: &mut i32, delta: i32) {
        self.step(pos, delta);
    }

    fn step_vertical(&mut self, pos: &mut i32, delta: i32, bufs: &mut Buffers) {
        let Buffers {
            to_move,
            curr_set,
            next_set,
        } = bufs;

        to_move.clear();
        curr_set.clear();
        next_set.clear();

        curr_set.insert(*pos);

        while !curr_set.is_empty() {
            for curr in curr_set.drain() {
                to_move.insert(curr, self[curr]);
                let next = curr + delta;

                match self[next] {
                    b'#' => return,
                    b'[' => {
                        next_set.insert(next);
                        next_set.insert(next + 1);
                    }
                    b']' => {
                        next_set.insert(next - 1);
                        next_set.insert(next);
                    }
                    b'.' => {}
                    _ => unreachable!(),
                }
            }

            mem::swap(next_set, curr_set);
        }

        for (&pos, &byte) in to_move.iter() {
            self[pos + delta] = byte;

            if !to_move.contains_key(&(pos - delta)) {
                self[pos] = b'.';
            }
        }

        *pos += delta;
    }

    #[allow(unused)]
    fn print(&self) {
        for line in self.bytes.chunks_exact(self.w as usize) {
            println!("{}", std::str::from_utf8(line).unwrap());
        }
    }
}

impl Index<i32> for Grid {
    type Output = u8;

    fn index(&self, idx: i32) -> &Self::Output {
        self.bytes.index(idx as usize)
    }
}

impl IndexMut<i32> for Grid {
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        self.bytes.index_mut(idx as usize)
    }
}

#[derive(Default)]
struct Buffers {
    to_move: HashMap<i32, u8, FxBuildHasher>,
    curr_set: HashSet<i32, FxBuildHasher>,
    next_set: HashSet<i32, FxBuildHasher>,
}
