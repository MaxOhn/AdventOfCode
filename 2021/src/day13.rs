use std::{collections::HashSet, fmt::Write};

use aoc_rust::Solution;
use eyre::Result;

use crate::util::{Parse, Pos2};

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();
    let mut dots: Vec<Pos2<usize>> = Vec::new();

    for line in lines.by_ref() {
        if line.len() == 1 {
            break;
        }

        let (left, right) = line.trim_end().split_once(',').unwrap();

        let left = left.as_bytes().parse();
        let right = right.as_bytes().parse();
        let pos = Pos2 { x: left, y: right };

        dots.push(pos);
    }

    let p1 = {
        let line_ = lines.next().unwrap().trim_end().as_bytes();
        let axis = line_[11];
        let n = (&line_[13..]).parse();
        fold(axis, n, &mut dots);

        dots.iter().collect::<HashSet<_>>().len()
    };

    for line in lines {
        let line = line.trim_end().as_bytes();

        let axis = line[11];
        let n = (&line[13..]).parse();
        fold(axis, n, &mut dots);
    }

    let p2 = print(&dots);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn fold(axis: u8, n: usize, dots: &mut [Pos2<usize>]) {
    if axis == b'x' {
        for dot in dots.iter_mut() {
            if dot.x > n {
                dot.x = n - (dot.x - n);
            }
        }
    } else {
        for dot in dots.iter_mut() {
            if dot.y > n {
                dot.y = n - (dot.y - n);
            }
        }
    }
}

fn print(dots: &[Pos2<usize>]) -> String {
    let mut max_x = 0;
    let mut max_y = 0;

    for dot in dots {
        max_x = max_x.max(dot.x);
        max_y = max_y.max(dot.y);
    }

    let mut grid = vec![vec![b'.'; max_x + 1]; max_y + 1];
    let mut ret = String::with_capacity(grid.len() + max_y);

    for dot in dots {
        grid[dot.y as usize][dot.x] = b'#';
    }

    for row in grid {
        let _ = writeln!(ret, "{}", std::str::from_utf8(&row).unwrap());
    }

    ret.pop();

    ret
}
