use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use crate::util::{Pos2, Pos2Iter};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    entries: Box<[T]>,
    width: usize,
}

impl<T: Clone + Default> Matrix<T> {
    #[inline]
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            entries: vec![T::default(); columns * rows].into_boxed_slice(),
            width: columns,
        }
    }
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
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    #[allow(unused)]
    pub fn get<I>(&self, pos: Pos2<I>) -> Option<&T>
    where
        usize: TryFrom<I>,
    {
        let x = pos.x.try_into().ok()?;
        let y = pos.y.try_into().ok()?;

        (x < self.width && y < self.height()).then(|| self.index((x, y)))
    }

    #[inline]
    #[allow(unused)]
    pub fn get_mut<I>(&mut self, pos: Pos2<I>) -> Option<&mut T>
    where
        usize: TryFrom<I>,
    {
        let x = pos.x.try_into().ok()?;
        let y = pos.y.try_into().ok()?;

        (x < self.width && y < self.height()).then(|| self.index_mut((x, y)))
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

    #[inline]
    /// Iterates over neighbors:
    ///   123
    ///   4 5
    ///   678
    pub fn neighbors_8(&self, pos: Pos2<usize>) -> impl Iterator<Item = Pos2<usize>> {
        let cx = pos.x as isize;
        let cy = pos.y as isize;
        let w = self.width as isize;
        let h = self.height() as isize;

        let offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

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

    #[inline]
    pub fn pos_iter(&self) -> Pos2Iter {
        Pos2Iter::new(0, self.width - 1, 0, self.height() - 1)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.entries.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.entries.iter_mut()
    }
}

impl<T: PartialEq> Matrix<T> {
    #[allow(unused)]
    /// Count how many neighbors of `(x,y)` equal `n`.
    pub fn count_neighbors(&self, x: usize, y: usize, n: T) -> u8 {
        let width = self.width;
        let height = self.height();
        let mut neighbors = 0;

        for cx in x.saturating_sub(1)..=x + 1 {
            for cy in y.saturating_sub(1)..=y + 1 {
                neighbors +=
                    ((cx != x || cy != y) && cx < width && cy < height && self[(cx, cy)] == n)
                        as u8;
            }
        }

        neighbors
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.entries.index(y * self.width + x)
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.entries.index_mut(y * self.width + x)
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

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    #[inline]
    fn index(&self, y: usize) -> &Self::Output {
        let row = y * self.width;
        self.entries.index(row..row + self.width)
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        let row = y * self.width;
        self.entries.index_mut(row..row + self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_matrix() -> Matrix<u8> {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        Matrix::from_vec(vec, 3)
    }

    #[test]
    fn neighbors_4_centre() {
        let matrix = basic_matrix();

        let mut neighbors = matrix.neighbors_4(Pos2::new(1, 1));

        assert_eq!(neighbors.next(), Some(Pos2::new(1, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(0, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(2, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 2)));
        assert_eq!(neighbors.next(), None);
    }

    #[test]
    fn neighbors_4_edge() {
        let matrix = basic_matrix();

        let mut neighbors = matrix.neighbors_4(Pos2::new(0, 1));

        assert_eq!(neighbors.next(), Some(Pos2::new(0, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(0, 2)));
        assert_eq!(neighbors.next(), None);
    }

    #[test]
    fn neighbors_8_centre() {
        let matrix = basic_matrix();

        let mut neighbors = matrix.neighbors_8(Pos2::new(1, 1));

        assert_eq!(neighbors.next(), Some(Pos2::new(0, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(2, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(0, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(2, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(0, 2)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 2)));
        assert_eq!(neighbors.next(), Some(Pos2::new(2, 2)));
        assert_eq!(neighbors.next(), None);
    }

    #[test]
    fn neighbors_8_edge() {
        let matrix = basic_matrix();

        let mut neighbors = matrix.neighbors_8(Pos2::new(0, 1));

        assert_eq!(neighbors.next(), Some(Pos2::new(0, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 0)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 1)));
        assert_eq!(neighbors.next(), Some(Pos2::new(0, 2)));
        assert_eq!(neighbors.next(), Some(Pos2::new(1, 2)));
        assert_eq!(neighbors.next(), None);
    }

    #[test]
    fn count_neighbors_centre() {
        let matrix = basic_matrix();

        assert_eq!(matrix.count_neighbors(1, 1, 2), 1);
    }

    #[test]
    fn count_neighbors_edge() {
        let matrix = basic_matrix();

        assert_eq!(matrix.count_neighbors(0, 1, 7), 1);
        assert_eq!(matrix.count_neighbors(0, 1, 9), 0);
    }

    #[test]
    fn index_tuple() {
        let matrix = basic_matrix();

        assert_eq!(matrix[(2, 1)], 6);
    }

    #[test]
    fn index_row() {
        let matrix = basic_matrix();

        assert_eq!(&matrix[1], &[4, 5, 6]);
    }
}
