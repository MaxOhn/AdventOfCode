use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> i64 {
    let mut curr = Vec::new();
    let mut next = Vec::new();
    let mut history = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        curr.clear();
        let iter = line.split(' ').map(|n| n.parse::<i64>().unwrap());
        curr.extend(iter);

        history.clear();
        history.push(curr.clone());

        while curr.iter().any(|&n| n != 0) {
            diffs(&curr, &mut next);
            std::mem::swap(&mut curr, &mut next);
            history.push(curr.clone());
            next.clear();
        }

        for entry in history.drain(..).rev() {
            sum += entry[entry.len() - 1];
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut curr = Vec::new();
    let mut next = Vec::new();
    let mut history = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        curr.clear();
        let iter = line.split(' ').map(|n| n.parse::<i64>().unwrap());
        curr.extend(iter);

        history.clear();
        history.push(curr.clone());

        while curr.iter().any(|&n| n != 0) {
            diffs(&curr, &mut next);
            std::mem::swap(&mut curr, &mut next);
            history.push(curr.clone());
            next.clear();
        }

        let mut diff = 0;

        for entry in history.drain(..).rev() {
            diff = entry[0] - diff;
        }

        sum += diff;
    }

    sum
}

fn diffs(from: &[i64], to: &mut Vec<i64>) {
    for window in from.windows(2) {
        to.push(window[1] - window[0])
    }
}
