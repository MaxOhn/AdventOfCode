use std::cmp;

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxHashMap;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let lights = parse_lights(iter.next().unwrap());
        let _joltage = iter.next_back();

        let buttons: Vec<u16> = iter
            .map(parse_button)
            .map(|button| button.iter().fold(0, |button, &i| button | (1 << i)))
            .collect();

        sum += solve_parity(lights, &buttons).unwrap();
    }

    sum
}

type Cache<'a> = FxHashMap<(u16, &'a [u16]), Option<usize>>;

fn solve_parity(lights: u16, buttons: &[u16]) -> Option<usize> {
    fn recurse_outer<'a>(
        lights: u16,
        button: u16,
        rest: &'a [u16],
        cache: &mut Cache<'a>,
    ) -> Option<usize> {
        let with = recurse_inner(lights ^ button, rest, 1, cache);
        let without = recurse_inner(lights, rest, 0, cache);

        match (with, without) {
            (Some(with), Some(without)) => Some(cmp::min(with, without)),
            (None, Some(count)) | (Some(count), None) => Some(count),
            (None, None) => None,
        }
    }

    fn recurse_inner<'a>(
        lights: u16,
        buttons: &'a [u16],
        count: usize,
        cache: &mut Cache<'a>,
    ) -> Option<usize> {
        let Some((button, rest)) = buttons.split_first() else {
            return (lights == 0x00_00).then_some(count);
        };

        let key = (lights, buttons);

        if let Some(cached) = cache.get(&key) {
            return cached.map(|n| n + count);
        }

        let res = recurse_outer(lights, *button, rest, cache);
        cache.insert(key, res);

        res.map(|n| n + count)
    }

    let (button, rest) = buttons.split_first()?;

    let mut cache = Cache::default();

    recurse_outer(lights, *button, rest, &mut cache)
}

#[cfg(target_arch = "wasm32")]
fn part2(input: &str) -> String {
    "Cannot use z3 on WASM :(".to_owned()
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
