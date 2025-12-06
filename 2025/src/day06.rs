use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let ops = parse_ops(lines.by_ref());
    let mut curr: Vec<_> = ops.iter().copied().map(Op::init).collect();

    for line in lines {
        let iter = line
            .split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .zip(ops.iter().copied())
            .zip(curr.iter_mut());

        for ((n, op), curr) in iter {
            *curr = op.apply(*curr, n);
        }
    }

    curr.into_iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let ops = parse_ops(lines.by_ref());

    let mut lines: Vec<_> = lines.collect();
    let mut col = Vec::new();

    let mut curr_i = 0;
    let mut sum = 0;

    loop {
        let max_len = lines
            .iter()
            .map(|s| s.find(' ').unwrap_or(s.len()))
            .max()
            .unwrap_or(0);

        if max_len == 0 {
            break;
        }

        col.clear();

        for line in lines.iter_mut() {
            col.push(line[..max_len].as_bytes());
            *line = line.get(max_len + 1..).unwrap_or_default();
        }

        let op = ops[curr_i];
        let mut curr = op.init();

        for i in (0..max_len).rev() {
            let mut n = 0;

            for row in col.iter() {
                let digit = row[i];

                if digit != b' ' {
                    n *= 10;
                    n += u64::from(digit - b'0');
                }
            }

            curr = op.apply(curr, n);
        }

        curr_i += 1;
        sum += curr;
    }

    sum
}

fn parse_ops<'a>(mut lines: impl DoubleEndedIterator<Item = &'a str>) -> Vec<Op> {
    lines
        .next_back()
        .into_iter()
        .flat_map(str::split_ascii_whitespace)
        .map(Op::parse)
        .collect()
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }

    fn init(self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    fn parse(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!(),
        }
    }
}
