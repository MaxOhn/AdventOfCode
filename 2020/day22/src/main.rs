use hashbrown::HashSet;
use std::collections::VecDeque;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use util::Parse;

macro_rules! min_max {
    ($deck:ident, $len:ident) => {
        $deck
            .iter()
            .take($len as usize)
            .fold((u8::MAX, 0), |(min, max), &card| {
                (min.min(card), max.max(card))
            })
    };
}

type Deck = VecDeque<u8>;

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 33_473);
    assert_eq!(p2, 31_793);
}

enum Player {
    One,
    Two,
}

fn part1() -> usize {
    let start = Instant::now();

    let (mut deck1, mut deck2) = parse_decks();

    while !(deck1.is_empty() || deck2.is_empty()) {
        let card1 = deck1
            .pop_front()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });
        let card2 = deck2
            .pop_front()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let p1 = score(if deck1.is_empty() { deck2 } else { deck1 });

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 148µs

    p1
}

fn part2() -> usize {
    let start = Instant::now();

    let (deck1, deck2) = parse_decks();
    let (_, deck) = recurse(deck1, deck2);
    let p2 = score(deck);

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 6ms

    p2
}

fn recurse(mut deck1: Deck, mut deck2: Deck) -> (Player, Deck) {
    let mut history = HashSet::new();

    loop {
        if !history.insert((hash(&deck1), hash(&deck2))) {
            return (Player::One, deck1);
        }

        let card1 = match deck1.pop_front() {
            Some(card) => card,
            None => return (Player::Two, deck2),
        };

        let card2 = deck2
            .pop_front()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        let winner = if deck1.len() >= card1 as usize && deck2.len() >= card2 as usize {
            let (min1, max1) = min_max!(deck1, card1);
            let (min2, max2) = min_max!(deck2, card2);

            if max1 > max2
                && min1.min(min2) >= card1.min(deck1.len() as u8) + card2.min(deck2.len() as u8)
            {
                Player::One
            } else {
                let deck1 = deck1.iter().copied().take(card1 as usize).collect();
                let deck2 = deck2.iter().copied().take(card2 as usize).collect();

                recurse(deck1, deck2).0
            }
        } else if card1 > card2 {
            Player::One
        } else {
            Player::Two
        };

        match winner {
            Player::One => {
                deck1.push_back(card1);
                deck1.push_back(card2);

                if deck2.is_empty() {
                    return (Player::One, deck1);
                }
            }
            Player::Two => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }
}

fn score(deck: Deck) -> usize {
    deck.into_iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card as usize)
        .sum()
}

fn hash(deck: &Deck) -> u64 {
    deck.iter().fold(0, |hash, card| hash | (1 << *card))
}

fn parse_decks() -> (Deck, Deck) {
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut deck1 = VecDeque::with_capacity(16);

    let _ = input.read_line(&mut line);

    while {
        line.clear();
        let _ = input.read_line(&mut line);

        line.len() > 1
    } {
        deck1.push_back(Parse::parse(line.as_bytes()));
    }

    let _ = input.read_line(&mut line);
    line.clear();

    let mut deck2 = VecDeque::with_capacity(16);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        deck2.push_back(Parse::parse(line.as_bytes()));
        line.clear();
    }

    (deck1, deck2)
}
