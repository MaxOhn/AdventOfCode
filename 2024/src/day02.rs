use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let iter = line
                .split_ascii_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            is_safe(iter)
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut buf = Vec::new();

    input
        .lines()
        .filter(|line| {
            let iter = line
                .split_ascii_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            buf.clear();
            buf.extend(iter);

            (0..buf.len()).any(|i| {
                let iter = buf
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, level)| *level);

                is_safe(iter)
            })
        })
        .count()
}

fn is_safe<I: Iterator<Item = i32>>(mut iter: I) -> bool {
    let (Some(first), Some(second)) = (iter.next(), iter.next()) else {
        return true;
    };

    let cmp = if first < second { i32::lt } else { i32::gt };

    within_range(first, second) && check_remaining(iter, second, cmp)
}

fn check_remaining<I: Iterator<Item = i32>>(
    iter: I,
    init: i32,
    cmp: fn(&i32, &i32) -> bool,
) -> bool {
    iter.scan(init, |prev, curr| {
        let is_safe = cmp(prev, &curr) && within_range(*prev, curr);
        *prev = curr;

        Some(is_safe)
    })
    .all(std::convert::identity)
}

fn within_range(a: i32, b: i32) -> bool {
    (1..=3).contains(&(a - b).abs())
}
