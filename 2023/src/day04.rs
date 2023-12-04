use std::collections::VecDeque;

use aoc_rust::Solution;
use eyre::{ContextCompat, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (_, suffix) = line.split_once(":").wrap_err("missing colon")?;
        let (winning, owned) = suffix.split_once(" | ").wrap_err("missing split")?;

        let winning = winning
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()
            .wrap_err("invalid winning numbers")?;

        let owned = owned
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse::<u8>);

        let mut matches = 0;

        for n in owned {
            matches += winning.contains(&n?) as u8;
        }

        sum += matches.checked_sub(1).map_or(0, |n| 1 << n);
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let original = input
        .lines()
        .map(Card::parse)
        .collect::<Result<Vec<_>>>()
        .wrap_err("invalid cards")?;

    let mut cards_left = original.iter().map(|(id, _)| *id).collect::<VecDeque<_>>();

    let mut total = cards_left.len();

    while let Some(Id(id)) = cards_left.pop_front() {
        let (idx, (_, Matches(matches))) = original
            .iter()
            .enumerate()
            .find(|(_, (Id(card), _))| *card == id)
            .unwrap();

        for (id, _) in original[idx + 1..][..*matches as usize].iter() {
            cards_left.push_back(*id);
            total += 1;
        }
    }

    Ok(total)
}

struct Card;

#[derive(Copy, Clone)]
struct Id(u16);

struct Matches(u8);

impl Card {
    fn parse(line: &str) -> Result<(Id, Matches)> {
        let line = line.trim_start_matches("Card").trim_start();
        let (id, suffix) = line.split_once(":").wrap_err("missing colon")?;
        let (winning, owned) = suffix.split_once(" | ").wrap_err("missing split")?;

        let id = id.parse().map_err(|_| eyre!("invalid id"))?;

        let winning = winning
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()
            .wrap_err("invalid winning numbers")?;

        let owned = owned.split(' ').filter(|n| !n.is_empty()).map(str::parse);

        let mut matches = 0;

        for n in owned {
            matches += winning.contains(&n?) as u8;
        }

        Ok((Id(id), Matches(matches)))
    }
}
