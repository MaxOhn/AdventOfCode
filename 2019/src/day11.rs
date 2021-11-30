use crate::{
    computer::Computer,
    util::{Direction, GridMap, Point2},
    Error, Solution,
};
use std::collections::HashMap;

pub fn solve(input: String) -> Result<Solution<usize, String>, Error> {
    let mut grid = GridMap::new();
    run(0, input.clone(), &mut grid)?;
    let p1 = grid.len();
    grid.clear();
    run(1, input, &mut grid)?;
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

fn run(start: i64, input: String, grid: &mut GridMap<i64>) -> Result<(), Error> {
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
