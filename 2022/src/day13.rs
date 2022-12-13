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

impl FromStr for Packet {
    type Err = Report;

    #[inline]
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((b'[', rest)) = line.as_bytes().split_first() else {
            bail!("packet must start with `[`");
        };

        Self::parse_list(rest).map(|(packet, _)| packet)
    }
}

impl Packet {
    fn parse_list(mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut list = Vec::new();

        loop {
            match bytes.split_first() {
                Some((b'[', rest)) => {
                    let (item, rest) = Self::parse_list(rest)?;
                    bytes = rest;
                    list.push(item);
                }
                Some((b'0'..=b'9', _)) => {
                    let (packet, rest) = Self::parse_num(bytes)?;
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

    fn parse_num(mut bytes: &[u8]) -> Result<(Self, &[u8])> {
        let mut num = 0;

        loop {
            match bytes.split_first() {
                Some((byte @ b'0'..=b'9', rest)) => {
                    num *= 10;
                    num += (byte & 0xF) as i32;
                    bytes = rest;
                }
                Some((b',' | b']', _)) | None => return Ok((Self::Num(num), bytes)),
                Some((byte, _)) => bail!("unexpected byte `{byte}` while parsing number"),
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(this), Packet::Num(that)) => this.cmp(that),
            (Packet::Num(this), that @ Packet::List(_)) => this.partial_cmp(that).unwrap(),
            (this @ Packet::List(_), Packet::Num(that)) => this.partial_cmp(that).unwrap(),
            (Packet::List(this), Packet::List(that)) => {
                let mut this = this.iter();
                let mut that = that.iter();

                loop {
                    return match (this.next(), that.next()) {
                        (None, None) => Ordering::Equal,
                        (None, Some(_)) => Ordering::Less,
                        (Some(_), None) => Ordering::Greater,
                        (Some(this), Some(that)) => match this.cmp(that) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => Ordering::Greater,
                        },
                    };
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

impl PartialEq<i32> for Packet {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        match self {
            Packet::Num(n) => n.eq(other),
            Packet::List(_) => false,
        }
    }
}

impl PartialOrd<i32> for Packet {
    #[inline]
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
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

impl PartialEq<Packet> for i32 {
    #[inline]
    fn eq(&self, other: &Packet) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Packet> for i32 {
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
