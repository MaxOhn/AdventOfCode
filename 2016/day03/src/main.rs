use std::convert::identity;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let start = std::time::Instant::now();
    let p1 = part1(&input);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = std::time::Instant::now();
    let p2 = part2(&input);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 983);
    assert_eq!(p2, 1836);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Triangle::valid)
        .count()
}

fn part2(input: &str) -> u16 {
    input
        .lines()
        .scan(
            (
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
            ),
            |(x, y, z), line| {
                let mut nums = line
                    .split_ascii_whitespace()
                    .map(|num| num.chars().fold(0, |n, c| n * 10 + (c as u16 & 0xF)));

                let a1 = nums.next()?;
                let a2 = nums.next()?;
                let a3 = nums.next()?;

                if x.len() == 2 {
                    let mut drain = x.drain(..);
                    let b = drain.next()?;
                    let c = drain.next()?;
                    let mut count = Triangle { a: a1, b, c }.valid() as u16;

                    let mut drain = y.drain(..);
                    let b = drain.next()?;
                    let c = drain.next()?;
                    count += Triangle { a: a2, b, c }.valid() as u16;

                    let mut drain = z.drain(..);
                    let b = drain.next()?;
                    let c = drain.next()?;
                    count += Triangle { a: a3, b, c }.valid() as u16;

                    Some(Some(count))
                } else {
                    x.push(a1);
                    y.push(a2);
                    z.push(a3);

                    Some(None)
                }
            },
        )
        .filter_map(identity)
        .sum()
}

struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl Triangle {
    fn valid(&self) -> bool {
        2 * self.a.max(self.b).max(self.c) < self.a + self.b + self.c
    }
}

impl FromStr for Triangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split_ascii_whitespace()
            .map(|num| num.chars().fold(0, |n, c| n * 10 + (c as u16 & 0xF)));

        let a = nums.next().ok_or(())?;
        let b = nums.next().ok_or(())?;
        let c = nums.next().ok_or(())?;

        Ok(Self { a, b, c })
    }
}
