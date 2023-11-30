use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut a: u16 = lines.next().unwrap().parse()?;
    let mut b: u16 = lines.next().unwrap().parse()?;

    p1 += (b > a) as usize;

    let mut c: u16 = lines.next().unwrap().parse()?;

    p1 += (c > b) as usize;

    for line in lines {
        let curr = line.parse()?;
        p1 += (curr > c) as usize;
        p2 += (curr > a) as usize;
        a = b;
        b = c;
        c = curr;
    }

    Ok(Solution::new().part1(p1).part2(p2))
}
