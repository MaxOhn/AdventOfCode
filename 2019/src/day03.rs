use crate::{
    util::{Direction, GridMap, Point2},
    Error, Solution,
};

use num::Signed;
use std::{cmp, convert::TryFrom};

pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let wires: Vec<Vec<(Direction, i32)>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elem| elem.split_at(1))
                .map(|(a, b)| {
                    let letter = a
                        .chars()
                        .next()
                        .ok_or_else(|| error!("Could not call next() on empty Chars"))?;
                    let direction = Direction::try_from(letter)?;
                    let num = b.parse()?;
                    Ok((direction, num))
                })
                .collect::<Result<Vec<(Direction, i32)>, Error>>()
        })
        .collect::<Result<Vec<Vec<(Direction, i32)>>, Error>>()?;
    let mut visited = GridMap::new();
    follow_wire(&wires[0], &mut visited, false);
    let (p1, p2) = follow_wire(&wires[1], &mut visited, true);
    Ok(Solution::new(p1, p2))
} // 777.47ms

fn follow_wire(wire: &[(Direction, i32)], visited: &mut GridMap<i32>, output: bool) -> (i32, i32) {
    let mut pos = Point2::new(0, 0);
    let mut path = 0;
    let mut closest_cross = i32::max_value();
    let mut shortest_cross = i32::max_value();
    for (dir, len) in wire {
        for _ in 1..=*len {
            pos += dir.shift();
            path += 1;
            if output {
                if visited.contains_key(&pos) {
                    closest_cross = cmp::min(closest_cross, pos.abs().sum());
                    shortest_cross = cmp::min(shortest_cross, path + visited[&pos]);
                }
            } else {
                visited.entry(pos).or_insert(path);
            }
        }
    }
    (closest_cross, shortest_cross)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test03() {
        assert_eq!(
            solve(String::from("R8,U5,L5,D3\nU7,R6,D4,L4")).unwrap(),
            Solution::new(6, 30)
        );
        assert_eq!(
            solve(String::from(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ))
            .unwrap(),
            Solution::new(159, 610)
        );
        assert_eq!(
            solve(String::from(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ))
            .unwrap(),
            Solution::new(135, 410)
        );
        crate::util::tests::test_full_problem(3, solve, 855, 11_238);
    }
}
