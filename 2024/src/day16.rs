use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
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

fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let w = memchr::memchr(b'\n', bytes).unwrap() as i32 + 1;
    let start = memchr::memchr(b'S', bytes).unwrap() as i32;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start, 1));

    let mut dists = vec![[u32::MAX; 4]; bytes.len()];
    dists[start as usize][1] = 0;

    let dirs = directions(w);

    while let Some((Reverse(score), pos, dir)) = heap.pop() {
        if bytes[pos as usize] == b'E' {
            return score;
        }

        for (i, delta) in dirs.into_iter().enumerate() {
            let next = pos + delta;

            if bytes[next as usize] == b'#' {
                continue;
            }

            let weight = if dirs[dir] == delta {
                1
            } else if dirs[dir] == -delta {
                continue;
            } else {
                1001
            };

            let next_score = score + weight;

            if dists[next as usize][i] > next_score {
                dists[next as usize][i] = next_score;
                heap.push((Reverse(next_score), next, i));
            }
        }
    }

    panic!("no path found")
}

fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let w = memchr::memchr(b'\n', bytes).unwrap() as i32 + 1;
    let start = memchr::memchr(b'S', bytes).unwrap() as i32;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start, 1));

    let mut best = u32::MAX;

    let mut dists = vec![[u32::MAX; 4]; bytes.len()];
    dists[start as usize][1] = 0;

    let mut prevs: Vec<_> = (0..bytes.len())
        .map(|_| [(); 4].map(|_| HashSet::with_hasher(FxBuildHasher::default())))
        .collect();

    let mut run_back = Vec::new();

    let dirs = directions(w);

    while let Some((Reverse(score), pos, dir)) = heap.pop() {
        if dists[pos as usize][dir] < score {
            continue;
        } else if score > best {
            break;
        } else if bytes[pos as usize] == b'E' {
            best = score;
            run_back.push((pos, dir));

            continue;
        }

        for (i, delta) in dirs.into_iter().enumerate() {
            let next = pos + delta;

            if bytes[next as usize] == b'#' {
                continue;
            }

            let weight = if dirs[dir] == delta {
                1
            } else if dirs[dir] == -delta {
                continue;
            } else {
                1001
            };

            let next_score = score + weight;

            match dists[next as usize][i].cmp(&next_score) {
                Ordering::Greater => {
                    dists[next as usize][i] = next_score;
                    heap.push((Reverse(next_score), next, i));

                    prevs[next as usize][i].clear();
                    prevs[next as usize][i].insert((pos, dir));
                }
                Ordering::Equal => {
                    prevs[next as usize][i].insert((pos, dir));
                }
                Ordering::Less => {}
            }
        }
    }

    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut seen = HashSet::with_hasher(FxBuildHasher::default());

    while let Some((pos, dir)) = run_back.pop() {
        if !seen.insert((pos, dir)) {
            continue;
        }

        for &(prev, dir) in prevs[pos as usize][dir].iter() {
            set.insert(prev);
            run_back.push((prev, dir));
        }
    }

    1 + set.len()
}

fn directions(w: i32) -> [i32; 4] {
    [-1, 1, -w, w]
}
