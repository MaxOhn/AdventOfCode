use std::{collections::HashSet, mem, ops::Add};

use ahash::RandomState;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let elves = parse_elves(input);
    let mut state = State::new(elves);

    let p1 = part1(&mut state);
    let p2 = part2(&mut state) + 10;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_elves(input: &str) -> HashSet<Pos, RandomState> {
    input
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| {
            line.bytes()
                .zip(0..)
                .filter(|(tile, _)| *tile == b'#')
                .map(move |(_, x)| Pos { x, y })
        })
        .collect()
}

fn part1(state: &mut State) -> i16 {
    for _ in 0..10 {
        iteration(state);
    }

    let total_area = state
        .elves
        .iter()
        .fold(Borders::default(), |borders, &elve| borders.update(elve))
        .area();

    total_area - state.elves.len() as i16
}

fn part2(state: &mut State) -> i32 {
    for round in 1.. {
        if !iteration(state) {
            return round;
        }
    }

    unreachable!()
}

type Elves = HashSet<Pos, RandomState>;

fn iteration(state: &mut State) -> bool {
    let State { elves, bufs, cases } = state;

    let mut moved = 0;

    'next_elve: for &elve in elves.iter() {
        if Direction::iter().all(|dir| !elves.contains(&elve.neighbor(dir))) {
            bufs.elves.insert(elve);

            continue;
        }

        let mut steps = |directions: [Direction; 3]| {
            let is_empty = directions
                .into_iter()
                .all(|direction| !elves.contains(&elve.neighbor(direction)));

            if is_empty {
                let neighbor = elve.neighbor(directions[0]);

                if !bufs.elves.insert(neighbor) {
                    bufs.elves.remove(&neighbor);
                    bufs.elves.insert(elve);
                    bufs.elves.insert(Pos {
                        x: neighbor.x * 2 - elve.x,
                        y: neighbor.y * 2 - elve.y,
                    });
                    moved -= 2;
                } else {
                    moved += 1;
                }
            }

            is_empty
        };

        for check in *cases {
            if steps(check) {
                continue 'next_elve;
            }
        }

        bufs.elves.insert(elve);
    }

    mem::swap(&mut bufs.elves, elves);
    bufs.elves.clear();
    cases.rotate_left(1);

    moved > 0
}

#[derive(Copy, Clone)]
struct Direction(u8);

impl Direction {
    const N: Self = Self(0);
    const NE: Self = Self(1);
    const E: Self = Self(2);
    const SE: Self = Self(3);
    const S: Self = Self(4);
    const SW: Self = Self(5);
    const W: Self = Self(6);
    const NW: Self = Self(7);

    fn delta(self) -> Pos {
        const DELTAS: [Pos; 8] = [
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: 1, y: 1 },
            Pos { x: 0, y: 1 },
            Pos { x: -1, y: 1 },
            Pos { x: -1, y: 0 },
            Pos { x: -1, y: -1 },
        ];

        DELTAS[self.0 as usize]
    }

    fn iter() -> Self {
        Self::N
    }
}

impl Iterator for Direction {
    type Item = Self;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = *self;
        self.0 += 1;

        (next.0 < 8).then_some(next)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn neighbor(self, direction: Direction) -> Self {
        self + direction.delta()
    }
}

impl Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Borders {
    top: i16,
    bot: i16,
    left: i16,
    right: i16,
}

impl Default for Borders {
    #[inline]
    fn default() -> Self {
        Self {
            top: i16::MIN,
            bot: i16::MAX,
            left: i16::MAX,
            right: i16::MIN,
        }
    }
}

impl Borders {
    fn update(self, pos: Pos) -> Self {
        Self {
            top: self.top.max(pos.y),
            bot: self.bot.min(pos.y),
            left: self.left.min(pos.x),
            right: self.right.max(pos.x),
        }
    }

    fn area(&self) -> i16 {
        (self.top - self.bot + 1) * (self.right - self.left + 1)
    }
}

#[derive(Default)]
struct Buffers {
    elves: Elves,
}

struct State {
    elves: Elves,
    bufs: Buffers,
    cases: [[Direction; 3]; 4],
}

impl State {
    fn new(elves: Elves) -> Self {
        Self {
            elves,
            bufs: Buffers::default(),
            cases: [
                [Direction::N, Direction::NE, Direction::NW],
                [Direction::S, Direction::SE, Direction::SW],
                [Direction::W, Direction::NW, Direction::SW],
                [Direction::E, Direction::NE, Direction::SE],
            ],
        }
    }
}
