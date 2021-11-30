use crate::{computer::Computer, Error, Solution};

use itertools::Itertools;

pub fn solve(input: String) -> Result<Solution<i64, i64>, Error> {
    let p1 = solve_part1(input.clone())?;
    let p2 = solve_part2(input)?;
    Ok(Solution::new(p1, p2))
} // 230.73ms

fn solve_part1(input: String) -> Result<i64, Error> {
    let mut max_signal = 0;
    for phases in (0..5).permutations(5) {
        let mut amplifiers: Vec<Computer> = phases
            .iter()
            .map(|&phase| {
                let mut computer = Computer::new(input.clone())?;
                computer.insert(phase);
                Ok(computer)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut signal = 0;
        for amplifier in &mut amplifiers {
            amplifier.insert(signal).run()?;
            signal = amplifier
                .pop()
                .ok_or_else(|| error!("Expected output from ampifier, none found"))?;
        }
        max_signal = max_signal.max(signal);
    }
    Ok(max_signal)
}

fn solve_part2(input: String) -> Result<i64, Error> {
    let mut max_signal = 0;
    for phases in (5..10).permutations(5) {
        let mut amplifiers: Vec<Computer> = phases
            .iter()
            .map(|&phase| {
                let mut computer = Computer::new(input.clone())?;
                computer.insert(phase);
                Ok(computer)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut idx = 0;
        let mut signal = 0;
        loop {
            amplifiers[idx].insert(signal).run()?;
            match amplifiers[idx].pop() {
                Some(output) => signal = output,
                None => break,
            }
            max_signal = max_signal.max(signal);
            idx = (idx + 1) % 5;
        }
    }
    Ok(max_signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test07() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_owned();
        assert_eq!(solve_part1(input).unwrap(), 43210);
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_owned();
        assert_eq!(solve_part2(input).unwrap(), 139629729);
        crate::util::tests::test_full_problem(7, solve, 65464, 1518124);
    }
}
