use aoc_rust::Solution;
use eyre::{ContextCompat, Result};

use self::grid::Grid;

pub fn run(input: &str) -> Result<Solution> {
    let mut grids = input
        .trim()
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<Grid>>>()?;

    let p1 = part1(&grids)?;
    let p2 = part2(&mut grids)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(grids: &[Grid]) -> Result<usize> {
    grids
        .iter()
        .try_fold(0, |sum, grid| Some(sum + find_mirrored(grid, usize::MAX)?))
        .wrap_err("no mirrored line for grid")
}

fn part2(grids: &mut [Grid]) -> Result<usize> {
    let mut sum = 0;

    'next: for grid in grids.iter_mut() {
        let orig = find_mirrored(grid, usize::MAX).wrap_err("no mirrored line for grid")?;

        for i in 0..grid.len() {
            grid[i].flip();

            if let Some(curr) = find_mirrored(grid, orig) {
                sum += curr;
                continue 'next;
            }

            grid[i].flip();
        }

        eyre::bail!("no mirrored line for grid after smudging");
    }

    Ok(sum)
}

fn find_mirrored(grid: &Grid, not_equal: usize) -> Option<usize> {
    vertical_mirror(grid, not_equal).or_else(|| horizontal_mirror(grid, not_equal))
}

fn vertical_mirror(grid: &Grid, not_equal: usize) -> Option<usize> {
    'next: for col in 0..grid.width() - 1 {
        let mut offset = 0;

        while col >= offset && col + offset + 1 < grid.width() {
            for row in grid.rows() {
                if row[col - offset] != row[col + 1 + offset] {
                    continue 'next;
                }
            }

            offset += 1;
        }

        let value = col + 1;

        if value != not_equal {
            return Some(value);
        }
    }

    None
}

fn horizontal_mirror(grid: &Grid, not_equal: usize) -> Option<usize> {
    'next: for row in 0..grid.height() - 1 {
        let mut offset = 0;

        while row >= offset && row + offset + 1 < grid.height() {
            let a = grid.row(row - offset);
            let b = grid.row(row + 1 + offset);

            if a != b {
                continue 'next;
            }

            offset += 1;
        }

        let value = 100 * (row + 1);

        if value != not_equal {
            return Some(value);
        }
    }

    None
}

mod grid {
    use std::{
        ops::{Index, IndexMut},
        slice::SliceIndex,
        str::FromStr,
    };

    use eyre::Report;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Field {
        Ash,
        Rock,
    }

    impl Field {
        pub fn flip(&mut self) {
            *self = match self {
                Self::Ash => Self::Rock,
                Self::Rock => Self::Ash,
            }
        }
    }

    impl TryFrom<u8> for Field {
        type Error = Report;

        fn try_from(byte: u8) -> Result<Self, Self::Error> {
            match byte {
                b'.' => Ok(Self::Ash),
                b'#' => Ok(Self::Rock),
                _ => eyre::bail!("invalid field byte `{byte}`"),
            }
        }
    }

    pub struct Grid {
        width: usize,
        inner: Vec<Field>,
    }

    impl Grid {
        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.inner.len() / self.width
        }

        pub fn len(&self) -> usize {
            self.inner.len()
        }

        pub fn row(&self, idx: usize) -> &[Field] {
            &self.inner[idx * self.width()..][..self.width()]
        }

        pub fn rows(&self) -> impl Iterator<Item = &[Field]> {
            self.inner.chunks_exact(self.width())
        }
    }

    impl FromStr for Grid {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map_or(0, str::len);

            let inner = s
                .lines()
                .flat_map(|line| line.bytes().map(Field::try_from))
                .collect::<Result<Vec<_>, Report>>()?;

            Ok(Self { width, inner })
        }
    }

    impl<I: SliceIndex<[Field]>> Index<I> for Grid {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            self.inner.index(index)
        }
    }

    impl<I: SliceIndex<[Field]>> IndexMut<I> for Grid {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            self.inner.index_mut(index)
        }
    }
}
