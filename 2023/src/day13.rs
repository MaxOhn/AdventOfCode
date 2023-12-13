use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let mut grids = parse_grids(input.trim())?;

    let p1 = part1(&grids)?;
    let p2 = part2(&mut grids)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

type Grid = Vec<Vec<Field>>;

fn parse_grids(input: &str) -> Result<Vec<Grid>> {
    let mut grids = Vec::new();

    for grid in input.split("\n\n") {
        let grid = grid
            .lines()
            .map(|line| {
                line.bytes()
                    .map(Field::try_from)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        grids.push(grid);
    }

    Ok(grids)
}

fn part1(grids: &[Grid]) -> Result<usize> {
    let mut sum = 0;
    let mut values = Vec::new();

    for grid in grids.iter() {
        find_mirrored(grid, &mut values);
        sum += values.first().wrap_err("no mirrored line for grid")?;
    }

    Ok(sum)
}

fn part2(grids: &mut [Grid]) -> Result<usize> {
    let mut sum = 0;
    let mut values = Vec::new();

    'next: for grid in grids.iter_mut() {
        find_mirrored(grid, &mut values);
        let orig = values[0];

        for smudge_x in 0..grid[0].len() {
            for smudge_y in 0..grid.len() {
                grid[smudge_y][smudge_x].flip();
                find_mirrored(grid, &mut values);

                if let Some(curr) = values.iter().find(|&n| *n != orig) {
                    sum += curr;
                    continue 'next;
                }

                grid[smudge_y][smudge_x].flip();
            }
        }

        eyre::bail!("no mirrored line for grid after smudging");
    }

    Ok(sum)
}

fn find_mirrored(grid: &Grid, values: &mut Vec<usize>) {
    values.clear();
    vertical_mirror(grid, values);
    horizontal_mirror(grid, values);
}

fn vertical_mirror(grid: &Grid, values: &mut Vec<usize>) {
    'next: for col in 0..grid[0].len() - 1 {
        let mut offset = 0;

        while col >= offset && col + offset + 1 < grid[0].len() {
            for y in 0..grid.len() {
                if grid[y][col - offset] != grid[y][col + 1 + offset] {
                    continue 'next;
                }
            }

            offset += 1;
        }

        values.push(col + 1);
    }
}

fn horizontal_mirror(grid: &Grid, values: &mut Vec<usize>) {
    'next: for row in 0..grid.len() - 1 {
        let mut offset = 0;

        while row >= offset && row + offset + 1 < grid.len() {
            let a = &grid[row - offset];
            let b = &grid[row + 1 + offset];

            if a != b {
                continue 'next;
            }

            offset += 1;
        }

        values.push(100 * (row + 1));
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Field {
    Ash,
    Rock,
}

impl Field {
    fn flip(&mut self) {
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
            _ => bail!("invalid field byte `{byte}`"),
        }
    }
}
