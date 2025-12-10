use std::collections::VecDeque;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    let mut queue = VecDeque::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let lights = parse_lights(iter.next().unwrap());
        let _joltage = iter.next_back();

        let buttons: Vec<u16> = iter
            .map(parse_button)
            .map(|button| button.iter().fold(0, |button, &i| button | (1 << i)))
            .collect();

        queue.clear();
        queue.push_back((0, 1));

        'outer: while let Some((curr, presses)) = queue.pop_front() {
            for button in buttons.iter() {
                let next = curr ^ button;

                if next == lights {
                    sum += presses;

                    break 'outer;
                }

                queue.push_back((next, presses + 1));
            }
        }
    }

    sum
}

#[cfg(target_arch = "wasm32")]
fn part2(input: &str) -> String {
    "TODO".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn part2(input: &str) -> u64 {
    use z3::{Optimize, SatResult, ast::Int};

    let mut sum = 0;

    let mut vars = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let _lights = iter.next();
        let joltage = parse_joltage(iter.next_back().unwrap());
        let buttons: Vec<_> = iter.map(parse_button).collect();

        let opt = Optimize::new();

        let button_ints: Vec<_> = (0..buttons.len() as u32)
            .map(Int::new_const)
            .inspect(|int| opt.assert(&int.ge(0)))
            .collect();

        for (i, &j) in joltage.iter().enumerate() {
            let goal = Int::from_u64(j as u64);
            vars.clear();

            for (k, button) in buttons.iter().enumerate() {
                if button.contains(&i) {
                    vars.push(button_ints[k].clone());
                }
            }

            let sum = Int::add(&vars);
            opt.assert(&sum.eq(goal));
        }

        let presses = Int::fresh_const("presses");
        opt.assert(&presses.eq(&Int::add(&button_ints)));
        opt.minimize(&presses);

        let SatResult::Sat = opt.check(&[]) else {
            unreachable!()
        };

        sum += opt
            .get_model()
            .and_then(|model| model.eval(&presses, true))
            .as_ref()
            .and_then(Int::as_u64)
            .unwrap();
    }

    sum
}

fn parse_lights(s: &str) -> u16 {
    s[1..s.len() - 1]
        .bytes()
        .rev()
        .map(|b| b == b'#')
        .fold(0, |lights, bit| (lights << 1) | bit as u16)
}

fn parse_button(s: &str) -> Box<[usize]> {
    s[1..s.len() - 1]
        .split(',')
        .map(str::parse)
        .flat_map(Result::ok)
        .collect()
}

fn parse_joltage(s: &str) -> Box<[u16]> {
    s[1..s.len() - 1]
        .split(',')
        .map(str::parse)
        .flat_map(Result::ok)
        .collect()
}
