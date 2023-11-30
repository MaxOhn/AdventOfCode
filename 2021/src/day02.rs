use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap().as_bytes()[0];
        let val: i64 = split.next().unwrap().parse()?;

        match dir {
            b'f' => {
                horizontal += val;
                depth += aim * val;
            }
            b'd' => aim += val,
            b'u' => aim -= val,
            _ => unreachable!(),
        }
    }

    let p1 = horizontal * aim;
    let p2 = horizontal * depth;

    Ok(Solution::new().part1(p1).part2(p2))
}
