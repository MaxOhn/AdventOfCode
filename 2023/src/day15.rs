use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn part2(input: &str) -> Result<usize> {
    let mut boxes = [0_u8; 256].map(|_| Vec::new());

    for step in input.split(',') {
        match step.split_once('=') {
            Some((label, value)) => {
                let value: u8 = value.parse().map_err(|_| eyre::eyre!("invalid value"))?;
                let idx = hash(label);

                if let Some((_, focal_len)) = boxes[idx].iter_mut().find(|(a, _)| *a == label) {
                    *focal_len = value;
                } else {
                    boxes[idx].push((label, value));
                }
            }
            None => {
                let label = &step[..step.len() - 1];
                let idx = hash(label);
                boxes[idx].retain(|(a, _)| *a != label);
            }
        }
    }

    let mut sum = 0;

    for (b, i) in boxes.iter().zip(1..) {
        for ((_, focal_len), j) in b.iter().zip(1..) {
            sum += i * j * *focal_len as usize;
        }
    }

    Ok(sum)
}

fn hash(s: &str) -> usize {
    let mut hash = 0;

    for byte in s.bytes() {
        hash += byte as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
