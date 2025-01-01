use crate::{
    computer::Computer,
    util::{Direction, GridMap, Point2i},
    Error, Solution,
};

use std::collections::HashSet;

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<i32, i32>, Error> {
    let mut computer = Computer::new(input)?;
    let mut curr_pos = Point2i::new(0, 0);
    let mut curr_dir = Direction::N;
    // 0: Wall; 1: Path; 2: Blocked; 3: Oxygenated
    let mut grid = GridMap::new();
    grid.insert(curr_pos, 1u8);
    let mut oxy_pos = Point2i::default();
    let mut p1 = 0;
    let p2;
    loop {
        match curr_dir {
            Direction::N => computer.insert(1),
            Direction::S => computer.insert(2),
            Direction::W => computer.insert(3),
            Direction::E => computer.insert(4),
        };
        match computer
            .run()?
            .pop()
            .ok_or_else(|| error!("Expected output for terrain status, none found"))?
        {
            0 => {
                grid.insert(curr_pos + curr_dir.shift(), 0);
            }
            1 => {
                curr_pos += curr_dir.shift();
                grid.insert(curr_pos, 1);
            }
            2 => {
                curr_pos += curr_dir.shift();
                oxy_pos = curr_pos;
                grid.insert(curr_pos, 4);
            }
            _ => unreachable!(),
        }
        let mut found = false;
        // Find unvisited neighbor
        for &dir in Direction::iter() {
            let next_pos = curr_pos + dir.shift();
            if !grid.contains_key(&next_pos) {
                curr_dir = dir;
                found = true;
                break;
            }
        }
        // Find unblocked neighbor
        if !found {
            grid.insert(curr_pos, 2);
            for &dir in Direction::iter() {
                let next_pos = curr_pos + dir.shift();
                if grid[&next_pos] == 1 {
                    curr_dir = dir;
                    found = true;
                    break;
                }
            }
        }
        // Grid fully discovered, start DFS to flood
        if !found {
            /*
            let mut mapping = std::collections::HashMap::new();
            mapping.insert(0, '█');
            mapping.insert(2, ' ');
            mapping.insert(5, 'O');
            let draw_grid = grid.map_values(&mapping, Some(' '));
            println!("Grid:\n{}", draw_grid);
            */
            // Part 1
            let pos = Point2i::new(0, 0);
            let mut visited = HashSet::new();
            visited.insert(pos);
            let mut backtrack = vec![(pos, 0)];
            while let Some((pos, dist)) = backtrack.pop() {
                for dir in Direction::iter() {
                    let next_pos = pos + dir.shift();
                    if grid[&next_pos] != 0 && !visited.contains(&next_pos) {
                        if next_pos == oxy_pos {
                            p1 = dist + 1;
                            backtrack.clear();
                            break;
                        } else {
                            visited.insert(next_pos);
                            backtrack.push((next_pos, dist + 1));
                        }
                    }
                }
            }
            // Part 2
            visited.clear();
            visited.insert(oxy_pos);
            let mut backtrack = vec![(oxy_pos, 0)];
            let mut minutes = 0;
            while let Some((pos, m)) = backtrack.pop() {
                minutes = minutes.max(m);
                grid.insert(pos, 3);
                for dir in Direction::iter() {
                    let next_pos = pos + dir.shift();
                    if grid[&next_pos] != 0 && !visited.contains(&next_pos) {
                        visited.insert(next_pos);
                        backtrack.push((next_pos, m + 1));
                    }
                }
            }
            p2 = minutes;
            break;
        }
    }
    Ok(Solution::new(p1, p2))
} // 59.31ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test15() {
        crate::util::tests::test_full_problem(15, solve, 272, 398);
    }
}
