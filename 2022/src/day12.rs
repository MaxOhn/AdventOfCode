use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::{Add, Index},
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    run_dijkstra(input)
    // run_a_star(input)
}

#[allow(unused)]
pub fn run_dijkstra(input: &str) -> Result<Solution> {
    let hill = Hill::parse(input)?;

    let goal = |pos| pos == hill.end;

    let valid_height = |curr, neighbor| curr + 1 >= neighbor;

    let start = State {
        pos: hill.start,
        height: 0,
        path_len: 0,
        f_score: (),
    };

    let p1 = hike_with_dijkstra(&hill, start, goal, valid_height).wrap_err("missing end")?;

    let goal = |pos| hill[pos] == 0;

    let valid_height = |curr, neighbor| curr <= neighbor + 1;

    let start = State {
        pos: hill.end,
        height: 25,
        path_len: 0,
        f_score: (),
    };

    let p2 =
        hike_with_dijkstra(&hill, start, goal, valid_height).wrap_err("missing height of 0")?;

    Ok(Solution::new().part1(p1).part2(p2))
}

#[allow(unused)]
pub fn run_a_star(input: &str) -> Result<Solution> {
    let hill = Hill::parse(input)?;

    let goal = |pos| pos == hill.end;

    let valid_height = |curr, neighbor| curr + 1 >= neighbor;

    let start = State {
        pos: hill.start,
        height: 0,
        path_len: 0,
        f_score: i32::MAX,
    };

    let heuristic = |hill: &Hill, pos: Pos| pos.manhatten_dist(&hill.end);

    let p1 =
        hike_with_a_star(&hill, start, goal, valid_height, heuristic).wrap_err("missing end")?;

    let goal = |pos| hill[pos] == 0;

    let valid_height = |curr, neighbor| curr <= neighbor + 1;

    let start = State {
        pos: hill.end,
        height: 25,
        path_len: 0,
        f_score: i32::MAX,
    };

    let heuristic = |hill: &Hill, pos: Pos| -(hill[pos] as i32);

    let p2 = hike_with_a_star(&hill, start, goal, valid_height, heuristic)
        .wrap_err("missing height of 0")?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn hike_with_dijkstra<G, V>(hill: &Hill, start: State, goal: G, valid_height: V) -> Option<usize>
where
    G: Fn(Pos) -> bool,
    V: Fn(u8, u8) -> bool,
{
    let mut heap = BinaryHeap::with_capacity(hill.width);
    let mut seen = HashSet::with_capacity(hill.width * 2);

    heap.push(start);

    while let Some(state) = heap.pop() {
        let State {
            pos,
            height,
            path_len,
            f_score: _,
        } = state;

        if goal(pos) {
            return Some(path_len);
        }

        if !seen.insert(pos) {
            continue;
        }

        const DIRECTIONS: [Pos; 4] = [
            Pos { x: -1, y: 0 },
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 1 },
        ];

        for direction in DIRECTIONS {
            let npos = pos + direction;

            if !hill.is_valid_pos(npos) {
                continue;
            }

            let nheight = hill[npos];

            if !valid_height(height, nheight) {
                continue;
            }

            let nstate = State {
                pos: npos,
                height: nheight,
                path_len: path_len + 1,
                f_score: (),
            };

            heap.push(nstate);
        }
    }

    None
}

fn hike_with_a_star<G, V, H>(
    hill: &Hill,
    start: State<i32>,
    goal: G,
    valid_height: V,
    heuristic: H,
) -> Option<usize>
where
    G: Fn(Pos) -> bool,
    V: Fn(u8, u8) -> bool,
    H: Fn(&Hill, Pos) -> i32,
{
    let mut heap = BinaryHeap::with_capacity(hill.width);

    let mut g_score = HashMap::with_capacity(hill.width * 2);
    g_score.insert(start.pos, 0);

    heap.push(start);

    let mut seen = HashSet::with_capacity(hill.width * 2);

    while let Some(current) = heap.pop() {
        let State {
            pos,
            height,
            path_len,
            f_score: _,
        } = current;

        if goal(pos) {
            return Some(path_len);
        }

        const DIRECTIONS: [Pos; 4] = [
            Pos { x: -1, y: 0 },
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 1 },
        ];

        for direction in DIRECTIONS {
            let npos = pos + direction;

            if !hill.is_valid_pos(npos) {
                continue;
            }

            let nheight = hill[npos];

            if !valid_height(height, nheight) {
                continue;
            }

            let g_score_current = *g_score.entry(pos).or_insert(i32::MAX);
            let g_score_neighbor = g_score.entry(npos).or_insert(i32::MAX);

            let tentative_g_score = g_score_current + 1;

            if tentative_g_score < *g_score_neighbor && seen.insert(npos) {
                *g_score_neighbor = tentative_g_score;

                let nstate = State {
                    pos: npos,
                    height: nheight,
                    path_len: path_len + 1,
                    f_score: tentative_g_score + heuristic(hill, npos),
                };

                heap.push(nstate);
            }
        }
    }

    None
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn manhatten_dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Pos {
    type Output = Pos;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Hill {
    start: Pos,
    end: Pos,
    inner: Box<[u8]>,
    width: usize,
}

impl Hill {
    fn parse(input: &str) -> Result<Self> {
        let mut lines = input.lines();

        fn map_byte((byte, _): (u8, i32)) -> Result<u8> {
            match byte {
                b'S' => Ok(0),
                b'E' => Ok(25),
                b'a'..=b'z' => Ok(byte - b'a'),
                _ => bail!("invalid square `{}`", byte as char),
            }
        }

        let mut start = None;
        let mut end = None;
        let mut y = 0;

        let mut inner = if let Some(line) = lines.next() {
            line.bytes()
                .zip(0..)
                .inspect(|&(byte, x)| {
                    if byte == b'S' {
                        start = Some(Pos { x, y });
                    } else if byte == b'E' {
                        end = Some(Pos { x, y });
                    }
                })
                .map(map_byte)
                .collect::<Result<_>>()?
        } else {
            Vec::new()
        };

        let width = inner.len();
        y += 1;

        for line in lines {
            let mut buf = line
                .bytes()
                .zip(0..)
                .inspect(|&(byte, x)| {
                    if byte == b'S' {
                        start = Some(Pos { x, y });
                    } else if byte == b'E' {
                        end = Some(Pos { x, y });
                    }
                })
                .map(map_byte)
                .collect::<Result<_>>()?;

            inner.append(&mut buf);
            y += 1;
        }

        let inner = inner.into();
        let start = start.wrap_err("missing start")?;
        let end = end.wrap_err("missing end")?;

        Ok(Self {
            inner,
            width,
            start,
            end,
        })
    }

    fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    fn is_valid_pos(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height() as i32
    }
}

impl Index<Pos> for Hill {
    type Output = u8;

    #[inline]
    fn index(&self, idx: Pos) -> &Self::Output {
        let idx = idx.y as usize * self.width + idx.x as usize;

        &self.inner[idx]
    }
}

struct State<S = ()> {
    pos: Pos,
    height: u8,
    path_len: usize,
    f_score: S,
}

impl Ord for State<()> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.path_len.cmp(&self.path_len)
    }
}

impl Ord for State<i32> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for State<()> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for State<i32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> PartialEq for State<S> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<S> Eq for State<S> {}

impl<S> Hash for State<S> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
