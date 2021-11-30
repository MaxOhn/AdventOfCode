use std::ops::{Index, IndexMut};

use crate::Pos2;

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
}

impl<T: PartialEq> Matrix<T> {
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
    fn neighbors_centre() {
        let matrix = basic_matrix();

        assert_eq!(matrix.count_neighbors(1, 1, 2), 1);
    }

    #[test]
    fn neighbors_border() {
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
