use std::{collections::VecDeque, convert::identity, str::FromStr};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    // dyn_monkey_op::run(input)
    // dyn_monkey_op_no_pop::run(input)
    enum_monkey_op_no_pop::run(input)
}

pub mod dyn_monkey_op {
    use super::*;

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
                while let Some((next_monkey, worry)) = simulate_monkey(get_mut!(monkeys, i), f) {
                    get_mut!(monkeys, next_monkey).items.push_back(worry);
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
}

pub mod dyn_monkey_op_no_pop {
    use super::*;

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
        let mut buf = Vec::new();

        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                buf.append(&mut get_mut!(monkeys, i).items);

                for worry in buf.drain(..) {
                    let monkey = get_mut!(monkeys, i);
                    monkey.inspect_count += 1;

                    let mut worry = (monkey.op)(worry);
                    f(&mut worry);

                    let next_monkey = if worry % monkey.test_div == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    get_mut!(monkeys, next_monkey).items.push(worry);
                }
            }
        }
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
        items: Vec<Worry>,
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
}

pub mod enum_monkey_op_no_pop {
    use super::*;

    pub fn run(input: &str) -> Result<Solution> {
        let mut monkeys = parse_monkeys(input)?;

        let p1 = part1(&mut monkeys.clone())?;
        let p2 = part2(&mut monkeys)?;

        Ok(Solution::new().part1(p1).part2(p2))
    }

    fn parse_monkeys(input: &str) -> Result<Vec<Monkey>> {
        input.split("\n\n").map(str::parse).collect()
    }

    fn part1(monkeys: &mut [Monkey]) -> Result<u64> {
        simulate_rounds(monkeys, 20, |val| *val /= 3);

        Ok(mult_two_max(monkeys))
    }

    fn part2(monkeys: &mut [Monkey]) -> Result<u64> {
        let modulo: Worry = monkeys.iter().map(|monkey| monkey.test_div).product();
        simulate_rounds(monkeys, 10_000, |val| *val %= modulo);

        Ok(mult_two_max(monkeys))
    }

    fn simulate_rounds<F>(monkeys: &mut [Monkey], rounds: usize, f: F)
    where
        F: Fn(&mut Worry) + Copy,
    {
        let mut buf = Vec::new();

        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                buf.append(&mut get_mut!(monkeys, i).items);

                for worry in buf.drain(..) {
                    let monkey = get_mut!(monkeys, i);
                    monkey.inspect_count += 1;

                    let mut worry = monkey.op.exec(worry);
                    f(&mut worry);

                    let next_monkey = if worry % monkey.test_div == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    get_mut!(monkeys, next_monkey).items.push(worry);
                }
            }
        }
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

    #[derive(Copy, Clone)]
    enum MonkeyOp {
        Add { a: Option<Worry>, b: Option<Worry> },
        Mult { a: Option<Worry>, b: Option<Worry> },
        Assign { val: Option<Worry> },
    }

    impl FromStr for MonkeyOp {
        type Err = Report;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let mut op_iter = line.split(' ');

            let term1 = op_iter.next().wrap_err("missing term1")?;
            let op = op_iter.next();
            let term2 = op_iter.next();

            let a = match term1 {
                "old" => None,
                _ => term1
                    .parse()
                    .map(Some)
                    .map_err(|_| eyre!("invalid term1 `{term1}`"))?,
            };

            let b = match term2 {
                Some("old") | None => None,
                Some(term2) => term2
                    .parse()
                    .map(Some)
                    .map_err(|_| eyre!("invalid term2 `{term2}`"))?,
            };

            let op = match op {
                Some("+") => Self::Add { a, b },
                Some("*") => Self::Mult { a, b },
                None => Self::Assign { val: a },
                Some(op) => bail!("unknow operation `{op}`"),
            };

            Ok(op)
        }
    }

    impl MonkeyOp {
        fn exec(&self, old: Worry) -> Worry {
            match self {
                Self::Add { a, b } => {
                    let a = a.unwrap_or(old);
                    let b = b.unwrap_or(old);

                    a + b
                }
                Self::Mult { a, b } => {
                    let a = a.unwrap_or(old);
                    let b = b.unwrap_or(old);

                    a * b
                }
                Self::Assign { val } => val.unwrap_or(old),
            }
        }
    }

    #[derive(Clone)]
    struct Monkey {
        items: Vec<Worry>,
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

            let op = lines
                .next()
                .and_then(|line| line.strip_prefix("  Operation: new = "))
                .wrap_err("invalid op line")?
                .parse()
                .wrap_err("invalid monkey op")?;

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
}
