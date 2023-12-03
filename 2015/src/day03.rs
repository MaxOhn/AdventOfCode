use aoc_rust::Solution;
use eyre::Result;
use hashbrown::HashSet;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();

    let mut pos = Pos::default();
    visited.insert(pos);

    for byte in input.trim().bytes() {
        match byte {
            b'<' => pos.x -= 1,
            b'>' => pos.x += 1,
            b'^' => pos.y -= 1,
            b'v' => pos.y += 1,
            _ => {}
        }

        visited.insert(pos);
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();

    let mut santa = Pos::default();
    let mut robo = Pos::default();

    visited.insert(santa);

    for (i, byte) in input.trim().bytes().enumerate() {
        let is_santa = i % 2 == 0;

        let pos = if is_santa { &mut santa } else { &mut robo };

        match byte {
            b'<' => pos.x -= 1,
            b'>' => pos.x += 1,
            b'^' => pos.y -= 1,
            b'v' => pos.y += 1,
            _ => {}
        }

        visited.insert(*pos);
    }

    visited.len()
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
