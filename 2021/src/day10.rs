use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut stack = Vec::new();
    let mut p1 = 0;
    let mut scores = Vec::new();

    for line in input.lines() {
        stack.clear();
        let before = p1;

        for c in line.trim_end().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => p1 += (stack.pop() != Some('(')) as usize * 3,
                ']' => p1 += (stack.pop() != Some('[')) as usize * 57,
                '}' => p1 += (stack.pop() != Some('{')) as usize * 1197,
                '>' => p1 += (stack.pop() != Some('<')) as usize * 25_137,
                _ => unreachable!("invalid {c}"),
            }
        }

        if p1 == before {
            let mut score = 0_u64;

            for &c in stack.iter().rev() {
                score *= 5;

                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }

            scores.push(score);
        }
    }

    scores.sort_unstable();
    let p2 = scores[scores.len() / 2];

    Ok(Solution::new().part1(p1).part2(p2))
}
