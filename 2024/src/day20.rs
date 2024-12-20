use std::{cmp::Reverse, collections::BinaryHeap, ops::Index};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxHashMap;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    solve::<2, 100>(input)
}

fn part2(input: &str) -> u32 {
    solve::<20, 100>(input)
}

fn solve<const MAX_CHEAT: u16, const AT_LEAST: u16>(input: &str) -> u32 {
    let track = RaceTrack::new(input);
    let start = track.find(b'S');
    let end = track.find(b'E');

    let end_dists = track.trace(end, start);
    let fair_dist = end_dists[&start];

    let mut curr = start;
    let mut dist = fair_dist;
    let mut path = Vec::with_capacity(fair_dist as usize + 1);

    while curr != end {
        let (next, next_dist) = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let next = (curr.0 as i16 + dx, curr.1 as i16 + dy);
                let next_dist = *end_dists.get(&next)?;

                Some((next, next_dist))
            })
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        path.push((curr, dist));
        curr = next;
        dist = next_dist;
    }

    path.push((end, 0));

    path.par_iter()
        .enumerate()
        .map(|(i, &(a, dist_a_end))| {
            let mut count = 0;

            for &(b, dist_b_end) in path.iter().skip(i + 2 + AT_LEAST as usize) {
                let dist_a_b = manhatten_dist(a, b);

                if dist_a_b > MAX_CHEAT {
                    continue;
                }

                let saved = dist_a_end - (dist_a_b + dist_b_end);

                if saved < AT_LEAST {
                    continue;
                }

                count += 1;
            }

            count
        })
        .sum()
}

fn manhatten_dist((ax, ay): (i16, i16), (bx, by): (i16, i16)) -> u16 {
    ax.abs_diff(bx) + ay.abs_diff(by)
}

struct RaceTrack<'a> {
    bytes: &'a [u8],
    w: i16,
}

impl<'a> RaceTrack<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', input.as_bytes()).unwrap_or(bytes.len()) as i16 + 1;

        Self { bytes, w }
    }

    fn find(&self, byte: u8) -> (i16, i16) {
        let pos = memchr::memchr(byte, &self.bytes).unwrap() as i16;

        (pos % self.w, pos / self.w)
    }

    fn trace(&self, start: (i16, i16), end: (i16, i16)) -> FxHashMap<(i16, i16), u16> {
        let mut heap = BinaryHeap::new();
        let mut dists = FxHashMap::default();

        heap.push((Reverse(manhatten_dist(start, end)), start));
        dists.insert(start, 0);

        while let Some((_, (x, y))) = heap.pop() {
            let dist = dists[&(x, y)];

            if (x, y) == end {
                return dists;
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (x + dx, y + dy);

                if self[next] == b'#' {
                    continue;
                }

                let entry = dists.entry(next).or_insert(u16::MAX);

                if *entry > dist + 1 {
                    *entry = dist + 1;
                    heap.push((Reverse(manhatten_dist(next, end)), next));
                }
            }
        }

        panic!("no path found")
    }
}

impl Index<(i16, i16)> for RaceTrack<'_> {
    type Output = u8;

    fn index(&self, (x, y): (i16, i16)) -> &Self::Output {
        self.bytes.index((y * self.w + x) as usize)
    }
}
