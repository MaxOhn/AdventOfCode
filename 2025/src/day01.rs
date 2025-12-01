use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut curr = 50;
    let mut count = 0;

    for line in input.lines() {
        let (a, b) = line.split_at(1);
        let n: i32 = b.parse().unwrap();

        match a {
            "L" => curr -= n,
            "R" => curr += n,
            _ => unreachable!(),
        }

        curr = curr.rem_euclid(100);

        if curr == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> i32 {
    let mut curr = 50;
    let mut count = 0;

    for line in input.lines() {
        let (a, b) = line.split_at(1);
        let mut n: i32 = b.parse().unwrap();

        count += n / 100;
        n %= 100;

        let next = match a {
            "L" => curr - n,
            "R" => curr + n,
            _ => unreachable!(),
        };

        if (next <= 0 && curr != 0) || next >= 100 {
            count += 1;
        }

        curr = next.rem_euclid(100);
    }

    count
}
