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

    // assert monkeys to justify unwraps
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

    let (a, b) = match monkeys.remove("root").wrap_err("missing root")? {
        Monkey::Value(_) => bail!("root must be binary operation"),
        Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (a, b),
    };

    let p2 = if humans(a, &monkeys) == 1 {
        solve(a, eval(b, &monkeys), &monkeys)
    } else if humans(b, &monkeys) == 1 {
        solve(b, eval(a, &monkeys), &monkeys)
    } else {
        bail!("humn must occur exactly once")
    };

    Ok(Solution::new().part1(p1).part2(p2))
}

fn eval(name: &str, monkeys: &Monkeys) -> i64 {
    match monkeys.get(name).unwrap() {
        Monkey::Value(n) => *n,
        Monkey::Add(a, b) => eval(a, monkeys) + eval(b, monkeys),
        Monkey::Sub(a, b) => eval(a, monkeys) - eval(b, monkeys),
        Monkey::Mul(a, b) => eval(a, monkeys) * eval(b, monkeys),
        Monkey::Div(a, b) => eval(a, monkeys) / eval(b, monkeys),
    }
}

fn humans(name: &str, monkeys: &Monkeys) -> usize {
    if name == "humn" {
        1
    } else {
        match monkeys.get(name).unwrap() {
            Monkey::Value(_) => 0,
            Monkey::Add(n1, n2)
            | Monkey::Sub(n1, n2)
            | Monkey::Mul(n1, n2)
            | Monkey::Div(n1, n2) => humans(n1, monkeys) + humans(n2, monkeys),
        }
    }
}

fn solve(name: &str, v: i64, monkeys: &Monkeys<'_>) -> i64 {
    if name == "humn" {
        v
    } else {
        match monkeys.get(name).unwrap() {
            Monkey::Value(n) => *n,
            Monkey::Add(a, b) if humans(a, monkeys) == 1 => solve(a, v - eval(b, monkeys), monkeys),
            Monkey::Add(a, b) => solve(b, v - eval(a, monkeys), monkeys),
            Monkey::Sub(a, b) if humans(a, monkeys) == 1 => solve(a, v + eval(b, monkeys), monkeys),
            Monkey::Sub(a, b) => solve(b, eval(a, monkeys) - v, monkeys),
            Monkey::Mul(a, b) if humans(a, monkeys) == 1 => solve(a, v / eval(b, monkeys), monkeys),
            Monkey::Mul(a, b) => solve(b, v / eval(a, monkeys), monkeys),
            Monkey::Div(a, b) if humans(a, monkeys) == 1 => solve(a, v * eval(b, monkeys), monkeys),
            Monkey::Div(a, b) => solve(b, eval(a, monkeys) / v, monkeys),
        }
    }
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
