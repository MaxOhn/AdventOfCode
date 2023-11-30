use aoc_rust::Solution;
use eyre::Result;

use crate::util::Parse;

pub fn run(input: &str) -> Result<Solution> {
    let values = parse_values(input)?;
    let conditions = calculate_conditions(&values);

    let p1 = part1(&conditions);
    let p2 = part2(&conditions);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_values(input: &str) -> Result<Vec<(i64, i64)>> {
    let mut lines = input.lines();

    let mut values = Vec::with_capacity(14);

    for _ in 0..14 {
        for _ in 0..5 {
            lines.next();
        }

        let add_x: i64 = lines.next().unwrap().trim_end().as_bytes()[6..].parse();

        for _ in 0..9 {
            lines.next();
        }

        let add_y: i64 = lines.next().unwrap().trim_end().as_bytes()[6..].parse();

        for _ in 0..2 {
            lines.next();
        }

        values.push((add_x, add_y));
    }

    Ok(values)
}

fn calculate_conditions(values: &[(i64, i64)]) -> Vec<(u32, u32, i64)> {
    let mut z = Vec::new();
    let mut conditions = Vec::new();

    for (w, (add_x, add_y)) in values.into_iter().enumerate() {
        if *add_x > 0 {
            z.push((w, add_y));
        } else if let Some((r, v)) = z.pop() {
            conditions.push((w as u32, r as u32, v + add_x));
        }
    }

    conditions
}

fn part1(conditions: &[(u32, u32, i64)]) -> i64 {
    let mut answer = 99_999_999_999_999;

    for &(w, r, offset) in conditions {
        let (sign, shift) = if offset > 0 { (-1, r) } else { (1, w) };
        answer += sign * offset * 10_i64.pow(13 - shift);
    }

    answer
}

fn part2(conditions: &[(u32, u32, i64)]) -> i64 {
    let mut answer = 11_111_111_111_111;

    for &(w, r, offset) in conditions {
        let (sign, shift) = if offset > 0 { (1, w) } else { (-1, r) };
        answer += sign * offset * 10_i64.pow(13 - shift);
    }

    answer
}
