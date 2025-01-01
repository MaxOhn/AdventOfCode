use crate::{
    computer::Computer,
    util::{Direction, GridMap, Point2},
    Error, Solution,
};
use std::collections::HashMap;

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<usize, String>, Error> {
    let mut grid = GridMap::new();
    execute(0, input, &mut grid)?;
    let p1 = grid.len();
    grid.clear();
    execute(1, input, &mut grid)?;
    let mut mapping = HashMap::new();
    mapping.insert(0, ' ');
    mapping.insert(1, '█');
    let p2 = grid
        .iter()
        .filter(|(_, v)| **v != 0)
        .map(|(&p, &v)| (p, v))
        .collect::<GridMap<i64>>()
        .map_values(&mapping, Some(' '))?
        .to_string();
    Ok(Solution::new(p1, p2))
} // 69.16ms

fn execute(start: i64, input: &str, grid: &mut GridMap<i64>) -> Result<(), Error> {
    let mut brain = Computer::new(input)?;
    let mut pos = Point2::new(0, 0);
    grid.insert(pos, start);
    let mut direction = Direction::N;
    loop {
        brain.insert(*grid.entry(pos).or_insert(0)).run()?;
        match brain.pop() {
            Some(output) => grid.insert(pos, output),
            None => break,
        };
        direction = match brain
            .pop()
            .ok_or_else(|| error!("Expected output for direction, none found"))?
        {
            0 => direction.to_left(),
            1 => direction.to_right(),
            other => bail!("Found neither 0 nor 1 for directions, but {}", other),
        };
        pos += direction.shift();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        crate::util::tests::test_full_problem(11, solve, 2319,
            "█  █ ████ ███  ███  ███  ████  ██    ██\n█  █ █    █  █ █  █ █  █ █    █  █    █\n█  █ ███  █  █ █  █ █  █ ███  █       █\n█  █ █    ███  ███  ███  █    █ ██    █\n█  █ █    █ █  █    █ █  █    █  █ █  █\n ██  ████ █  █ █    █  █ █     ███  ██ "
                .to_owned()
        );
    }
}
