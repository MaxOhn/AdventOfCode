use crate::{Error, Solution};

use std::collections::HashSet;

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<usize, usize>, Error> {
    let area: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let p1 = solve_part1(area.clone());
    const STEPS: usize = 200;
    let mut layers = Layers::new(&area, STEPS * 2);
    for _ in 0..STEPS {
        layers.step();
    }
    let p2 = layers.count_bugs();
    Ok(Solution::new(p1, p2))
} // 591.25ms

struct Layers {
    layers: Vec<Vec<char>>,
    outer: usize,
    inner: usize,
}

impl Layers {
    fn new(init_area: &[char], amount: usize) -> Self {
        let mut layers = vec![vec!['.'; 25]; amount + 4];
        layers[amount / 2 + 2].copy_from_slice(init_area);
        Self {
            layers,
            outer: amount / 2 + 1,
            inner: amount / 2 + 3,
        }
    }

    fn step(&mut self) {
        let mut new_layers = self.layers.clone();
        for (idx, area) in self
            .layers
            .iter()
            .enumerate()
            .take(self.inner + 1)
            .skip(self.outer)
        {
            let mut new_area = ['.'; 25];
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let pos = y * 5 + x;
                    let mut bugs = 0;
                    for (nx, ny) in get_neighbors(x, y) {
                        if nx == 2 && ny == 2 {
                            let inner_area = &self.layers[idx + 1];
                            if (x == 2 && y == 1)
                                || (x == 2 && y == 3)
                                || (x == 1 && y == 2)
                                || (x == 3 && y == 2)
                            {
                                bugs += get_inner_bugs(inner_area, x, y);
                            }
                        } else if area[ny * 5 + nx] == '#' {
                            bugs += 1;
                        }
                    }
                    let outer_area = &self.layers[idx - 1];
                    bugs += get_outer_bugs(outer_area, x, y);
                    new_area[pos] = if area[pos] == '#' && bugs != 1 {
                        '.'
                    } else if area[pos] == '.' && (bugs == 1 || bugs == 2) {
                        '#'
                    } else {
                        area[pos]
                    }
                }
            }
            new_layers[idx].clone_from_slice(&new_area);
        }
        self.layers.clone_from_slice(&new_layers);
        if self.layers[self.outer].iter().any(|c| *c == '#') {
            self.outer -= 1;
        }
        if self.layers[self.inner].iter().any(|c| *c == '#') {
            self.inner += 1;
        }
    }

    fn count_bugs(&self) -> usize {
        let mut bugs: usize = 0;
        for idx in self.outer..=self.inner {
            bugs += self.layers[idx]
                .iter()
                .fold(0, |sum, next| if *next == '#' { sum + 1 } else { sum });
        }
        bugs
    }
}

fn get_inner_bugs(area: &[char], x: usize, y: usize) -> usize {
    let mut bugs = 0;
    for i in 0..5 {
        let pos = if y == 1 {
            i
        } else if y == 3 {
            20 + i
        } else if x == 1 {
            i * 5
        } else {
            i * 5 + 4
        };
        if area[pos] == '#' {
            bugs += 1;
        }
    }
    bugs
}

fn get_outer_bugs(area: &[char], x: usize, y: usize) -> usize {
    let mut bugs = 0;
    if x == 0 && area[11] == '#' {
        bugs += 1;
    }
    if x == 4 && area[13] == '#' {
        bugs += 1;
    }
    if y == 0 && area[7] == '#' {
        bugs += 1;
    }
    if y == 4 && area[17] == '#' {
        bugs += 1;
    }
    bugs
}

fn solve_part1(mut area: Vec<char>) -> usize {
    let mut seen = HashSet::new();
    let mut new_area = ['.'; 25];
    while seen.insert(area.clone()) {
        for y in 0..5 {
            for x in 0..5 {
                let pos = y * 5 + x;
                let mut bugs = 0;
                for (nx, ny) in get_neighbors(x, y) {
                    if area[ny * 5 + nx] == '#' {
                        bugs += 1;
                    }
                }
                new_area[pos] = if area[pos] == '#' && bugs != 1 {
                    '.'
                } else if area[pos] == '.' && (bugs == 1 || bugs == 2) {
                    '#'
                } else {
                    area[pos]
                }
            }
        }
        area.clone_from_slice(&new_area);
    }
    area.iter().enumerate().fold(0, |total, (exp, next)| {
        if *next == '#' {
            total + (1 << exp)
        } else {
            total
        }
    })
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < 4 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < 4 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test24() {
        crate::util::tests::test_full_problem(24, solve, 18844281, 1872);
    }
}
