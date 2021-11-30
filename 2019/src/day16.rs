use crate::{Error, Solution};

pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let signal = parse_input(input)?;
    let p1 = solve_part1(signal.clone());
    let p2 = solve_part2(signal)?;
    Ok(Solution::new(p1, p2))
} // 6.07s

fn parse_input(input: String) -> Result<Vec<i32>, Error> {
    Ok(input
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| error!("Can not parse {} to digit", c))
        })
        .collect::<Result<Vec<_>, Error>>()? // TODO: Nicer way?
        .into_iter()
        .map(|d| d as i32)
        .collect::<Vec<_>>())
}

fn solve_part1(mut signal: Vec<i32>) -> i32 {
    let len = signal.len();
    let mut next = vec![0; len];
    for _ in 0..100 {
        for i in 1..=len {
            let mut sum = 0;
            for (j, s) in signal.iter().enumerate() {
                let pattern_elem = match (j + 1) % (4 * i) {
                    x if x < i => continue,
                    x if x < 2 * i => 1,
                    x if x < 3 * i => continue,
                    _ => -1,
                };
                sum += s * pattern_elem;
            }
            next[i - 1] = sum.abs() % 10;
        }
        signal.clone_from_slice(&next[..len]);
    }
    signal
        .into_iter()
        .take(8)
        .fold(0, |sum, next| 10 * sum + next)
}

fn solve_part2(mut signal: Vec<i32>) -> Result<i32, Error> {
    let offset = signal[..7].iter().fold(0, |sum, &next| 10 * sum + next) as usize;
    if offset < signal.len() / 2 {
        bail!(
            "offset ({}) must be at least signal.len() / 2 ({}) for part2 to work",
            offset,
            signal.len() / 2
        );
    }
    let len = signal.len() * 10_000 - offset;
    signal = signal.into_iter().rev().cycle().take(len).collect();
    for _ in 0..100 {
        signal = signal
            .iter()
            .scan(0, |sum, curr| {
                *sum += curr;
                Some(*sum % 10)
            })
            .collect();
    }
    Ok(signal
        .into_iter()
        .rev()
        .take(8)
        .fold(0, |sum, next| 10 * sum + next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test16() {
        let signal = parse_input("80871224585914546619083218645595".to_owned()).unwrap();
        assert_eq!(solve_part1(signal), 24_176_176);
        let signal = parse_input("19617804207202209144916044189917".to_owned()).unwrap();
        assert_eq!(solve_part1(signal), 73_745_418);
        let signal = parse_input("69317163492948606335995924319873".to_owned()).unwrap();
        assert_eq!(solve_part1(signal), 52_432_133);
        let signal = parse_input("03036732577212944063491565474664".to_owned()).unwrap();
        assert_eq!(solve_part2(signal).unwrap(), 84_462_026);
        let signal = parse_input("02935109699940807407585447034323".to_owned()).unwrap();
        assert_eq!(solve_part2(signal).unwrap(), 78_725_270);
        let signal = parse_input("03081770884921959731165446850517".to_owned()).unwrap();
        assert_eq!(solve_part2(signal).unwrap(), 53_553_731);
        crate::util::tests::test_full_problem(16, solve, 36627552, 79723033);
    }
}
