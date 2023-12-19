#![allow(unused)]

use std::{
    cmp,
    collections::HashMap,
    ops::{Index, IndexMut, Range, RangeInclusive},
};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> i32 {
    let (workflows, mut ratings) = parse_input(input);

    for set in ratings.iter_mut() {
        loop {
            let workflow = workflows
                .iter()
                .find(|workflow| workflow.name == set.workflow)
                .unwrap_or_else(|| panic!("search for {}", set.workflow));

            let rule = workflow
                .rules
                .iter()
                .find(|rule| {
                    let Some(ref condition) = rule.condition else {
                        return true;
                    };

                    let rating = match condition.op {
                        Op::Greater => set.ratings.iter().find(|rating| {
                            rating.name == condition.left && rating.value > condition.right
                        }),
                        Op::Less => set.ratings.iter().find(|rating| {
                            rating.name == condition.left && rating.value < condition.right
                        }),
                    };

                    rating.is_some()
                })
                .unwrap();

            match rule.send_to {
                "R" => set.workflow = "R",
                "A" => set.workflow = "A",
                name => {
                    set.workflow = name;
                    continue;
                }
            }

            break;
        }
    }

    let mut sum = 0;

    for set in ratings {
        if set.workflow == "A" {
            for rating in set.ratings {
                sum += rating.value as i32;
            }
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let (workflows, _) = parse_input(input);

    let mut stack = vec![("in", 0, Status::default())];
    let mut accepted = 0;

    while let Some((curr, idx, status)) = stack.pop() {
        if curr == "A" {
            accepted += status.value();
            continue;
        } else if curr == "R" {
            continue;
        }

        let rule = workflows
            .iter()
            .find_map(|workflow| (workflow.name == curr).then(|| &workflow.rules[idx]))
            .unwrap();

        let next = rule.send_to;

        let Some(ref condition) = rule.condition else {
            stack.push((next, 0, status));
            continue;
        };

        let part = condition.left;

        match condition.op {
            Op::Greater => {
                if status[part].end <= condition.right {
                    stack.push((curr, idx + 1, status));
                } else if status[part].start > condition.right {
                    stack.push((next, 0, status));
                } else {
                    let mut else_status = status.clone();
                    else_status[part].end = condition.right + 1;
                    stack.push((curr, idx + 1, else_status));

                    let mut match_status = status.clone();
                    match_status[part].start = condition.right + 1;
                    stack.push((next, 0, match_status));
                }
            }
            Op::Less => {
                if status[part].start >= condition.right {
                    stack.push((curr, idx + 1, status));
                } else if status[part].end < condition.right {
                    stack.push((next, 0, status));
                } else {
                    let mut else_status = status.clone();
                    else_status[part].start = condition.right;
                    stack.push((curr, idx + 1, else_status));

                    let mut match_status = status.clone();
                    match_status[part].end = condition.right;
                    stack.push((next, 0, match_status))
                }
            }
        }
    }

    accepted
}

#[derive(Clone, Debug)]
struct Status {
    parts: [Range<i16>; 4],
}

impl Status {
    fn value(&self) -> u64 {
        self.parts.iter().map(|range| range.len() as u64).product()
    }

    fn merge(&mut self, other: Self) {
        for i in 0..4 {
            self.parts[i] = self.parts[i].start.max(other.parts[i].start)
                ..self.parts[i].end.min(other.parts[i].end);
        }
    }
}

impl Index<Part> for Status {
    type Output = Range<i16>;

    fn index(&self, part: Part) -> &Self::Output {
        self.parts.index(part as usize)
    }
}

impl IndexMut<Part> for Status {
    fn index_mut(&mut self, part: Part) -> &mut Self::Output {
        self.parts.index_mut(part as usize)
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            parts: [0_u8; 4].map(|_| 1..4001),
        }
    }
}

fn parse_input(input: &str) -> (Vec<WorkFlow>, Vec<RatingSet>) {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let workflows: Vec<_> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.trim_end_matches('}').split_once("{").unwrap();

            let rules: Vec<_> = rules
                .split(',')
                .map(|rule| {
                    let Some((condition, send_to)) = rule.split_once(':') else {
                        return Rule {
                            condition: None,
                            send_to: rule,
                        };
                    };

                    Rule {
                        condition: Some(Condition::new(condition)),
                        send_to,
                    }
                })
                .collect();

            WorkFlow { name, rules }
        })
        .collect();

    let ratings: Vec<_> = ratings
        .lines()
        .map(|line| {
            let mut ratings = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',');

            let ratings = [
                ratings.next().map(Rating::new).unwrap(),
                ratings.next().map(Rating::new).unwrap(),
                ratings.next().map(Rating::new).unwrap(),
                ratings.next().map(Rating::new).unwrap(),
            ];

            RatingSet {
                workflow: "in",
                ratings,
            }
        })
        .collect();

    (workflows, ratings)
}

#[derive(Debug)]
struct RatingSet<'a> {
    workflow: &'a str,
    ratings: [Rating; 4],
}

#[derive(Debug)]
struct Rating {
    name: Part,
    value: i16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn new(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

impl Rating {
    fn new(s: &str) -> Self {
        let (name, value) = s.split_once('=').unwrap();

        Self {
            name: Part::new(name),
            value: value.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct WorkFlow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    send_to: &'a str,
}

#[derive(Debug)]
struct Condition {
    left: Part,
    op: Op,
    right: i16,
}

impl Condition {
    fn new(s: &str) -> Self {
        if let Some(_) = s.find('>') {
            let (name, value) = s.split_once('>').unwrap();

            Self {
                left: Part::new(name),
                op: Op::Greater,
                right: value.parse().unwrap(),
            }
        } else if let Some(_) = s.find('<') {
            let (name, value) = s.split_once('<').unwrap();

            Self {
                left: Part::new(name),
                op: Op::Less,
                right: value.parse().unwrap(),
            }
        } else {
            panic!()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Op {
    Greater,
    Less,
}
