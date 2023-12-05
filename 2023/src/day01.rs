use std::str::Chars;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    fn make_iter(line: &str) -> Chars<'_> {
        line.chars()
    }

    let find_digit = |c: char| c.to_digit(10);

    solve(input, make_iter, find_digit)
}

fn part2(input: &str) -> u32 {
    fn make_iter(line: &str) -> impl DoubleEndedIterator<Item = &str> {
        (0..line.len()).map(|i| &line[i..])
    }

    let find_digit = |s: &str| {
        static KV_MAP: &[(u8, &str)] = &[
            (b'0', "zero"),
            (b'1', "one"),
            (b'2', "two"),
            (b'3', "three"),
            (b'4', "four"),
            (b'5', "five"),
            (b'6', "six"),
            (b'7', "seven"),
            (b'8', "eight"),
            (b'9', "nine"),
        ];

        KV_MAP.iter().find_map(|(digit, word)| {
            (s.starts_with(*digit as char) || s.starts_with(word)).then_some((*digit - b'0') as u32)
        })
    };

    solve(input, make_iter, find_digit)
}

fn solve<'i, M, I, T, F>(input: &'i str, make_iter: M, find_digit: F) -> u32
where
    M: Fn(&'i str) -> I,
    I: DoubleEndedIterator<Item = T>,
    F: Copy + Fn(T) -> Option<u32>,
{
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = make_iter(line);

        let Some(first) = iter.find_map(find_digit) else {
            continue;
        };

        let second = iter.rev().find_map(find_digit).unwrap_or(first);

        sum += first * 10 + second;
    }

    sum
}
