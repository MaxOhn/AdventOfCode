use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

fn parse_line(line: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut curr = &mut x;

    for byte in line.bytes() {
        if byte == b',' {
            curr = &mut y;
        } else {
            *curr *= 10;
            *curr += (byte & 0xF) as i32;
        }
    }

    (x, y)
}

const DIM: i32 = 70;
const TAKE: usize = 1024;

type Corrupted = HashSet<(i32, i32), FxBuildHasher>;

fn part1(input: &str) -> i32 {
    let corrupted: Corrupted = input.lines().take(TAKE).map(parse_line).collect();
    let mut dijkstra = Dijkstra::default();

    dijkstra
        .run(&corrupted, true)
        .best()
        .expect("no path found")
}

fn part2(input: &str) -> Box<str> {
    let mut lines = input.lines();

    let mut corrupted: Corrupted = lines.by_ref().take(TAKE).map(parse_line).collect();
    let mut dijkstra = Dijkstra::default();

    for line in lines {
        corrupted.insert(parse_line(line));
        dijkstra.reset();

        if dijkstra.run(&corrupted, false).no_path() {
            return Box::from(line);
        }
    }

    panic!("end always reachable")
}

#[derive(Default)]
struct Dijkstra {
    heap: BinaryHeap<(Reverse<i32>, i32, i32)>,
    dists: HashMap<(i32, i32), i32, FxBuildHasher>,
}

impl Dijkstra {
    fn reset(&mut self) {
        self.heap.clear();
        self.dists.clear();
    }

    fn run(&mut self, corrupted: &Corrupted, keep_going: bool) -> DijkstraResult {
        let heap = &mut self.heap;
        let dists = &mut self.dists;

        heap.push((Reverse(2 * DIM), 0, 0));
        dists.insert((0, 0), 0);

        let mut best = i32::MAX;

        while let Some((_, x, y)) = heap.pop() {
            let dist = dists[&(x, y)];

            if dist > best {
                continue;
            } else if (x, y) == (DIM, DIM) {
                if !keep_going {
                    return DijkstraResult::PathFound;
                }

                if best > dist {
                    best = dist;
                }

                continue;
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;

                if !(0..=DIM).contains(&nx)
                    || !(0..=DIM).contains(&ny)
                    || corrupted.contains(&(nx, ny))
                {
                    continue;
                }

                let dn = dists.entry((nx, ny)).or_insert(i32::MAX);

                if *dn > dist + 1 {
                    *dn = dist + 1;

                    heap.push((Reverse(2 * DIM - nx - ny), nx, ny));
                }
            }
        }

        (best < i32::MAX)
            .then_some(DijkstraResult::Best(best))
            .unwrap_or(DijkstraResult::NoPath)
    }
}

enum DijkstraResult {
    Best(i32),
    NoPath,
    PathFound,
}

impl DijkstraResult {
    fn best(self) -> Option<i32> {
        match self {
            DijkstraResult::Best(best) => Some(best),
            _ => None,
        }
    }

    fn no_path(self) -> bool {
        matches!(self, DijkstraResult::NoPath)
    }
}
