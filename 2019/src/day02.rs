use crate::{Error, Solution};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<usize, usize>, Error> {
    let intcodes: Vec<usize> = input
        .split(',')
        .map(|n| n.parse().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;
    let mut memory = intcodes.clone();
    let p1 = execute(12, 2, &mut memory)?;
    for noun in 0..100 {
        for verb in 0..100 {
            memory = intcodes.clone();
            if execute(noun, verb, &mut memory)? == 19_690_720 {
                return Ok(Solution::new(p1, 100 * noun + verb));
            }
        }
    }
    bail!("No noun-verb combination resulted in the desired solution");
} // 8.52ms

fn execute(noun: usize, verb: usize, memory: &mut [usize]) -> Result<usize, Error> {
    memory[1] = noun;
    memory[2] = verb;
    let mut i = 0;
    loop {
        let (a1, a2, s) = (memory[i + 1], memory[i + 2], memory[i + 3]);
        match memory[i] {
            99 => break Ok(memory[0]),
            1 => memory[s] = memory[a1] + memory[a2],
            2 => memory[s] = memory[a1] * memory[a2],
            other => bail!("Can't process opcode {} in day 2", other),
        }
        i += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test02() {
        let input = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut memory = input.clone();
        assert_eq!(execute(9, 10, &mut memory).unwrap(), 3500);
        crate::util::tests::test_full_problem(2, solve, 3_562_624, 8298);
    }
}
