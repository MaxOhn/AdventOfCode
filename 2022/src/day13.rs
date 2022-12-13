use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
};

use nom::{
    branch::alt, character::complete as ch, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    run_manual(input)
    // run_nom(input)
}

pub fn run_manual(input: &str) -> Result<Solution> {
    run_with_fn(input, Packet::parse_manual)
}

pub fn run_nom(input: &str) -> Result<Solution> {
    run_with_fn(input, Packet::parse_nom)
}

fn run_with_fn(input: &str, f: fn(&str) -> Result<Packet>) -> Result<Solution> {
    let mut p1 = 0;

    let divider1 = Packet::List(vec![Packet::Num(2)]);
    let mut idx1 = 1;

    let divider2 = Packet::List(vec![Packet::Num(6)]);
    let mut idx2 = 2;

    for (group, i) in input.split("\n\n").zip(1..) {
        let (line_a, line_b) = group.split_once('\n').wrap_err("invalid group")?;
        let packet_a = (f)(line_a)?;
        let packet_b = (f)(line_b)?;

        p1 += (packet_a <= packet_b) as i32 * i;

        idx1 += (packet_a < divider1) as usize;
        idx1 += (packet_b < divider1) as usize;
        idx2 += (packet_a < divider2) as usize;
        idx2 += (packet_b < divider2) as usize;
    }

    let p2 = idx1 * idx2;

    Ok(Solution::new().part1(p1).part2(p2))
}

#[derive(PartialEq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse_manual(line: &str) -> Result<Self> {
        let Some((b'[', rest)) = line.as_bytes().split_first() else {
            bail!("packet must start with `[`");
        };

        Self::parse_list(rest).map(|(packet, _)| packet)
    }

    fn parse_list(mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut list = Vec::new();

        loop {
            match bytes.split_first() {
                Some((b'[', rest)) => {
                    let (item, rest) = Self::parse_list(rest)?;
                    bytes = rest;
                    list.push(item);
                }
                Some((byte @ b'0'..=b'9', rest)) => {
                    let (packet, rest) = Self::parse_num(*byte, rest)?;
                    bytes = rest;
                    list.push(packet);
                }
                Some((b',', rest)) => bytes = rest,
                Some((b']', rest)) => return Ok((Self::List(list), rest)),
                None => bail!("unexpected end of list"),
                Some((byte, _)) => bail!("invalid byte `{byte}`"),
            }
        }
    }

    fn parse_num(start: u8, mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut num = start & 0xF;

        loop {
            match bytes.split_first() {
                Some((byte @ b'0'..=b'9', rest)) => {
                    num *= 10;
                    num += byte & 0xF;
                    bytes = rest;
                }
                Some((b',' | b']', _)) | None => return Ok((Self::Num(num), bytes)),
                Some((byte, _)) => bail!("unexpected byte `{byte}` while parsing number"),
            }
        }
    }

    fn parse_nom(input: &str) -> Result<Self> {
        fn list(input: &str) -> IResult<&str, Packet> {
            let num = map(ch::u8, Packet::Num);
            let list_or_num = alt((list, num));
            let separated = separated_list0(ch::char(','), list_or_num);
            let delim = delimited(ch::char('['), separated, ch::char(']'));
            let mut mapped = map(delim, Packet::List);

            (mapped)(input)
        }

        let (_, packet) = list(input)
            .map_err(|e| e.to_owned())
            .wrap_err("failed to parse packet")?;

        Ok(packet)
    }
}

impl PartialOrd for Packet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Num(this), Packet::Num(that)) => this.partial_cmp(that),
            (Packet::Num(this), that @ Packet::List(_)) => this.partial_cmp(that),
            (this @ Packet::List(_), Packet::Num(that)) => this.partial_cmp(that),
            (Packet::List(this), Packet::List(that)) => this.partial_cmp(that),
        }
    }
}

impl PartialEq<u8> for Packet {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        match self {
            Packet::Num(n) => n.eq(other),
            Packet::List(_) => false,
        }
    }
}

impl PartialOrd<u8> for Packet {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        match self {
            Packet::Num(n) => n.partial_cmp(other),
            Packet::List(list) => match list.as_slice() {
                [] => Some(Ordering::Less),
                [a] => a.partial_cmp(other),
                [a, _, ..] => match a.partial_cmp(other)? {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Equal | Ordering::Greater => Some(Ordering::Greater),
                },
            },
        }
    }
}

impl PartialEq<Packet> for u8 {
    #[inline]
    fn eq(&self, other: &Packet) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Packet> for u8 {
    #[inline]
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::List(list) => {
                f.write_str("[")?;
                let mut iter = list.iter();

                if let Some(item) = iter.next() {
                    write!(f, "{item}")?;

                    for item in iter {
                        write!(f, ",{item}")?;
                    }
                }

                f.write_str("]")
            }
        }
    }
}
