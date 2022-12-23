use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    mem,
    ops::Add,
};

use ahash::RandomState;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let elves = parse_elves(input);

    let p1 = part1(elves.clone());
    let p2 = part2(elves);

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

fn part1(elves: Elves) -> i16 {
    let mut state = State::new(elves);

    for _ in 0..10 {
        state.borders = Borders::default();
        iteration(&mut state);
    }

    state.borders.area() - state.elves.len() as i16
}

fn part2(elves: Elves) -> i32 {
    let mut state = State::new(elves);

    for round in 1.. {
        if !iteration(&mut state) {
            return round;
        }
    }

    unreachable!()
}

type Elves = HashSet<Pos, RandomState>;
type Plans = HashMap<Pos, Vec<Pos>, RandomState>;

fn iteration(state: &mut State) -> bool {
    let State {
        elves,
        bufs,
        plans,
        cases,
        borders,
    } = state;

    let mut has_motion = false;

    'next_elve: for &elve in elves.iter() {
        if Direction::iter().all(|dir| !elves.contains(&elve.neighbor(dir))) {
            bufs.elves.insert(elve);
            borders.update(elve);

            continue;
        }

        let mut steps = |directions: [Direction; 3]| {
            if directions
                .into_iter()
                .all(|direction| !elves.contains(&elve.neighbor(direction)))
            {
                let plan = directions[0];

                match plans.entry(elve.neighbor(plan)) {
                    Entry::Occupied(e) => e.into_mut().push(elve),
                    Entry::Vacant(e) => match bufs.lists.pop() {
                        Some(mut buf) => {
                            buf.push(elve);
                            e.insert(buf);
                        }
                        None => {
                            e.insert(vec![elve]);
                        }
                    },
                }

                true
            } else {
                false
            }
        };

        for check in *cases {
            if steps(check) {
                continue 'next_elve;
            }
        }

        bufs.elves.insert(elve);
        borders.update(elve);
    }

    mem::swap(&mut bufs.elves, elves);
    bufs.elves.clear();

    for (plan, mut list) in plans.drain() {
        if list.len() == 1 {
            elves.insert(plan);
            borders.update(plan);
            has_motion = true;
            list.clear();
        } else {
            for elve in list.drain(..) {
                elves.insert(elve);
                borders.update(elve);
            }
        }

        bufs.lists.push(list);
    }

    cases.rotate_left(1);

    has_motion
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
    fn update(&mut self, pos: Pos) {
        self.top = self.top.max(pos.y);
        self.bot = self.bot.min(pos.y);
        self.left = self.left.min(pos.x);
        self.right = self.right.max(pos.x);
    }

    fn area(&self) -> i16 {
        (self.top - self.bot + 1) * (self.right - self.left + 1)
    }
}

#[derive(Default)]
struct Buffers {
    elves: Elves,
    lists: Vec<Vec<Pos>>,
}

struct State {
    elves: Elves,
    bufs: Buffers,
    plans: Plans,
    cases: [[Direction; 3]; 4],
    borders: Borders,
}

impl State {
    fn new(elves: Elves) -> Self {
        Self {
            elves,
            bufs: Buffers::default(),
            plans: Plans::default(),
            cases: [
                [Direction::N, Direction::NE, Direction::NW],
                [Direction::S, Direction::SE, Direction::SW],
                [Direction::W, Direction::NW, Direction::SW],
                [Direction::E, Direction::NE, Direction::SE],
            ],
            borders: Borders::default(),
        }
    }
}
