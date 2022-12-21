use std::collections::HashMap;

use ahash::RandomState;

use crate::prelude::*;

type Monkeys<'m> = HashMap<&'m str, Monkey<'m>, RandomState>;

pub fn run(input: &str) -> Result<Solution> {
    let mut monkeys = input
        .lines()
        .map(|line| {
            let (name, monkey) = line.split_once(": ").wrap_err("invalid line")?;
            let monkey = Monkey::from_str(monkey)?;

            Ok((name, monkey))
        })
        .collect::<Result<Monkeys<'_>>>()?;

    // assert monkeys to justify later unwraps
    let invalid_monkey = monkeys.values().find_map(|monkey| match monkey {
        Monkey::Value(_) => None,
        Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (!monkeys
            .contains_key(a))
        .then_some(a)
        .or_else(|| (!monkeys.contains_key(b)).then_some(b)),
    });

    if let Some(name) = invalid_monkey {
        bail!("monkey `{name}` appears in expression but has no expression itself");
    }

    let p1 = eval("root", &monkeys);

    let humans = count_humans("root", &monkeys);

    ensure!(
        humans == 1,
        "expected exactly one path to `humn`, found {humans}",
    );

    let (a, b) = match monkeys.remove("root").wrap_err("missing root")? {
        Monkey::Value(_) => bail!("root must be binary operation"),
        Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (a, b),
    };

    let mut chain = Vec::new();

    let p2 = if find_human(a, &monkeys, &mut chain) {
        part2(eval(b, &monkeys), &monkeys, &chain)
    } else if find_human(b, &monkeys, &mut chain) {
        part2(eval(a, &monkeys), &monkeys, &chain)
    } else {
        unreachable!()
    };

    Ok(Solution::new().part1(p1).part2(p2))
}

fn eval<'m>(name: &'m str, monkeys: &Monkeys<'m>) -> i64 {
    match monkeys.get(name).unwrap() {
        Monkey::Value(n) => *n,
        Monkey::Add(a, b) => eval(a, monkeys) + eval(b, monkeys),
        Monkey::Sub(a, b) => eval(a, monkeys) - eval(b, monkeys),
        Monkey::Mul(a, b) => eval(a, monkeys) * eval(b, monkeys),
        Monkey::Div(a, b) => eval(a, monkeys) / eval(b, monkeys),
    }
}

fn count_humans(name: &str, monkeys: &Monkeys<'_>) -> usize {
    match monkeys.get(name).unwrap() {
        Monkey::Value(_) => (name == "humn") as usize,
        Monkey::Add(n1, n2) | Monkey::Sub(n1, n2) | Monkey::Mul(n1, n2) | Monkey::Div(n1, n2) => {
            count_humans(n1, monkeys) + count_humans(n2, monkeys)
        }
    }
}

enum Side {
    Left,
    Right,
}

fn find_human<'m>(
    name: &'m str,
    monkeys: &'m Monkeys<'m>,
    chain: &mut Vec<(&'m Monkey<'m>, Side)>,
) -> bool {
    if name == "humn" {
        return true;
    }

    match monkeys.get(name).unwrap() {
        Monkey::Value(_) => false,
        monkey
        @ (Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b)) => {
            if find_human(a, monkeys, chain) {
                chain.push((monkey, Side::Left));

                true
            } else if find_human(b, monkeys, chain) {
                chain.push((monkey, Side::Right));

                true
            } else {
                false
            }
        }
    }
}

fn part2(mut solution: i64, monkeys: &Monkeys<'_>, chain: &[(&Monkey<'_>, Side)]) -> i64 {
    for entry in chain.iter().rev() {
        match entry {
            (Monkey::Add(_, m), Side::Left) | (Monkey::Add(m, _), Side::Right) => {
                solution -= eval(m, monkeys)
            }
            (Monkey::Mul(_, m), Side::Left) | (Monkey::Mul(m, _), Side::Right) => {
                solution /= eval(m, monkeys)
            }
            (Monkey::Sub(_, m), Side::Left) => solution += eval(m, monkeys),
            (Monkey::Div(_, m), Side::Left) => solution *= eval(m, monkeys),
            (Monkey::Sub(m, _), Side::Right) => solution = eval(m, monkeys) - solution,
            (Monkey::Div(m, _), Side::Right) => solution = eval(m, monkeys) / solution,
            (Monkey::Value(_), _) => unreachable!(),
        }
    }

    solution
}

enum Monkey<'m> {
    Value(i64),
    Add(&'m str, &'m str),
    Sub(&'m str, &'m str),
    Mul(&'m str, &'m str),
    Div(&'m str, &'m str),
}

impl<'m> Monkey<'m> {
    fn from_str(s: &'m str) -> Result<Self> {
        if let Ok(n) = s.parse() {
            return Ok(Self::Value(n));
        }

        let mut iter = s.split(' ');

        let a = iter.next().wrap_err("missing term 1")?;
        let op = iter.next().wrap_err("missing operation")?;
        let b = iter.next().wrap_err("missing term 2")?;

        match op {
            "+" => Ok(Self::Add(a, b)),
            "-" => Ok(Self::Sub(a, b)),
            "*" => Ok(Self::Mul(a, b)),
            "/" => Ok(Self::Div(a, b)),
            _ => bail!("invalid op"),
        }
    }
}
