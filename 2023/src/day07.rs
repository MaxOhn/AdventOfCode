use std::{cmp::Ordering, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let hands = parse_input(input.trim())?;

    let p1 = part1(hands.clone());
    let p2 = part2(hands);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_input(input: &str) -> Result<Vec<(Hand, u32)>> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, value) = line.split_once(' ').wrap_err("invalid line")?;
        let hand: Hand = hand.parse()?;
        let bid: u32 = value.parse().wrap_err("invalid bid")?;

        hands.push((hand, bid));
    }

    Ok(hands)
}

fn part1(mut hands: Vec<(Hand, u32)>) -> u32 {
    sort_by_type(&mut hands, Hand::get_type, Card::cmp);

    sum_values(&hands)
}

fn part2(mut hands: Vec<(Hand, u32)>) -> u32 {
    sort_by_type(&mut hands, Hand::get_type_with_joker, Card::cmp_joker);

    sum_values(&hands)
}

fn sort_by_type<T, C>(hands: &mut [(Hand, u32)], get_type: T, cmp: C)
where
    T: Fn(Hand) -> Type,
    C: Fn(&Card, &Card) -> Ordering,
{
    hands.sort_unstable_by(|(a, _), (b, _)| {
        get_type(*a).cmp(&get_type(*b)).then_with(|| {
            for (a, b) in a.cards.iter().zip(b.cards.iter()) {
                match cmp(a, b) {
                    Ordering::Equal => {}
                    other => return other,
                }
            }

            Ordering::Equal
        })
    });
}

fn sum_values(hands: &[(Hand, u32)]) -> u32 {
    hands
        .into_iter()
        .zip(1..)
        .map(|((_, bid), rank)| bid * rank)
        .sum()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl Card {
    fn cmp_joker(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::Joker, Card::Joker) => Ordering::Equal,
            (Card::Joker, _) => Ordering::Less,
            (_, Card::Joker) => Ordering::Greater,
            (a, b) => a.cmp(b),
        }
    }

    fn from_byte(byte: u8) -> Result<Self> {
        let card = match byte {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::Ten,
            b'J' => Card::Joker,
            b'Q' => Card::Queen,
            b'K' => Card::King,
            b'A' => Card::Ace,
            _ => eyre::bail!("invalid card byte `{byte}`"),
        };

        Ok(card)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn get_type(self) -> Type {
        if self.is_five_of_a_kind() {
            Type::FiveOfAKind
        } else if self.is_four_of_a_kind() {
            Type::FourOfAKind
        } else if self.is_full_house() {
            Type::FullHouse
        } else if self.is_three_of_a_kind() {
            Type::ThreeOfAKind
        } else if self.is_two_pair() {
            Type::TwoPair
        } else if self.is_one_pair() {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }

    fn get_type_with_joker(self) -> Type {
        if self.with_joker(Self::is_five_of_a_kind) {
            Type::FiveOfAKind
        } else if self.with_joker(Self::is_four_of_a_kind) {
            Type::FourOfAKind
        } else if self.with_joker(Self::is_full_house) {
            Type::FullHouse
        } else if self.with_joker(Self::is_three_of_a_kind) {
            Type::ThreeOfAKind
        } else if self.with_joker(Self::is_two_pair) {
            Type::TwoPair
        } else if self.with_joker(Self::is_one_pair) {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }

    fn with_joker(self, f: impl Fn(Self) -> bool + Copy) -> bool {
        const OPTIONS: [Card; 12] = [
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Queen,
            Card::King,
            Card::Ace,
        ];

        for (i, card) in self.cards.into_iter().enumerate() {
            if card != Card::Joker {
                continue;
            }

            let mut new = self;

            for option in OPTIONS {
                new.cards[i] = option;

                if new.with_joker(f) {
                    return true;
                }
            }
        }

        f(self)
    }

    fn is_five_of_a_kind(self) -> bool {
        for window in self.cards.windows(2) {
            if window[0] != window[1] {
                return false;
            }
        }

        true
    }

    fn is_four_of_a_kind(self) -> bool {
        let mut first = 0;
        let mut second = 0;

        for card in self.cards {
            if card == self.cards[0] {
                first += 1;
            }

            if card == self.cards[1] {
                second += 1;
            }
        }

        first == 4 || second == 4
    }

    fn is_full_house(self) -> bool {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut fourth = 0;

        for card in self.cards {
            if card == self.cards[0] {
                first += 1;
            }

            if card == self.cards[1] {
                second += 1;
            }

            if card == self.cards[2] {
                third += 1;
            }

            if card == self.cards[3] {
                fourth += 1;
            }
        }

        (first == 2 || second == 2 || third == 2 || fourth == 2)
            && (first == 3 || second == 3 || third == 3 || fourth == 3)
    }

    fn is_three_of_a_kind(self) -> bool {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;

        for card in self.cards {
            if card == self.cards[0] {
                first += 1;
            }

            if card == self.cards[1] {
                second += 1;
            }

            if card == self.cards[2] {
                third += 1;
            }
        }

        first == 3 || second == 3 || third == 3
    }

    fn is_two_pair(self) -> bool {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut fourth = 0;

        for card in self.cards {
            if card == self.cards[0] {
                first += 1;
            }

            if card == self.cards[1] {
                second += 1;
            }

            if card == self.cards[2] {
                third += 1;
            }

            if card == self.cards[3] {
                fourth += 1;
            }
        }

        let pairs = (first == 2) as usize
            + (second == 2) as usize
            + (third == 2) as usize
            + (fourth == 2) as usize;

        pairs >= 3
    }

    fn is_one_pair(self) -> bool {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut fourth = 0;

        for card in self.cards {
            if card == self.cards[0] {
                first += 1;
            }

            if card == self.cards[1] {
                second += 1;
            }

            if card == self.cards[2] {
                third += 1;
            }

            if card == self.cards[3] {
                fourth += 1;
            }
        }

        (first == 2 || second == 2 || third == 2 || fourth == 2)
            && (first < 3 && second < 3 && third < 3 && fourth < 3)
    }
}

impl FromStr for Hand {
    type Err = Report;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let mut iter = hand.bytes().map(Card::from_byte);

        let cards = [
            iter.next().wrap_err("invalid hand")??,
            iter.next().wrap_err("invalid hand")??,
            iter.next().wrap_err("invalid hand")??,
            iter.next().wrap_err("invalid hand")??,
            iter.next().wrap_err("invalid hand")??,
        ];

        Ok(Self { cards })
    }
}
