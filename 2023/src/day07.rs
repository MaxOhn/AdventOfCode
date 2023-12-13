use std::{marker::PhantomData, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u32> {
    solve::<Part1>(input)
}

fn part2(input: &str) -> Result<u32> {
    solve::<Part2>(input)
}

fn solve<Part>(input: &str) -> Result<u32>
where
    Hand<Part>: FromStr<Err = Report>,
{
    let mut hands = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Hand<Part>>>>()
        .wrap_err("invalid hand")?;

    hands.sort_unstable_by_key(|hand| hand.value);

    let winnings = hands
        .into_iter()
        .zip(1..)
        .fold(0, |sum, (hand, rank)| sum + hand.bid * rank);

    Ok(winnings)
}

#[derive(Copy, Clone)]
struct Hand<Part> {
    value: u32,
    bid: u32,
    _phantom: PhantomData<Part>,
}

trait HandExt<Part> {
    fn combo(cards: [Card<Part>; 5]) -> Combo;
}

impl HandExt<Part1> for Hand<Part1> {
    fn combo(cards: [Card<Part1>; 5]) -> Combo {
        let mut counts = [0_u8; 15];

        for card in cards {
            counts[card.0 as usize] += 1;
        }

        let (_, second, [first]) = counts.select_nth_unstable(13) else {
            unreachable!()
        };

        match (*first, *second) {
            (5, _) => Combo::FiveOfAKind,
            (4, _) => Combo::FourOfAKind,
            (3, 2) => Combo::FullHouse,
            (3, _) => Combo::ThreeOfAKind,
            (2, 2) => Combo::TwoPair,
            (2, _) => Combo::OnePair,
            _ => Combo::HighCard,
        }
    }
}

impl HandExt<Part2> for Hand<Part2> {
    fn combo(cards: [Card<Part2>; 5]) -> Combo {
        let mut counts = [0_u8; 15];
        let mut jokers = 0;

        for card in cards {
            if card.is_joker() {
                jokers += 1;
            } else {
                counts[card.0 as usize] += 1;
            }
        }

        let (_, second, [first]) = counts.select_nth_unstable(13) else {
            unreachable!()
        };

        match (*first + jokers, *second) {
            (5, _) => Combo::FiveOfAKind,
            (4, _) => Combo::FourOfAKind,
            (3, 2) => Combo::FullHouse,
            (3, _) => Combo::ThreeOfAKind,
            (2, 2) => Combo::TwoPair,
            (2, _) => Combo::OnePair,
            _ => Combo::HighCard,
        }
    }
}

impl<Part> FromStr for Hand<Part>
where
    Hand<Part>: HandExt<Part>,
    Card<Part>: Copy + TryFrom<u8, Error = Report>,
{
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut bytes = line.bytes();

        let cards = [
            Card::try_from(bytes.next().wrap_err("missing card")?)?,
            Card::try_from(bytes.next().wrap_err("missing card")?)?,
            Card::try_from(bytes.next().wrap_err("missing card")?)?,
            Card::try_from(bytes.next().wrap_err("missing card")?)?,
            Card::try_from(bytes.next().wrap_err("missing card")?)?,
        ];

        eyre::ensure!(bytes.next() == Some(b' '), "missing whitespace");

        let bid = bytes.try_fold(0, |num, byte| match byte {
            b'0'..=b'9' => Ok(num * 10 + (byte & 0xF) as u32),
            _ => Err(eyre::eyre!("invalid digit byte `{byte}`")),
        })?;

        let combo = Self::combo(cards);

        let value = cards
            .into_iter()
            .fold(combo as u32, |value, card| (value << 4) | card.0 as u32);

        Ok(Self {
            value,
            bid,
            _phantom: PhantomData,
        })
    }
}

struct Card<Part>(u8, PhantomData<Part>);

impl<Part> Card<Part> {
    fn new(card: u8) -> Self {
        Self(card, PhantomData)
    }
}

impl TryFrom<u8> for Card<Part1> {
    type Error = Report;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let card = match byte {
            b'2'..=b'9' => byte - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => eyre::bail!("invalid card byte `{byte}`"),
        };

        Ok(Self::new(card))
    }
}

impl<Part> Copy for Card<Part> {}

impl<Part> Clone for Card<Part> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Card<Part2> {
    fn is_joker(self) -> bool {
        self.0 == 1
    }
}

impl TryFrom<u8> for Card<Part2> {
    type Error = Report;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let card = match byte {
            b'2'..=b'9' => byte - b'0',
            b'T' => 10,
            b'J' => 1,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => eyre::bail!("invalid card byte `{byte}`"),
        };

        Ok(Self::new(card))
    }
}

enum Combo {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Part1;

struct Part2;
