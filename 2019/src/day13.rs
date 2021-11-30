use crate::{
    computer::Computer,
    util::{GridMap, Point2i},
    Error, Solution,
};
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve(mut input: String) -> Result<Solution<usize, i64>, Error> {
    let mut computer = Computer::new(input.clone())?;
    computer.run()?;
    let mut grid = GridMap::new();
    while let Some(x) = computer.pop() {
        let y = computer
            .pop()
            .ok_or_else(|| error!("Expected output for y, none found"))?;
        let tile = computer
            .pop()
            .ok_or_else(|| error!("Expected output for tile, none found"))?;
        grid.insert(Point2i::new(x as i32, y as i32), tile);
    }
    let p1 = grid.iter().filter(|(_, v)| **v == 2).count();
    let mut mapping = HashMap::new();
    mapping.insert(0, ' ');
    mapping.insert(1, '█');
    mapping.insert(2, 'X');
    mapping.insert(3, '-');
    mapping.insert(4, '•');
    input.replace_range(..1, "2");
    let mut computer = Computer::new(input)?;
    let mut ready_to_play = false;
    let mut p2 = 0;
    let mut ball;
    let mut paddle = 0;
    const MANUAL: bool = false; // false -> let AI play; true -> play yourself
    computer.insert(-1).run()?;
    while let Some(x) = computer.pop() {
        let y = computer
            .pop()
            .ok_or_else(|| error!("Expected output for y, none found"))?;
        let tile = computer
            .pop()
            .ok_or_else(|| error!("Expected output for tile, none found"))?;
        if x == -1 && y == 0 {
            ready_to_play = true;
            p2 = p2.max(tile);
        } else {
            grid.insert(Point2i::new(x as i32, y as i32), tile);
            if tile == 3 {
                paddle = x;
            } else if tile == 4 {
                ball = x;
                if ready_to_play {
                    if MANUAL {
                        computer
                            .insert(read_stdin(&grid.map_values(&mapping, Some(' '))?)?)
                            .run()?;
                    } else {
                        computer.insert((ball - paddle).min(1).max(-1)).run()?;
                    }
                }
            }
        }
    }
    if MANUAL {
        println!("Score: {}", p2);
        println!("Game Over");
    }
    Ok(Solution::new(p1, p2))
} // 250.96ms

fn read_stdin(grid: &GridMap<char>) -> Result<i64, Error> {
    println!("{}", grid);
    println!("Next input:");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    // 1: Left, 2: Stay, 3: Right
    match line.trim().parse::<i64>() {
        Ok(val) if 0 < val && val < 4 => Ok(val - 2),
        _ => read_stdin(grid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test13() {
        crate::util::tests::test_full_problem(13, solve, 207, 10247);
    }
}
