use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    mem,
    ops::{BitAnd, BitOr, BitXor},
    str::Lines,
};

use ahash::HashSetExt;
use aoc_rust::Solution;
use eyre::Result;
use fxhash::{FxHashMap, FxHashSet};
use nom::{
    branch::alt,
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

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut inouts = parse_inputs(&mut lines);
    let gates = parse_gates(&mut lines);

    gates.iter().fold(0, |acc, gate| match gate.out.z() {
        Some(z) => acc | (gate.out.eval(&mut inouts, &gates) as u64) << z,
        None => acc,
    })
}

fn part2(input: &str) -> String {
    let (_, input) = input.split_once("\n\n").expect("bad input");
    let gates = parse_gates(&mut input.lines());

    let mut swapped = FxHashSet::with_capacity(8);

    let x00 = InOut(*b"x00");
    let y00 = InOut(*b"y00");
    let z00 = InOut(*b"z00");

    /*
        <https://en.wikipedia.org/wiki/Adder_(electronics)>

        x ---\--------\
             AND-\    XOR--- z
        y ---/----+---/
                   \-------- C
    */

    let xor_out = gates
        .iter()
        .find(|gate| gate.has_lhs(x00, XOR, y00))
        .unwrap()
        .out;

    if xor_out != z00 {
        swapped.insert(xor_out);
    }

    let mut carry = gates
        .iter()
        .find(|gate| gate.has_lhs(x00, AND, y00))
        .unwrap()
        .out;

    let mut i = 1;

    loop {
        let a = (i / 10) + b'0';
        let b = (i % 10) + b'0';
        i += 1;

        let x = InOut([b'x', a, b]);
        let y = InOut([b'y', a, b]);
        let z = InOut([b'z', a, b]);

        /*
            x ---\
                 XOR1---\
            y ---/      XOR2 --- z
            C ----------/
        */

        let Some(xor1) = gates.iter().find(|gate| gate.has_lhs(x, XOR, y)) else {
            break;
        };

        let xor2 = gates
            .iter()
            .find(|gate| gate.op == XOR && gate.has_either_input(xor1.out, carry))
            .unwrap();

        if xor2.out != z {
            swapped.insert(z);
            swapped.insert(xor2.out);
        }

        if !xor2.has_input(xor1.out) {
            swapped.insert(xor1.out);
        }

        if !xor2.has_input(carry) {
            swapped.insert(carry);
        }

        /*
            x ---\--------\
                 XOR1-\   AND1----\
            y ---/----+---/       OR--- C
                      |           |
                     AND2--------/
            C -------/
        */

        let and1_out = gates
            .iter()
            .find(|gate| gate.has_lhs(x, AND, y))
            .unwrap()
            .out;

        let and2 = gates
            .iter()
            .find(|gate| gate.op == AND && gate.has_either_input(xor1.out, carry))
            .unwrap();

        if !and2.has_input(xor1.out) {
            swapped.insert(xor1.out);
        }

        if !and2.has_input(carry) {
            swapped.insert(carry);
        }

        let or = gates
            .iter()
            .find(|gate| gate.op == OR && gate.has_either_input(and1_out, and2.out))
            .unwrap();

        if !or.has_input(and1_out) {
            swapped.insert(and1_out);
        }

        if !or.has_input(and2.out) {
            swapped.insert(and2.out);
        }

        carry = or.out;
    }

    let mut swapped: Vec<_> = swapped.into_iter().collect();
    swapped.sort_unstable();

    swapped.join(",")
}

fn parse_inputs(lines: &mut Lines<'_>) -> FxHashMap<InOut, bool> {
    let mut inouts = FxHashMap::default();

    for line in lines {
        if line.is_empty() {
            return inouts;
        }

        let (_, (inout, value)) =
            all_consuming(separated_pair(InOut::parse, by::tag(": "), ch::u8))(line)
                .expect("bad input");

        inouts.insert(inout, value == 1);
    }

    panic!("missing gates")
}

fn parse_gates(lines: &mut Lines<'_>) -> FxHashSet<Gate> {
    let mut gates = FxHashSet::default();

    for line in lines {
        let (_, gate) = all_consuming(Gate::parse)(line).expect("bad input");
        gates.insert(gate);
    }

    gates
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InOut([u8; 3]);

impl InOut {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, name) = by::take(3_usize)(input)?;

        Ok((rest, Self(name.as_bytes().try_into().unwrap())))
    }

    fn z(self) -> Option<u8> {
        if let Self([b'z', a, b]) = self {
            Some((a & 0xF) * 10 + (b & 0xF))
        } else {
            None
        }
    }

    fn eval(self, values: &mut FxHashMap<InOut, bool>, gates: &FxHashSet<Gate>) -> bool {
        if let Some(value) = values.get(&self) {
            return *value;
        }

        let value = gates.get(&self).unwrap().eval(values, gates);
        values.insert(self, value);

        value
    }
}

impl Borrow<str> for InOut {
    fn borrow(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

#[derive(PartialEq, Eq)]
struct Gate {
    in1: InOut,
    op: fn(bool, bool) -> bool,
    in2: InOut,
    out: InOut,
}

const AND: fn(bool, bool) -> bool = bool::bitand;
const OR: fn(bool, bool) -> bool = bool::bitor;
const XOR: fn(bool, bool) -> bool = bool::bitxor;

impl Gate {
    fn has_lhs(&self, a: InOut, op: fn(bool, bool) -> bool, b: InOut) -> bool {
        self.in1 == a && self.op == op && self.in2 == b
    }

    fn has_input(&self, input: InOut) -> bool {
        self.in1 == input || self.in2 == input
    }

    fn has_either_input(&self, a: InOut, b: InOut) -> bool {
        self.has_input(a) || self.has_input(b)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_op = |input| {
            let (rest, op) = alt((by::tag("AND"), by::tag("OR"), by::tag("XOR")))(input)?;

            let op = match op {
                "AND" => AND,
                "OR" => OR,
                "XOR" => XOR,
                _ => unreachable!(),
            };

            Ok((rest, op))
        };

        let (rest, mut in1) = InOut::parse(input)?;
        let (rest, op) = delimited(by::tag(" "), parse_op, by::tag(" "))(rest)?;
        let (rest, mut in2) = InOut::parse(rest)?;
        let (rest, out) = preceded(by::tag(" -> "), InOut::parse)(rest)?;

        if in2 < in1 {
            mem::swap(&mut in1, &mut in2);
        }

        Ok((rest, Self { in1, op, in2, out }))
    }

    fn eval(&self, values: &mut FxHashMap<InOut, bool>, gates: &FxHashSet<Gate>) -> bool {
        let Self {
            in1,
            op,
            in2,
            out: _,
        } = self;

        (op)(in1.eval(values, gates), in2.eval(values, gates))
    }
}

impl PartialEq<InOut> for Gate {
    fn eq(&self, gate: &InOut) -> bool {
        self.out == *gate
    }
}

impl Hash for Gate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.out.hash(state);
    }
}

impl Borrow<InOut> for Gate {
    fn borrow(&self) -> &InOut {
        &self.out
    }
}
