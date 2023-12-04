use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.strip_prefix("Card").unwrap();
        let (id, suffix) = line.split_once(":").unwrap();
        let _id = id.trim().parse::<u32>().unwrap();
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

        let mut points = 0;

        for n in mine {
            if winning.contains(&n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    }

    sum
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
