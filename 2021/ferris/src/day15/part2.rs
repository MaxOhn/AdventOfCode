use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    iter,
    ops::{Index, IndexMut},
};

use memchr::{memchr, memchr_iter};

pub fn run(input: &[u8]) -> i64 {
    let matrix = parse_input(input);
    let w = matrix.width();
    let h = matrix.height();

    let start = Pos2::new(0, 0);
    let end = Pos2::new(w - 1, h - 1);

    let dists: Vec<_> = iter::repeat(u16::MAX).take(w * h).collect();
    let mut dists = Matrix::from_vec(dists, w);
    dists[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State::new(start));

    while let Some(State { pos, cost }) = heap.pop() {
        if pos == end {
            break;
        } else if cost > dists[pos] {
            continue;
        }

        for n in matrix.neighbors_4(pos) {
            let cost = cost + matrix[n] as u16;

            if cost < dists[n] {
                dists[n] = cost;
                heap.push(State { pos: n, cost });
            }
        }
    }

    dists[end] as i64
}

fn parse_input(input: &[u8]) -> Matrix<u8> {
    let trim = input[input.len() - 1] == b'\n';
    let input = &input[..input.len() - trim as usize];

    let w = 5 * memchr(b'\n', input).unwrap();
    let mut grid = Vec::with_capacity(w * w);
    let mut prev = 0;

    for i in memchr_iter(b'\n', input) {
        for j in 0..5 {
            let row = input[prev..i]
                .iter()
                .map(|&b| (b & 0x0F) as u8 + j)
                .map(|n| n - (n > 9) as u8 * 9);

            grid.extend(row);
        }

        prev = i + 1;
    }

    for j in 0..5 {
        let row = input[prev..]
            .iter()
            .map(|&b| (b & 0x0F) as u8 + j)
            .map(|n| n - (n > 9) as u8 * 9);

        grid.extend(row);
    }

    let original = grid.clone();

    for i in 1..=4 {
        let row = original
            .iter()
            .map(|&n| n + i)
            .map(|n| n - (n > 9) as u8 * 9);

        grid.extend(row);
    }

    Matrix::from_vec(grid, w)
}

#[derive(Eq, PartialEq)]
struct State {
    pos: Pos2<usize>,
    cost: u16,
}

impl State {
    fn new(pos: Pos2<usize>) -> Self {
        Self { pos, cost: 0 }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct Matrix<T> {
    entries: Box<[T]>,
    width: usize,
}

impl<T> Matrix<T> {
    #[inline]
    pub fn from_vec(vec: Vec<T>, width: usize) -> Self {
        Self {
            entries: vec.into_boxed_slice(),
            width,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.entries.len() / self.width
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    /// Iterates over neighbors:
    ///    1
    ///   2 3
    ///    4
    pub fn neighbors_4(&self, pos: Pos2<usize>) -> impl Iterator<Item = Pos2<usize>> {
        let cx = pos.x as isize;
        let cy = pos.y as isize;
        let w = self.width as isize;
        let h = self.height() as isize;

        let offsets = [(0, -1), (-1, 0), (1, 0), (0, 1)];

        offsets
            .into_iter()
            .map(move |(x, y)| {
                let nx = cx + x;
                let ny = cy + y;

                (nx >= 0 && ny >= 0 && nx < w && ny < h)
                    .then(|| Pos2::new(nx as usize, ny as usize))
            })
            .flatten()
    }
}

impl<T> Index<Pos2<usize>> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Pos2<usize>) -> &Self::Output {
        self.entries.index(pos.y * self.width + pos.x)
    }
}

impl<T> IndexMut<Pos2<usize>> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, pos: Pos2<usize>) -> &mut Self::Output {
        self.entries.index_mut(pos.y * self.width + pos.x)
    }
}

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Pos2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
