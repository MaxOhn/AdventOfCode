use std::{cmp::Reverse, collections::BinaryHeap, iter, ops::Add};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 25)
}

type Cost = u64;
type Heap = BinaryHeap<(Reverse<Cost>, Pos, Button)>;

fn solve(input: &str, depth: u8) -> u64 {
    let mut heap = Heap::new();
    let costs = directional_costs(depth, &mut heap);

    input
        .lines()
        .map(|line| {
            let code: u64 = line[..3].parse().unwrap();

            let from = iter::once(b'A').chain(line.bytes());
            let to = line.bytes();

            let min_len: u64 = from
                .zip(to)
                .map(|(from, to)| numeric_cost(from, to, &costs, &mut heap))
                .sum();

            min_len * code
        })
        .sum()
}

fn numeric_cost(from: u8, to: u8, costs: &[[Cost; 5]; 5], heap: &mut Heap) -> Cost {
    let from_pos = from.pos();
    let to_pos = to.pos();

    heap.clear();
    heap.push((Reverse(0), from_pos, Button::A));

    while let Some((Reverse(cost), pos, button)) = heap.pop() {
        if pos == to_pos {
            if button == Button::A {
                return cost;
            }

            let next_cost = cost + costs[button as usize][Button::A as usize];
            heap.push((Reverse(next_cost), pos, Button::A));

            continue;
        }

        for delta in Pos::deltas() {
            let next_pos = pos + delta;

            if !next_pos.is_numeric_button() {
                continue;
            }

            let next_button = Button::from_delta(delta);
            let next_cost = cost + costs[button as usize][next_button as usize];
            heap.push((Reverse(next_cost), next_pos, next_button));
        }
    }

    unreachable!()
}

fn directional_costs(depth: u8, heap: &mut Heap) -> [[Cost; 5]; 5] {
    if depth == 0 {
        return [[1; 5]; 5];
    }

    let prev_costs = directional_costs(depth - 1, heap);
    let mut curr_costs = [[u64::MAX; 5]; 5];

    for from in Button::enumerate() {
        let mut seen = [false; 5];
        heap.push((Reverse(0), from.pos(), Button::A));

        while let Some((Reverse(cost), pos, button)) = heap.pop() {
            let Some(to) = pos.directional_button() else {
                continue;
            };

            if button == Button::A && cost > 0 {
                curr_costs[from as usize][to as usize] =
                    curr_costs[from as usize][to as usize].min(cost);

                continue;
            }

            let next_cost = cost + prev_costs[button as usize][Button::A as usize];
            heap.push((Reverse(next_cost), pos, Button::A));

            seen[to as usize] = true;

            for delta in Pos::deltas() {
                let next_pos = pos + delta;

                if next_pos
                    .directional_button()
                    .is_none_or(|n| seen[n as usize])
                {
                    continue;
                }

                let next_button = Button::from_delta(delta);
                let next_cost = cost + prev_costs[button as usize][next_button as usize];
                heap.push((Reverse(next_cost), next_pos, next_button));
            }
        }
    }

    curr_costs
}

trait NumericButton {
    fn pos(self) -> Pos;
}

impl NumericButton for u8 {
    fn pos(self) -> Pos {
        match self {
            b'0' => Pos::new(1, 3),
            b'1' => Pos::new(0, 2),
            b'2' => Pos::new(1, 2),
            b'3' => Pos::new(2, 2),
            b'4' => Pos::new(0, 1),
            b'5' => Pos::new(1, 1),
            b'6' => Pos::new(2, 1),
            b'7' => Pos::new(0, 0),
            b'8' => Pos::new(1, 0),
            b'9' => Pos::new(2, 0),
            b'A' => Pos::new(2, 3),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Button {
    const fn enumerate() -> [Self; 5] {
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::A]
    }

    const fn pos(self) -> Pos {
        match self {
            Self::Up => Pos::new(1, 0),
            Self::A => Pos::new(2, 0),
            Self::Left => Pos::new(0, 1),
            Self::Down => Pos::new(1, 1),
            Self::Right => Pos::new(2, 1),
        }
    }

    fn from_delta(delta: Pos) -> Self {
        match delta {
            Pos { x: 0, y: -1 } => Self::Up,
            Pos { x: 0, y: 1 } => Self::Down,
            Pos { x: -1, y: 0 } => Self::Left,
            Pos { x: 1, y: 0 } => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    const fn directional_button(self) -> Option<Button> {
        match self {
            Self { x: 1, y: 0 } => Some(Button::Up),
            Self { x: 2, y: 0 } => Some(Button::A),
            Self { x: 0, y: 1 } => Some(Button::Left),
            Self { x: 1, y: 1 } => Some(Button::Down),
            Self { x: 2, y: 1 } => Some(Button::Right),
            _ => None,
        }
    }

    const fn deltas() -> [Self; 4] {
        [
            Self::new(0, 1),
            Self::new(0, -1),
            Self::new(1, 0),
            Self::new(-1, 0),
        ]
    }

    fn is_numeric_button(self) -> bool {
        (self.x != 0 || self.y != 3) && (0..=2).contains(&self.x) && (0..=3).contains(&self.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
