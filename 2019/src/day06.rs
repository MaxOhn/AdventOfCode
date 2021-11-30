use crate::{Error, Solution};

use std::collections::{HashMap, HashSet, VecDeque};

type Relations = HashMap<usize, Vec<usize>>;

pub fn solve(input: String) -> Result<Solution<usize, usize>, Error> {
    let (ids, directed, undirected) = prepare_maps(input)?;
    let p1 = solve_part1(
        &directed,
        *ids.get("COM")
            .ok_or_else(|| error!("Could not find key 'COM' in ids"))?,
    );
    let p2 = solve_part2(
        &undirected,
        *ids.get("YOU")
            .ok_or_else(|| error!("Could not find key 'YOU' in ids"))?,
        *ids.get("SAN")
            .ok_or_else(|| error!("Could not find key 'SAN' in ids"))?,
    )?;
    Ok(Solution::new(p1, p2))
} // 47.8ms

fn prepare_maps(input: String) -> Result<(HashMap<String, usize>, Relations, Relations), Error> {
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut directed: Relations = Relations::new();
    let mut undirected: Relations = Relations::new();
    let mut id = 0;
    for line in input.lines() {
        let mut line_iter = line.split(')');
        let center_name = String::from(
            line_iter
                .next()
                .ok_or_else(|| error!("Could not find name of center"))?,
        );
        let center = *ids.entry(center_name).or_insert_with(|| {
            id += 1;
            id
        });
        let orbiter_name = String::from(
            line_iter
                .next()
                .ok_or_else(|| error!("Could not find name of orbiter"))?,
        );
        let orbiter = *ids.entry(orbiter_name).or_insert_with(|| {
            id += 1;
            id
        });
        directed
            .entry(center)
            .or_insert_with(Vec::new)
            .push(orbiter);
        undirected
            .entry(center)
            .or_insert_with(Vec::new)
            .push(orbiter);
        undirected
            .entry(orbiter)
            .or_insert_with(Vec::new)
            .push(center);
    }
    Ok((ids, directed, undirected))
}

fn solve_part1(map: &Relations, start: usize) -> usize {
    let mut sum = 0;
    let mut depths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    depths.insert(start, 0);
    queue.push_back(start);
    visited.insert(start);

    while let Some(ref center) = queue.pop_front() {
        if let Some(orbiters) = map.get(center) {
            for &orbiter in orbiters {
                if !visited.contains(&orbiter) {
                    let depth = depths.get(center).unwrap() + 1;
                    depths.insert(orbiter, depth);
                    visited.insert(orbiter);
                    sum += depth;
                    queue.push_back(orbiter);
                }
            }
        }
    }
    sum
}

fn solve_part2(map: &Relations, start: usize, end: usize) -> Result<usize, Error> {
    let mut depths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    depths.insert(start, 0usize);
    queue.push_back(start);
    visited.insert(start);

    while let Some(ref center) = queue.pop_front() {
        if let Some(orbiters) = map.get(center) {
            for &orbiter in orbiters {
                if !visited.contains(&orbiter) {
                    let depth = depths.get(center).unwrap() + 1;
                    if orbiter == end {
                        return Ok(depth - 2);
                    }
                    visited.insert(orbiter);
                    depths.insert(orbiter, depth);
                    queue.push_back(orbiter);
                }
            }
        }
    }
    bail!("No path from 'YOU' to 'SAN' found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test06() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_owned();
        let (ids, map, _) = prepare_maps(input).unwrap();
        assert_eq!(solve_part1(&map, *ids.get("COM").unwrap()), 42);
        let input =
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_owned();
        assert_eq!(solve(input).unwrap(), Solution::new(54, 4));
        crate::util::tests::test_full_problem(6, solve, 453028, 562);
    }
}
