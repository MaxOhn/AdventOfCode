use std::collections::HashMap;

use util::Parse;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let start = std::time::Instant::now();
    let p1 = part1(&input);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = std::time::Instant::now();
    let p2 = part2(&input);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 185_371);
    assert_eq!(p2, 984);
}

fn part1(input: &str) -> usize {
    let mut counts = HashMap::with_capacity(26);

    input
        .lines()
        .filter_map(|line| parse_line(line, &mut counts))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut counts = HashMap::with_capacity(26);

    input
        .lines()
        .filter_map(|line| {
            let id = parse_line(line, &mut counts)?;

            Some((&line[..line.len() - 11], id))
        })
        .find_map(|(line, id)| {
            for (c, required) in line.chars().zip("northpole object storage".chars()) {
                let decrypted = match c {
                    '-' => ' ',
                    _ => ((((c as u8 - b'a') as usize + id) % 26) as u8 + b'a') as char,
                };

                if decrypted != required {
                    return None;
                }
            }

            Some(id)
        })
        .unwrap()
}

fn parse_line(line: &str, counts: &mut HashMap<char, u8>) -> Option<usize> {
    let checksum = &line[line.len() - 6..line.len() - 1];

    let mut names = line[..line.len() - 7].split('-');

    let id = names.next_back().unwrap();

    for name in names {
        for c in name.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_unstable_by(|(l1, c1), (l2, c2)| c2.cmp(c1).then_with(|| l1.cmp(l2)));

    let valid = sorted
        .into_iter()
        .zip(checksum.chars())
        .take_while(|((&order, _), required)| order == *required)
        .count();

    counts.clear();

    if valid == 5 {
        Some(usize::parse(id.as_bytes()))
    } else {
        None
    }
}
