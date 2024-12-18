use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_rust::{util::lines::Lines, Solution};
use eyre::Result;
use fxhash::FxBuildHasher;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_line(line: &str) -> (i16, i16) {
    let mut x = 0;
    let mut y = 0;
    let mut curr = &mut x;

    for byte in line.bytes() {
        if byte == b',' {
            curr = &mut y;
        } else {
            *curr *= 10;
            *curr += (byte & 0xF) as i16;
        }
    }

    (x, y)
}

const DIM: i16 = 70;
const TAKE: usize = 1024;

type Corrupted = HashSet<(i16, i16), FxBuildHasher>;

fn part1(input: &str) -> i16 {
    let corrupted: Corrupted = Lines::new(input).take(TAKE).map(parse_line).collect();
    let mut dijkstra = Dijkstra::default();

    dijkstra
        .run(&corrupted, true)
        .best()
        .expect("no path found")
}

fn part2(input: &str) -> Box<str> {
    let mut lines = Lines::new(input);

    let mut corrupted: Corrupted = lines.by_ref().take(TAKE).map(parse_line).collect();
    let mut dijkstra = Dijkstra::default();
    let mut path = HashSet::with_hasher(FxBuildHasher::default());

    if let Some(line) = lines.next() {
        corrupted.insert(parse_line(line));

        if dijkstra.run(&corrupted, false).no_path() {
            return Box::from(line);
        }

        dijkstra.collect_path(&mut path);

        for line in lines {
            let pos = parse_line(line);
            corrupted.insert(pos);

            if !path.contains(&pos) {
                continue;
            }

            dijkstra.reset();

            if dijkstra.run(&corrupted, false).no_path() {
                return Box::from(line);
            }

            dijkstra.collect_path(&mut path);
        }
    }

    panic!("end always reachable")
}

#[derive(Default)]
struct Dijkstra {
    heap: BinaryHeap<(i16, i16, i16)>,
    dists: HashMap<(i16, i16), i16, FxBuildHasher>,
    prevs: HashMap<(i16, i16), (i16, i16), FxBuildHasher>,
}

impl Dijkstra {
    fn reset(&mut self) {
        self.heap.clear();
        self.dists.clear();
        self.prevs.clear();
    }

    fn run(&mut self, corrupted: &Corrupted, keep_going: bool) -> DijkstraResult {
        let heap = &mut self.heap;
        let dists = &mut self.dists;
        let prevs = &mut self.prevs;

        heap.push((0, 0, 0));
        dists.insert((0, 0), 0);

        let mut best = i16::MAX;

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

                let dn = dists.entry((nx, ny)).or_insert(i16::MAX);

                if *dn > dist + 1 {
                    *dn = dist + 1;
                    prevs.insert((nx, ny), (x, y));
                    heap.push((nx + ny, nx, ny));
                }
            }
        }

        (best < i16::MAX)
            .then_some(DijkstraResult::Best(best))
            .unwrap_or(DijkstraResult::NoPath)
    }

    fn collect_path(&self, set: &mut HashSet<(i16, i16), FxBuildHasher>) {
        set.clear();
        let mut curr = (DIM, DIM);

        while curr != (0, 0) {
            set.insert(curr);
            curr = self.prevs[&curr];
        }
    }
}

enum DijkstraResult {
    Best(i16),
    NoPath,
    PathFound,
}

impl DijkstraResult {
    fn best(self) -> Option<i16> {
        match self {
            DijkstraResult::Best(best) => Some(best),
            _ => None,
        }
    }

    fn no_path(self) -> bool {
        matches!(self, DijkstraResult::NoPath)
    }
}
