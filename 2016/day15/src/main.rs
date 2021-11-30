use std::time::Instant;

use util::Parse;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let mut residues = Vec::new();
    let mut modulii = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut words = line.split(' ');

        let m = i32::parse(words.nth(3).unwrap().as_bytes());
        let last = words.last().unwrap().as_bytes();
        let pos = i32::parse(&last[..last.len() - 1]);

        modulii.push(m);
        residues.push(m - pos - i as i32 - 1);
    }

    let start = Instant::now();
    let p1 = part1(&residues, &modulii);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&mut residues, &mut modulii);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 400_589);
    assert_eq!(p2, 3_045_959);
}

fn part1(residues: &[i32], modulii: &[i32]) -> i32 {
    chinese_remainder(residues, modulii).unwrap()
}

fn part2(residues: &mut Vec<i32>, modulii: &mut Vec<i32>) -> i32 {
    let r = 10 - residues.len() as i32;
    residues.push(r);
    modulii.push(11);

    chinese_remainder(residues, modulii).unwrap()
}

fn chinese_remainder(residues: &[i32], modulii: &[i32]) -> Option<i32> {
    let prod: i32 = modulii.iter().product();
    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = egcd(x, n);

    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);

        (g, y - (b / a) * x, x)
    }
}
