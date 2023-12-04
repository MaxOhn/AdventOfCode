use aoc_rust::Solution;
use eyre::{ContextCompat, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.trim_start_matches("Card").trim_start();
        let (_, suffix) = line.split_once(":").wrap_err("missing colon")?;
        let (winning, mine) = suffix.split_once(" | ").wrap_err("missing split")?;

        let winning = winning
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()
            .wrap_err("invalid number")?;

        let mine = mine
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(str::parse::<u8>);

        let mut matches = 0;

        for n in mine {
            matches += winning.contains(&n?) as u8;
        }

        sum += matches.checked_sub(1).map_or(0, |n| 1 << n);
    }

    Ok(sum)
}

fn part2(input: &str) -> usize {
    let mut cards = Vec::new();

    for line in input.lines() {
        let line = line.strip_prefix("Card").unwrap();
        let (id, suffix) = line.split_once(":").unwrap();
        let id = id.trim().parse::<u32>().unwrap();
        let (winning, mine) = suffix.split_once(" | ").unwrap();

        let winning = winning
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mine = mine
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        cards.push((id, winning, mine));
    }

    let mut all = cards.clone();

    let mut i = 0;

    while i < all.len() {
        let (id, winning, mine) = &all[i];

        let mut matches = 0;

        for n in mine {
            if winning.contains(n) {
                matches += 1;
            }
        }

        let idx = cards.iter().position(|(i, ..)| i == id).unwrap();

        for card in cards[idx + 1..][..matches].iter() {
            all.push(card.to_owned());
        }

        i += 1;
    }

    all.len()
}
