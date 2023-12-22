use std::{cmp, convert, ops::Range, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result};
use fxhash::FxHashSet as HashSet;

pub fn run(input: &str) -> Result<Solution> {
    let bricks = prepare_bricks(input.trim())?;

    let p1 = part1(&bricks);
    let p2 = part2(&bricks);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn prepare_bricks(input: &str) -> Result<Vec<Brick>> {
    let mut bricks = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Brick>>>()?;

    bricks.sort_unstable_by_key(|brick| brick.z.start);

    for i in 0..bricks.len() {
        let supports = bricks[..i]
            .iter()
            .enumerate()
            .filter(|(_, prev)| prev.crosses(&bricks[i]))
            .max_by_key(|(_, prev)| prev.z.end);

        match supports {
            Some((j, prev)) => {
                let move_down = bricks[i].z.start - prev.z.end;

                if move_down > 0 {
                    let k = bricks[j..i]
                        .binary_search_by_key(&prev.z.end, |probe| probe.z.start)
                        .map_or_else(convert::identity, convert::identity);

                    bricks[i].move_down(move_down);
                    bricks[j + k..=i].rotate_right(1);
                }
            }
            None => {
                let move_down = bricks[i].z.start - 1;

                if move_down > 0 {
                    bricks[i].move_down(move_down);
                    bricks[..=i].rotate_right(1);
                }
            }
        }
    }

    Ok(bricks)
}

fn part1(bricks: &[Brick]) -> usize {
    let (supports, supported_by): (Vec<Vec<_>>, Vec<Vec<_>>) = bricks
        .iter()
        .enumerate()
        .map(|(i, brick)| {
            let supports = bricks
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(_, next)| brick.z.end == next.z.start && brick.crosses(next))
                .map(|(j, _)| j)
                .collect();

            let supported_by = bricks[..i]
                .iter()
                .enumerate()
                .filter(|(_, prev)| prev.z.end == brick.z.start && prev.crosses(brick))
                .map(|(j, _)| j)
                .collect();

            (supports, supported_by)
        })
        .unzip();

    (0..bricks.len())
        .filter(|&i| supports[i].iter().all(|&j| supported_by[j].len() > 1))
        .count()
}

fn part2(bricks: &[Brick]) -> usize {
    let mut curr_bricks = Vec::with_capacity(bricks.len());
    let mut bricked = HashSet::default();
    let mut sum = 0;

    for i in (0..bricks.len()).rev() {
        curr_bricks.clear();
        curr_bricks.extend_from_slice(bricks);
        curr_bricks.remove(i);

        bricked.clear();
        bricked.extend(curr_bricks.iter().flat_map(Brick::cubes));

        loop {
            let mut done = true;

            curr_bricks.retain(|brick| {
                if brick.z.start == 1 {
                    return true;
                }

                let no_brick = |pos| {
                    !bricked.contains(&pos)
                        || (brick.z.start != brick.z.end && pos.2 >= brick.z.start)
                };

                if brick.cubes().map(|(x, y, z)| (x, y, z - 1)).all(no_brick) {
                    sum += 1;

                    for curr in brick.cubes() {
                        bricked.remove(&curr);
                    }

                    done = false;

                    false
                } else {
                    true
                }
            });

            if done {
                break;
            }
        }
    }

    sum
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    x: Range<u16>,
    y: Range<u16>,
    z: Range<u16>,
}

impl Brick {
    fn move_down(&mut self, move_down: u16) {
        self.z.start -= move_down;
        self.z.end -= move_down;
    }

    fn crosses(&self, other: &Self) -> bool {
        let x = cmp::max(self.x.start, other.x.start)..cmp::min(self.x.end, other.x.end);
        let y = cmp::max(self.y.start, other.y.start)..cmp::min(self.y.end, other.y.end);

        !(x.is_empty() || y.is_empty())
    }

    fn cubes(&self) -> impl Iterator<Item = (u16, u16, u16)> + '_ {
        self.x.clone().flat_map(move |x| {
            self.y
                .clone()
                .flat_map(move |y| self.z.clone().map(move |z| (x, y, z)))
        })
    }
}

impl FromStr for Brick {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (front, back) = s.split_once('~').wrap_err("invalid brick")?;
        let front: Triple = front.parse()?;
        let back: Triple = back.parse()?;

        let x = cmp::min(front.x, back.x)..cmp::max(front.x, back.x) + 1;
        let y = cmp::min(front.y, back.y)..cmp::max(front.y, back.y) + 1;
        let z = cmp::min(front.z, back.z)..cmp::max(front.z, back.z) + 1;

        Ok(Self { x, y, z })
    }
}

struct Triple {
    x: u16,
    y: u16,
    z: u16,
}

impl FromStr for Triple {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',').map(str::parse).flat_map(Result::ok);

        let ((x, y), z) = split
            .next()
            .zip(split.next())
            .zip(split.next())
            .wrap_err("invalid triple")?;

        Ok(Self { x, y, z })
    }
}
