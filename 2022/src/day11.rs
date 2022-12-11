use std::{collections::VecDeque, convert::identity, str::FromStr};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_monkeys(input: &str) -> Result<Vec<Monkey>> {
    input.split("\n\n").map(str::parse).collect()
}

fn part1(input: &str) -> Result<u64> {
    let mut monkeys = parse_monkeys(input)?;
    simulate_rounds(&mut monkeys, 20, |val| *val /= 3);

    Ok(mult_two_max(&monkeys))
}

fn part2(input: &str) -> Result<u64> {
    let mut monkeys = parse_monkeys(input)?;
    let modulo: Worry = monkeys.iter().map(|monkey| monkey.test_div).product();
    simulate_rounds(&mut monkeys, 10_000, |val| *val %= modulo);

    Ok(mult_two_max(&monkeys))
}

fn simulate_rounds<F>(monkeys: &mut [Monkey], rounds: usize, f: F)
where
    F: Fn(&mut Worry) + Copy,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((next_monkey, worry)) = simulate_monkey(get_mut!(monkeys[i]), f) {
                get_mut!(monkeys[next_monkey]).items.push_back(worry);
            }
        }
    }
}

fn simulate_monkey(monkey: &mut Monkey, f: impl Fn(&mut Worry)) -> Option<(usize, Worry)> {
    let worry = monkey.items.pop_front()?;
    monkey.inspect_count += 1;

    let mut worry = (monkey.op)(worry);
    f(&mut worry);

    let next_monkey = if worry % monkey.test_div == 0 {
        monkey.if_true
    } else {
        monkey.if_false
    };

    Some((next_monkey, worry))
}

fn mult_two_max(monkeys: &[Monkey]) -> u64 {
    let (max1, max2) = monkeys
        .iter()
        .map(|monkey| monkey.inspect_count as u64)
        .fold((0, 0), |(max1, max2), count| {
            if count > max1 {
                (count, max1)
            } else if count > max2 {
                (max1, count)
            } else {
                (max1, max2)
            }
        });

    max1 * max2
}

type Worry = u64;
type MonkeyOp = Box<dyn Fn(Worry) -> Worry>;

struct Monkey {
    items: VecDeque<Worry>,
    op: MonkeyOp,
    test_div: Worry,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = Report;

    fn from_str(group: &str) -> Result<Self, Self::Err> {
        let mut lines = group.lines().skip(1);

        let items = lines
            .next()
            .and_then(|line| line.strip_prefix("  Starting items: "))
            .wrap_err("invalid items line")?
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()
            .wrap_err("invalid items")?;

        let mut op_iter = lines
            .next()
            .and_then(|line| line.strip_prefix("  Operation: new = "))
            .wrap_err("invalid op line")?
            .split(' ');

        let term1 = op_iter.next().wrap_err("invalid op")?;
        let op = op_iter.next();
        let term2 = op_iter.next();

        let op = match (term1, op, term2) {
            ("old", None, _) => Box::new(identity) as MonkeyOp,
            ("old", Some("+"), Some("old")) => Box::new(|old| old + old),
            ("old", Some("*"), Some("old")) => Box::new(|old| old * old),
            ("old", Some("+"), Some(b)) => {
                let b: Worry = b.parse().wrap_err("invalid term2")?;

                Box::new(move |old| old + b)
            }
            ("old", Some("*"), Some(b)) => {
                let b: Worry = b.parse().wrap_err("invalid term2")?;

                Box::new(move |old| old * b)
            }
            (a, None, _) => {
                let a: Worry = a.parse().wrap_err("invalid term1")?;

                Box::new(move |_| a)
            }
            (a, Some("+"), Some(b)) => {
                let a: Worry = a.parse().wrap_err("invalid term1")?;
                let b: Worry = b.parse().wrap_err("invalid term2")?;

                Box::new(move |_| a + b)
            }
            (a, Some("*"), Some(b)) => {
                let a: Worry = a.parse().wrap_err("invalid term1")?;
                let b: Worry = b.parse().wrap_err("invalid term2")?;

                Box::new(move |_| a * b)
            }
            (_, Some(op), _) => bail!("invalid op `{op}`"),
        };

        let test_div = lines
            .next()
            .and_then(|line| line.strip_prefix("  Test: divisible by "))
            .wrap_err("invalid test line")?
            .parse()
            .wrap_err("invalid test value")?;

        let if_true = lines
            .next()
            .and_then(|line| line.strip_prefix("    If true: throw to monkey "))
            .wrap_err("invalid if-true line")?
            .parse()
            .wrap_err("invalid if-true value")?;

        let if_false = lines
            .next()
            .and_then(|line| line.strip_prefix("    If false: throw to monkey "))
            .wrap_err("invalid if-false line")?
            .parse()
            .wrap_err("invalid if-false value")?;

        let monkey = Self {
            items,
            op,
            test_div,
            if_true,
            if_false,
            inspect_count: 0,
        };

        Ok(monkey)
    }
}
