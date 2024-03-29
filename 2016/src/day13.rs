use std::collections::{HashSet, VecDeque};

use aoc_rust::Solution;

pub fn run(_input: &str) -> eyre::Result<Solution> {
    Ok(Solution::new().part1(part1()).part2(part2()))
}

fn is_open(x: i64, y: i64) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + FAVOURITE_NUMBER).count_ones() % 2 == 0
}

static OFFSETS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

const FAVOURITE_NUMBER: i64 = 1358;
const GOAL_X: i64 = 31;
const GOAL_Y: i64 = 39;

pub fn part1() -> usize {
    let mut queue = VecDeque::with_capacity(16);
    let mut visited = HashSet::with_capacity(128);
    queue.push_back((1, 1, 0));
    visited.insert((1, 1));

    while let Some((x, y, d)) = queue.pop_front() {
        if x == GOAL_X && y == GOAL_Y {
            return d;
        }

        let neighbors = OFFSETS
            .iter()
            .map(|&(dx, dy)| (dx + x, dy + y))
            .filter(|&(cx, cy)| cx >= 0 && cy >= 0)
            .filter(|&(cx, cy)| is_open(cx, cy))
            .filter(|&(cx, cy)| visited.insert((cx, cy)))
            .map(|(cx, cy)| (cx, cy, d + 1));

        queue.extend(neighbors);
    }

    unsafe { std::hint::unreachable_unchecked() }
}

pub fn part2() -> usize {
    let mut stack = VecDeque::with_capacity(16);
    let mut visited = HashSet::with_capacity(256);
    stack.push_back((1, 1, 0));
    visited.insert((1, 1));

    while let Some((x, y, d)) = stack.pop_front() {
        if d == 50 {
            continue;
        }

        let neighbors = OFFSETS
            .iter()
            .map(|&(dx, dy)| (dx + x, dy + y))
            .filter(|&(cx, cy)| cx >= 0 && cy >= 0)
            .filter(|&(cx, cy)| is_open(cx, cy))
            .filter(|&(cx, cy)| visited.insert((cx, cy)))
            .map(|(cx, cy)| (cx, cy, d + 1));

        stack.extend(neighbors);
    }

    visited.len()
}
