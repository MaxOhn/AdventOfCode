use std::{cmp::Ordering, mem};

use nom::{
    branch::alt, character::complete as ch, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    run_manual(input)
    // run_nom(input)
    // run_wrapped(input)
}

pub fn run_manual(input: &str) -> Result<Solution> {
    run_with_fn(input, naive_packet::Packet::parse_manual)
}

pub fn run_nom(input: &str) -> Result<Solution> {
    run_with_fn(input, naive_packet::Packet::parse_nom)
}

pub fn run_wrapped(input: &str) -> Result<Solution> {
    run_with_fn(input, wrap_packet::Packet::from_str)
}

trait FromStr<'s>: Sized {
    fn from_str(s: &'s str) -> Result<Self>;
}

fn run_with_fn<'s, 'i, P>(input: &'i str, f: fn(&'i str) -> Result<P>) -> Result<Solution>
where
    P: FromStr<'s> + PartialOrd + 'i,
{
    let mut p1 = 0;

    let divider1 = P::from_str("[2]").unwrap();
    let mut idx1 = 1;

    let divider2 = P::from_str("[6]").unwrap();
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

mod wrap_packet {
    use super::*;

    #[derive(PartialEq)]
    pub struct Packet<'b> {
        bytes: &'b [u8],
    }

    impl<'b> Packet<'b> {
        fn new(bytes: &'b [u8]) -> Self {
            Self { bytes }
        }
    }

    impl<'s> FromStr<'s> for Packet<'s> {
        #[inline]
        fn from_str(s: &'s str) -> Result<Self> {
            Ok(Self::new(s.as_bytes()))
        }
    }

    impl PartialOrd for Packet<'_> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match (self.bytes, other.bytes) {
                ([b'[', this @ .., b']'], [b'[', that @ .., b']']) => {
                    let this = CommaSeparated::new(this);
                    let that = CommaSeparated::new(that);

                    this.partial_cmp(&that)
                }
                ([b'[', this @ .., b']'], that @ [b'0'..=b'9', ..]) => {
                    let this = CommaSeparated::new(this);
                    let that = CommaSeparated::new(that);

                    this.partial_cmp(&that)
                }
                (this @ [b'0'..=b'9'], [b'[', that @ .., b']']) => {
                    let this = CommaSeparated::new(this);
                    let that = CommaSeparated::new(that);

                    this.partial_cmp(&that)
                }
                ([this @ b'0'..=b'9', this_rest @ ..], [that @ b'0'..=b'9', that_rest @ ..]) => {
                    fn atoi(start: u8, rest: &[u8]) -> u8 {
                        rest.iter()
                            .fold(start & 0xF, |n, &byte| n * 10 + (byte & 0xF))
                    }

                    let this = atoi(*this, this_rest);
                    let that = atoi(*that, that_rest);

                    this.partial_cmp(&that)
                }
                _ => None,
            }
        }
    }

    #[derive(PartialEq)]
    struct CommaSeparated<'b> {
        bytes: &'b [u8],
    }

    impl<'b> CommaSeparated<'b> {
        fn new(bytes: &'b [u8]) -> Self {
            Self { bytes }
        }

        fn iter(&self) -> CommaSeparatedIter<'_> {
            CommaSeparatedIter { bytes: self.bytes }
        }
    }

    impl PartialOrd for CommaSeparated<'_> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let mut this = self.iter();
            let mut that = other.iter();

            loop {
                return match (this.next(), that.next()) {
                    (None, None) => Some(Ordering::Equal),
                    (None, Some(_)) => Some(Ordering::Less),
                    (Some(_), None) => Some(Ordering::Greater),
                    (Some(this), Some(that)) => match this.partial_cmp(&that)? {
                        Ordering::Less => Some(Ordering::Less),
                        Ordering::Equal => continue,
                        Ordering::Greater => Some(Ordering::Greater),
                    },
                };
            }
        }
    }

    struct CommaSeparatedIter<'b> {
        bytes: &'b [u8],
    }

    impl<'b> Iterator for CommaSeparatedIter<'b> {
        type Item = Packet<'b>;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            let mut depth = 0;
            let mut parsing_num = false;

            for (i, byte) in self.bytes.iter().enumerate() {
                match byte {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;

                        if depth == 0 {
                            let packet = Packet::new(&self.bytes[..=i]);
                            self.bytes = &self.bytes[self.bytes.len().min(i + 2)..];

                            return Some(packet);
                        }
                    }
                    b'0'..=b'9' if depth == 0 => parsing_num = true,
                    b',' if depth == 0 => {
                        let packet = Packet::new(&self.bytes[..i]);
                        self.bytes = &self.bytes[i + 1..];

                        return Some(packet);
                    }
                    _ => {}
                }
            }

            parsing_num
                .then(|| mem::take(&mut self.bytes))
                .map(Packet::new)
        }
    }
}

mod naive_packet {
    use super::*;

    #[derive(PartialEq)]
    pub enum Packet {
        Num(u8),
        List(Vec<Packet>),
    }

    impl FromStr<'_> for Packet {
        #[inline]
        fn from_str(s: &str) -> Result<Self> {
            Self::parse_manual(s)
        }
    }

    impl Packet {
        pub fn parse_manual(line: &str) -> Result<Self> {
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

        pub fn parse_nom(input: &str) -> Result<Self> {
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
}
