use aoc_rust::Solution;
use eyre::Result;
use nom::{
    bytes::complete as by,
    character::complete as ch,
    combinator::all_consuming,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|chunk| {
            let (_, machine) = Machine::parse(chunk).unwrap_or_else(|err| panic!("{err}"));

            solve(&machine)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    const BIG: i64 = 10_000_000_000_000;

    input
        .split("\n\n")
        .map(|chunk| {
            let (_, mut machine) = Machine::parse(chunk).unwrap_or_else(|err| panic!("{err}"));
            machine.x += BIG;
            machine.y += BIG;

            solve(&machine)
        })
        .sum()
}

fn solve(machine: &Machine) -> i64 {
    let b_num = machine.x * machine.a.y - machine.y * machine.a.x;
    let b_denom = machine.b.x * machine.a.y - machine.a.x * machine.b.y;

    if b_num % b_denom != 0 {
        return 0;
    }

    let b = b_num / b_denom;

    let a_num = machine.y - b * machine.b.y;
    let a_denom = machine.a.y;

    if a_num % a_denom != 0 {
        return 0;
    }

    let a = a_num / a_denom;

    a * 3 + b
}

struct Machine {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

impl Machine {
    fn parse(chunk: &str) -> IResult<&str, Self> {
        let parse_button = |prefix| delimited(by::tag(prefix), Button::parse, ch::newline);

        let (rest, a) = parse_button("Button A: ")(chunk)?;
        let (rest, b) = parse_button("Button B: ")(rest)?;

        let (_, (x, y)) = all_consuming(preceded(
            by::tag("Prize: "),
            separated_pair(
                preceded(by::tag("X="), ch::i64),
                by::tag(", "),
                preceded(by::tag("Y="), ch::i64),
            ),
        ))(rest)?;

        Ok((rest, Self { a, b, x, y }))
    }
}

struct Button {
    x: i64,
    y: i64,
}

impl Button {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (x, y)) = separated_pair(
            preceded(by::tag("X+"), ch::i64),
            by::tag(", "),
            preceded(by::tag("Y+"), ch::i64),
        )(input)?;

        Ok((rest, Self { x, y }))
    }
}
