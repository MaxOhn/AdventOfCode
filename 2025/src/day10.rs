use std::{cmp, ops::BitXor};

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

fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let _lights = iter.next();
        let joltage = parse_joltage(iter.next_back().unwrap());

        let buttons: Vec<u16> = iter
            .map(parse_button)
            .map(|button| button.iter().fold(0, |button, &i| button | (1 << i)))
            .collect();

        sum += f(&joltage, &buttons).unwrap();
    }

    sum
}

// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn f(joltage: &[u16], buttons: &[u16]) -> Option<usize> {
    /// Passes each combination of buttons in `rest` to `f` together with the
    /// current `best`.
    fn recurse_button_combis<F>(
        rest: &[u16],
        buttons: &mut Vec<u16>,
        best: &mut Option<usize>,
        f: F,
    ) where
        F: Fn(&[u16], &mut Option<usize>) + Copy,
    {
        let Some((button, rest)) = rest.split_first() else {
            return f(buttons, best);
        };

        recurse_button_combis(rest, buttons, best, f);

        buttons.push(*button);
        recurse_button_combis(rest, buttons, best, f);
        buttons.pop();
    }

    if joltage.iter().all(|&n| n == 0) {
        return Some(0);
    }

    // Converting joltage to binary representation, i.e. "lights" of part 1
    let lights = joltage
        .iter()
        .copied()
        .rev()
        .fold(0, |lights, next| (lights << 1) | (next % 2));

    let mut best = None;

    recurse_button_combis(
        &buttons,
        &mut Vec::with_capacity(buttons.len()),
        &mut best,
        |pressed, best| {
            let xor = pressed.iter().copied().reduce(u16::bitxor).unwrap_or(0);

            if xor != lights {
                return;
            }

            // Subtracting the `pressed` buttons from `joltage`
            let mut next_joltage = Box::<[_]>::from(joltage);

            for button in pressed {
                let mut button = *button;
                let mut i = 0;

                while button > 0 {
                    if button & 1 == 1 {
                        if next_joltage[i] == 0 {
                            return;
                        }

                        next_joltage[i] -= 1;
                    }

                    i += 1;
                    button >>= 1;
                }
            }

            // Halving the next joltage
            next_joltage.iter_mut().for_each(|j| *j /= 2);

            let Some(res) = f(&next_joltage, buttons) else {
                return;
            };

            let curr = pressed.len() + 2 * res;
            let best = best.get_or_insert(usize::MAX);

            if curr < *best {
                *best = curr;
            }
        },
    );

    best
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
