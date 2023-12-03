use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> i32 {
    input.bytes().fold(0, |floor, byte| match byte {
        b'(' => floor + 1,
        b')' => floor - 1,
        _ => floor,
    })
}

fn part2(input: &str) -> usize {
    let mut floor = 0;

    input
        .bytes()
        .position(|byte| {
            match byte {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => {}
            }

            floor < 0
        })
        .map(|i| i + 1)
        .unwrap_or(usize::MAX)
}
