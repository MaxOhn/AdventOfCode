use std::{
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let (workflows, mut ratings) = parse_input(input.trim())?;

    let p1 = part1(&workflows, &mut ratings)?;
    let p2 = part2(&workflows)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1<'a>(workflows: &'a [WorkFlow<'a>], ratings: &mut [RatingSet<'a>]) -> Result<i32> {
    let mut sum = 0;

    for set in ratings.iter_mut() {
        loop {
            let workflow = workflows
                .iter()
                .find(|workflow| workflow.name == set.workflow)
                .wrap_err("missing workflow")?;

            let rule = workflow
                .rules
                .iter()
                .find(|rule| {
                    let Some(ref condition) = rule.condition else {
                        return true;
                    };

                    match condition.op {
                        Op::Greater => set.ratings.iter().any(|rating| {
                            rating.part == condition.part && rating.value > condition.value
                        }),
                        Op::Less => set.ratings.iter().any(|rating| {
                            rating.part == condition.part && rating.value < condition.value
                        }),
                    }
                })
                .wrap_err("missing rule")?;

            match rule.send_to {
                "R" => break,
                "A" => {
                    for rating in set.ratings.iter() {
                        sum += rating.value as i32;
                    }

                    break;
                }
                name => {
                    set.workflow = name;
                    continue;
                }
            }
        }
    }

    Ok(sum)
}

fn part2(workflows: &[WorkFlow]) -> Result<u64> {
    let mut stack = vec![("in", 0, Ranges::default())];
    let mut accepted = 0;

    while let Some((curr, idx, ranges)) = stack.pop() {
        if curr == "A" {
            accepted += ranges.product();
            continue;
        } else if curr == "R" {
            continue;
        }

        let rule = workflows
            .iter()
            .find_map(|workflow| (workflow.name == curr).then(|| &workflow.rules[idx]))
            .wrap_err("missing workflow")?;

        let next = rule.send_to;

        let Some(ref condition) = rule.condition else {
            stack.push((next, 0, ranges));
            continue;
        };

        let part = condition.part;

        match condition.op {
            Op::Greater => {
                if ranges[part].end <= condition.value {
                    stack.push((curr, idx + 1, ranges));
                } else if ranges[part].start > condition.value {
                    stack.push((next, 0, ranges));
                } else {
                    let mut else_ranges = ranges.clone();
                    else_ranges[part].end = condition.value + 1;
                    stack.push((curr, idx + 1, else_ranges));

                    let mut match_ranges = ranges.clone();
                    match_ranges[part].start = condition.value + 1;
                    stack.push((next, 0, match_ranges));
                }
            }
            Op::Less => {
                if ranges[part].start >= condition.value {
                    stack.push((curr, idx + 1, ranges));
                } else if ranges[part].end < condition.value {
                    stack.push((next, 0, ranges));
                } else {
                    let mut else_ranges = ranges.clone();
                    else_ranges[part].start = condition.value;
                    stack.push((curr, idx + 1, else_ranges));

                    let mut match_ranges = ranges.clone();
                    match_ranges[part].end = condition.value;
                    stack.push((next, 0, match_ranges))
                }
            }
        }
    }

    Ok(accepted)
}

#[derive(Clone, Debug)]
struct Ranges {
    parts: [Range<i16>; 4],
}

impl Ranges {
    fn product(&self) -> u64 {
        self.parts.iter().map(|range| range.len() as u64).product()
    }
}

impl Index<Part> for Ranges {
    type Output = Range<i16>;

    fn index(&self, part: Part) -> &Self::Output {
        self.parts.index(part as usize)
    }
}

impl IndexMut<Part> for Ranges {
    fn index_mut(&mut self, part: Part) -> &mut Self::Output {
        self.parts.index_mut(part as usize)
    }
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            parts: [0_u8; 4].map(|_| 1..4001),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<WorkFlow>, Vec<RatingSet>)> {
    let (workflows, ratings) = input
        .split_once("\n\n")
        .wrap_err("missing double newline")?;

    let workflows: Vec<_> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line
                .trim_end_matches('}')
                .split_once('{')
                .wrap_err("missing parentheses")?;

            let rules = rules
                .split(',')
                .map(|rule| {
                    let Some((condition, send_to)) = rule.split_once(':') else {
                        return Ok(Rule {
                            condition: None,
                            send_to: rule,
                        });
                    };

                    Ok(Rule {
                        condition: Some(condition.parse()?),
                        send_to,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(WorkFlow { name, rules })
        })
        .collect::<Result<_>>()?;

    let ratings = ratings
        .lines()
        .map(|line| {
            let mut ratings = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',');

            let ratings = [
                ratings
                    .next()
                    .map(Rating::from_str)
                    .wrap_err("missing rating")??,
                ratings
                    .next()
                    .map(Rating::from_str)
                    .wrap_err("missing rating")??,
                ratings
                    .next()
                    .map(Rating::from_str)
                    .wrap_err("missing rating")??,
                ratings
                    .next()
                    .map(Rating::from_str)
                    .wrap_err("missing rating")??,
            ];

            Ok(RatingSet {
                workflow: "in",
                ratings,
            })
        })
        .collect::<Result<_>>()?;

    Ok((workflows, ratings))
}

struct RatingSet<'a> {
    workflow: &'a str,
    ratings: [Rating; 4],
}

struct Rating {
    part: Part,
    value: i16,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Part {
    X,
    M,
    A,
    S,
}

impl FromStr for Part {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => eyre::bail!("invalid part `{s}`"),
        }
    }
}

impl FromStr for Rating {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part, value) = s.split_once('=').wrap_err("missing equal")?;

        Ok(Self {
            part: part.parse()?,
            value: value.parse().wrap_err("invalid value")?,
        })
    }
}

struct WorkFlow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

struct Rule<'a> {
    condition: Option<Condition>,
    send_to: &'a str,
}

struct Condition {
    part: Part,
    op: Op,
    value: i16,
}

impl FromStr for Condition {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, op, right) = match s.split_once('>') {
            Some((left, right)) => (left, Op::Greater, right),
            None => {
                let (left, right) = s.split_once('<').wrap_err("missing op")?;

                (left, Op::Less, right)
            }
        };

        Ok(Self {
            part: left.parse()?,
            op,
            value: right.parse().wrap_err("invalid value")?,
        })
    }
}

#[derive(Copy, Clone)]
enum Op {
    Greater,
    Less,
}
