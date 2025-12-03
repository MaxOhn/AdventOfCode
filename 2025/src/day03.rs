use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    solve::<2>(input)
}

fn part2(input: &str) -> u64 {
    solve::<12>(input)
}

fn solve<const N: usize>(input: &str) -> u64 {
    let mut sum = 0;

    for bank in input.lines() {
        let mut remaining = bank;
        let mut joltage = 0;

        for i in (0..N).rev() {
            let (j, max) = remaining[..remaining.len() - i]
                .bytes()
                .enumerate()
                .rev() // `max_by_key` takes the *last* biggest
                .max_by_key(|(_, joltage)| *joltage)
                .unwrap();

            remaining = &remaining[j + 1..];
            joltage *= 10;
            joltage += (max as u8 - b'0') as u64;
        }

        sum += joltage;
    }

    sum
}
