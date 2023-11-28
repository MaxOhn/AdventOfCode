use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let mut residues = Vec::new();
    let mut modulii = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut words = line.split(' ');

        let m = words.nth(3).unwrap().parse().unwrap();
        let last = words.last().unwrap();
        let pos = last[..last.len() - 1].parse::<i32>().unwrap();

        modulii.push(m);
        residues.push(m - pos - i as i32 - 1);
    }

    let start = Instant::now();
    let p1 = aoc16_day15::part1(&residues, &modulii);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = aoc16_day15::part2(&mut residues, &mut modulii);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 400_589);
    assert_eq!(p2, 3_045_959);
}
