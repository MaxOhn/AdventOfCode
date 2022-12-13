use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut p1 = 0;
    let mut packets = Vec::with_capacity(16);

    for (group, i) in input.split("\n\n").zip(1..) {
        let (line_a, line_b) = group.split_once('\n').wrap_err("invalid group")?;
        let packet_a: Packet = line_a.parse().wrap_err("invalid packet")?;
        let packet_b: Packet = line_b.parse().wrap_err("invalid packet")?;

        if let Some(Ordering::Less | Ordering::Equal) = packet_a.partial_cmp(&packet_b) {
            p1 += i;
        }

        packets.push(packet_a);
        packets.push(packet_b);
    }

    let divider1 = Packet::List(vec![Packet::Num(2)]);
    let divider2 = Packet::List(vec![Packet::Num(6)]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_unstable();

    let idx_divider1 = packets.binary_search(&divider1).unwrap() + 1;
    let idx_divider2 = packets.binary_search(&divider2).unwrap() + 1;

    let p2 = idx_divider1 * idx_divider2;

    Ok(Solution::new().part1(p1).part2(p2))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse_list(mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut data = Vec::new();

        loop {
            match bytes.first() {
                Some(b'[') => {
                    let (item, rest) = Self::parse_list(&bytes[1..])?;
                    bytes = rest;
                    data.push(item);
                }
                Some(b'0'..=b'9') => {
                    let (item, rest) = Self::parse_num(bytes)?;
                    bytes = rest;
                    data.push(item);
                }
                Some(b',') => bytes = &bytes[1..],
                Some(b']') => return Ok((Self::List(data), &bytes[1..])),
                None => bail!("unexpected end of list"),
                Some(b) => bail!("invalid byte `{b}`"),
            }
        }
    }

    fn parse_num(mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut num = 0;

        loop {
            match bytes.first() {
                Some(b @ b'0'..=b'9') => {
                    num *= 10;
                    num += (b & 0xF) as i32;
                    bytes = &bytes[1..];
                }
                Some(b',' | b']') | None => return Ok((Self::Num(num), bytes)),
                Some(b) => bail!("unexpected byte `{b}` while parsing number"),
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let bytes = line.as_bytes();
        ensure!(bytes.first() == Some(&b'['), "missing `[`");
        let (packet, _) = Self::parse_list(&bytes[1..])?;

        Ok(packet)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(this), Packet::Num(that)) => this.cmp(that),
            (Packet::Num(n), that @ Packet::List(_)) => {
                let this = Self::List(vec![Self::Num(*n)]);

                this.cmp(that)
            }
            (this @ Packet::List(_), Packet::Num(n)) => {
                let that = Self::List(vec![Self::Num(*n)]);

                this.cmp(&that)
            }
            (Packet::List(this), Packet::List(that)) => {
                let mut this = this.iter();
                let mut that = that.iter();

                loop {
                    match (this.next(), that.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(this), Some(that)) => match this.cmp(that) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => {}
                            Ordering::Greater => return Ordering::Greater,
                        },
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
