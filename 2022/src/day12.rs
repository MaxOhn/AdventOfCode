use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Index},
    slice::ChunksExact,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let (hill, start) = Hill::parse(input)?;

    let p1 = hike(&hill, start, None).wrap_err("missing end")?;

    let p2 = hill
        .find_all(|square| square.height == 0)
        .fold(usize::MAX, |min, start| {
            match hike(&hill, start, Some(min)) {
                Some(path_len) => path_len.min(min),
                None => min,
            }
        });

    Ok(Solution::new().part1(p1).part2(p2))
}

fn hike(hill: &Hill, start: Pos, threshold: Option<usize>) -> Option<usize> {
    let mut heap = BinaryHeap::with_capacity(hill.width);
    let mut seen = HashSet::with_capacity(hill.width);
    let threshold = threshold.unwrap_or(usize::MAX);

    heap.push(State {
        pos: start,
        square: Square { height: 0 },
        path_len: 0,
    });

    while let Some(state) = heap.pop() {
        let State {
            pos,
            square,
            path_len,
        } = state;

        if square.is_end() {
            return Some(path_len);
        }

        if !seen.insert(pos) || path_len == threshold {
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

            let nsquare = hill[npos];

            if square.height + 1 < nsquare.height {
                continue;
            }

            let nstate = State {
                pos: npos,
                square: nsquare,
                path_len: path_len + 1,
            };

            heap.push(nstate);
        }
    }

    None
}

struct State {
    pos: Pos,
    square: Square,
    path_len: usize,
}

impl Ord for State {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let this = self.square.height as usize + self.path_len;
        let that = other.square.height as usize + other.path_len;

        that.cmp(&this)
    }
}

impl PartialOrd for State {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for State {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Square {
    height: u8,
}

impl Square {
    const END: u8 = 26;

    fn is_end(self) -> bool {
        self.height == Self::END
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Square { height: Self::END } => f.write_str("E"),
            Square { height } => write!(f, "{}", (height + b'a') as char),
        }
    }
}

struct Hill {
    inner: Box<[Square]>,
    width: usize,
}

impl Hill {
    fn parse(input: &str) -> Result<(Self, Pos)> {
        let mut lines = input.lines();

        fn map_byte((byte, _): (u8, i32)) -> Result<Square> {
            match byte {
                b'S' => Ok(Square { height: 0 }),
                b'E' => Ok(Square {
                    height: Square::END,
                }),
                b'a'..=b'z' => Ok(Square {
                    height: byte - b'a',
                }),
                _ => bail!("invalid square `{}`", byte as char),
            }
        }

        let mut start = None;
        let mut y = 0;

        let mut inner = if let Some(line) = lines.next() {
            line.bytes()
                .zip(0..)
                .inspect(|&(byte, x)| {
                    if byte == b'S' {
                        start = Some(Pos { x, y })
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
                        start = Some(Pos { x, y })
                    }
                })
                .map(map_byte)
                .collect::<Result<_>>()?;

            inner.append(&mut buf);
            y += 1;
        }

        let inner = inner.into();
        let start = start.wrap_err("missing start")?;

        Ok((Self { inner, width }, start))
    }

    fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    fn is_valid_pos(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height() as i32
    }

    fn find_all<F>(&self, f: F) -> HillIter<'_, F>
    where
        F: Fn(&Square) -> bool,
    {
        let mut chunks = self.inner.chunks_exact(self.width);
        let chunk = chunks.next();

        HillIter {
            filter: f,
            x: 0,
            y: 0,
            chunks,
            chunk,
        }
    }
}

impl Index<Pos> for Hill {
    type Output = Square;

    #[inline]
    fn index(&self, idx: Pos) -> &Self::Output {
        let idx = idx.y as usize * self.width + idx.x as usize;

        self.inner.index(idx)
    }
}

impl Display for Hill {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut chunks = self.inner.chunks_exact(self.width);

        if let Some(chunk) = chunks.next() {
            for square in chunk {
                write!(f, "{square}")?;
            }
        }

        for chunk in chunks {
            f.write_str("\n")?;

            for square in chunk {
                write!(f, "{square}")?;
            }
        }

        Ok(())
    }
}

struct HillIter<'h, F> {
    filter: F,
    x: i32,
    y: i32,
    chunks: ChunksExact<'h, Square>,
    chunk: Option<&'h [Square]>,
}

impl<'h, F> Iterator for HillIter<'h, F>
where
    F: Fn(&Square) -> bool,
{
    type Item = Pos;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let chunk = self.chunk?;

            match chunk.split_first() {
                Some((first, rest)) => {
                    self.chunk = Some(rest);

                    if (self.filter)(first) {
                        let x = self.x;
                        self.x += 1;

                        return Some(Pos { x, y: self.y });
                    }
                }
                None => {
                    self.x = 0;
                    self.y += 1;
                    self.chunk = self.chunks.next();
                }
            }
        }
    }
}
