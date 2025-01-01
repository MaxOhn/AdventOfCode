#![allow(clippy::map_entry)]
use crate::{
    util::{Direction, GridMap, Point2i},
    Error, Solution,
};
use pathfinding::directed::astar::astar;
use pathfinding::directed::bfs::bfs;
use std::collections::HashMap;

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<usize, usize>, Error> {
    let (maze, start, end) = parse_maze(input)?;
    let p1 = solve_part1(&maze, start, end)?;
    let p2 = solve_part2(&maze, start, end)?;
    Ok(Solution::new(p1, p2))
} // 2.86s

fn solve_part1(maze: &GridMap<Cell>, start: Point2i, end: Point2i) -> Result<usize, Error> {
    let path = bfs(
        &maze[&start],
        |Cell { pos, portal, .. }| {
            let mut neighbors = Vec::with_capacity(5);
            for direction in Direction::iter() {
                let neighbor = *pos + direction.shift();
                if let Some(Cell { passage, .. }) = maze.get(&neighbor) {
                    if *passage {
                        neighbors.push(maze[&neighbor]);
                    }
                }
            }
            if let Some((portal, _)) = portal {
                neighbors.push(maze[&portal]);
            }
            neighbors
        },
        |Cell { pos, .. }| pos == &end,
    )
    .ok_or_else(|| error!("No path found for part1"))?;
    Ok(path.len() - 1)
}

fn solve_part2(maze: &GridMap<Cell>, start: Point2i, end: Point2i) -> Result<usize, Error> {
    // Using A* (same interface as BFS but with heuristic and costs between cells)
    let h = maze.get_height();
    let path = astar(
        &maze[&start],
        |Cell {
             pos, portal, level, ..
         }| {
            let mut neighbors = Vec::with_capacity(5);
            for direction in Direction::iter() {
                let neighbor = *pos + direction.shift();
                if let Some(Cell { passage, .. }) = maze.get(&neighbor) {
                    if *passage {
                        let mut n_cell = maze[&neighbor];
                        n_cell.level = *level;
                        neighbors.push((n_cell, 1));
                    }
                }
            }
            if let Some((portal, outer)) = portal {
                if *level != 0 || !outer {
                    let mut portal_cell = maze[&portal];
                    if *outer {
                        portal_cell.level = level - 1;
                    } else {
                        portal_cell.level = level + 1;
                    }
                    neighbors.push((portal_cell, 1));
                }
            }
            neighbors
        },
        |Cell { level, .. }| *level * h,
        |Cell { pos, level, .. }| *level == 0 && *pos == end,
    )
    .ok_or_else(|| error!("No path found for part2"))?;
    Ok(path.1)
}

fn parse_maze(input: &str) -> Result<(GridMap<Cell>, Point2i, Point2i), Error> {
    let mut maze_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let h = maze_matrix.len();
    let w = maze_matrix[0].len();

    let last_row = maze_matrix.last_mut().unwrap();
    if last_row.len() < w {
        last_row.resize(w, ' ');
    }

    let mut maze: GridMap<Cell> = GridMap::new();
    let mut portals: HashMap<String, (Point2i, bool)> = HashMap::new();
    let mut start = None;
    let mut end = None;
    for y in 0..h {
        for x in 0..w {
            match maze_matrix[y][x] {
                ' ' => continue,
                '#' => {
                    let pos = Point2i::new(x as i32 - 2, y as i32 - 2);
                    maze.insert(pos, Cell::new(pos, false, None, 0));
                }
                '.' => {
                    let pos = Point2i::new(x as i32 - 2, y as i32 - 2);
                    maze.entry(pos)
                        .or_insert_with(|| Cell::new(pos, true, None, 0));
                }
                letter if letter.is_ascii_uppercase() => {
                    for (nx, ny) in get_successors(x, y, w, h) {
                        if maze_matrix[ny][nx].is_ascii_uppercase() {
                            let name = format!("{}{}", letter, maze_matrix[ny][nx]);
                            let pos = {
                                if x == nx {
                                    if ny + 1 < h && maze_matrix[ny + 1][nx] == '.' {
                                        Point2i::new(x as i32 - 2, ny as i32 - 1)
                                    } else if y > 0 && maze_matrix[y - 1][nx] == '.' {
                                        Point2i::new(x as i32 - 2, y as i32 - 3)
                                    } else {
                                        bail!("x == nx but could not find '.' on either end");
                                    }
                                } else {
                                    // y == ny
                                    if nx + 1 < w && maze_matrix[ny][nx + 1] == '.' {
                                        Point2i::new(nx as i32 - 1, y as i32 - 2)
                                    } else if x > 0 && maze_matrix[ny][x - 1] == '.' {
                                        Point2i::new(x as i32 - 3, y as i32 - 2)
                                    } else {
                                        bail!("y == ny but could not find '.' on either end");
                                    }
                                }
                            };
                            if name == "AA" {
                                start = Some(pos);
                            } else if name == "ZZ" {
                                end = Some(pos);
                            } else if let Some((first_pos, first_outer)) = portals.get(&name) {
                                maze.entry(pos)
                                    .and_modify(|cell| {
                                        cell.portal = Some((*first_pos, !first_outer))
                                    })
                                    .or_insert_with(|| {
                                        Cell::new(pos, true, Some((*first_pos, !first_outer)), 0)
                                    });
                                maze.entry(portals[&name].0)
                                    .and_modify(|cell| cell.portal = Some((pos, *first_outer)))
                                    .or_insert_with(|| {
                                        Cell::new(
                                            portals[&name].0,
                                            true,
                                            Some((pos, *first_outer)),
                                            0,
                                        )
                                    });
                            } else {
                                let outer = x == 0 || y == 0 || nx == w - 1 || ny == h - 1;
                                portals.insert(name, (pos, outer));
                            }
                            break;
                        }
                    }
                }
                other => bail!("Can't parse symbol {} from input into maze", other),
            }
        }
    }
    Ok((maze, start.unwrap(), end.unwrap()))
}

fn get_successors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut successors = Vec::with_capacity(2);
    if x < w - 1 {
        successors.push((x + 1, y));
    }
    if y < h - 1 {
        successors.push((x, y + 1));
    }
    successors
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Cell {
    pos: Point2i,
    passage: bool,
    portal: Option<(Point2i, bool)>,
    level: usize,
}

impl Cell {
    fn new(pos: Point2i, passage: bool, portal: Option<(Point2i, bool)>, level: usize) -> Self {
        Self {
            pos,
            passage,
            portal,
            level,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            pos: Point2i::default(),
            passage: false,
            portal: None,
            level: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test20() {
        let input =
            "         A           \n         A           \n  #######.#########  \n  #######.........#  \n  #######.#######.#  \n  #######.#######.#  \n  #######.#######.#  \n  #####  B    ###.#  \nBC...##  C    ###.#  \n  ##.##       ###.#  \n  ##...DE  F  ###.#  \n  #####    G  ###.#  \n  #########.#####.#  \nDE..#######...###.#  \n  #.#########.###.#  \nFG..#########.....#  \n  ###########.#####  \n             Z       \n             Z       ";
        let solution = solve(input).unwrap();
        assert_eq!(solution.part1, 23);
        assert_eq!(solution.part2, 26);
        let input =
            "             Z L X W       C                 \n             Z P Q B       K                 \n  ###########.#.#.#.#######.###############  \n  #...#.......#.#.......#.#.......#.#.#...#  \n  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n  #.###.#######.###.###.#.###.###.#.#######  \n  #...#.......#.#...#...#.............#...#  \n  #.#########.#######.#.#######.#######.###  \n  #...#.#    F       R I       Z    #.#.#.#  \n  #.###.#    D       E C       H    #.#.#.#  \n  #.#...#                           #...#.#  \n  #.###.#                           #.###.#  \n  #.#....OA                       WB..#.#..ZH\n  #.###.#                           #.#.#.#  \nCJ......#                           #.....#  \n  #######                           #######  \n  #.#....CK                         #......IC\n  #.###.#                           #.###.#  \n  #.....#                           #...#.#  \n  ###.###                           #.#.#.#  \nXF....#.#                         RF..#.#.#  \n  #####.#                           #######  \n  #......CJ                       NM..#...#  \n  ###.#.#                           #.###.#  \nRE....#.#                           #......RF\n  ###.###        X   X       L      #.#.#.#  \n  #.....#        F   Q       P      #.#.#.#  \n  ###.###########.###.#######.#########.###  \n  #.....#...#.....#.......#...#.....#.#...#  \n  #####.#.###.#######.#######.###.###.#.#.#  \n  #.......#.......#.#.#.#.#...#...#...#.#.#  \n  #####.###.#####.#.#.#.#.###.###.#.###.###  \n  #.......#.....#.#...#...............#...#  \n  #############.#.#.###.###################  \n               A O F   N                     \n               A A D   M                     ";
        let solution = solve(input).unwrap();
        assert_eq!(solution.part1, 77);
        assert_eq!(solution.part2, 396);
        crate::util::tests::test_full_problem(20, solve, 686, 8384);
    }
}
