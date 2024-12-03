use aoc_rust::Solution;
use eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete as ch,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_mul(input: &str) -> Option<u32> {
    let res: IResult<_, _> = delimited(
        tag("mul("),
        map(separated_pair(ch::u32, tag(","), ch::u32), |(a, b)| a * b),
        tag(")"),
    )(input);

    res.ok().map(|(_, mul)| mul)
}

fn part1(input: &str) -> u32 {
    memchr::memchr_iter(b'm', input.as_bytes())
        .map(|needle| &input[needle..])
        .filter_map(parse_mul)
        .sum()
}

fn part2(input: &str) -> u32 {
    memchr::memchr2_iter(b'm', b'd', input.as_bytes())
        .map(|needle| &input[needle..])
        .scan(true, |enabled, curr| {
            Some(if curr.starts_with("don't()") {
                *enabled = false;

                None
            } else if curr.starts_with("do()") {
                *enabled = true;

                None
            } else {
                enabled.then_some(curr)
            })
        })
        .flatten()
        .filter_map(parse_mul)
        .sum()
}
