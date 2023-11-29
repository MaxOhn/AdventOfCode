use crate::{Error, Solution};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<i32, i32>, Error> {
    let (mut p1, mut p2) = (0, 0);
    let f: fn(i32) -> i32 = |n| n / 3 - 2;
    for line in input.lines() {
        let mut num: i32 = f(line.parse()?);
        p1 += num;
        while num > 0 {
            p2 += num;
            num = f(num);
        }
    }
    Ok(Solution::new(p1, p2))
} // 0.085ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve("14").unwrap(), Solution::new(2, 2));
        assert_eq!(solve("1969").unwrap(), Solution::new(654, 966));
        assert_eq!(solve("100756").unwrap(), Solution::new(33_583, 50_346));
        crate::util::tests::test_full_problem(1, solve, 3_296_560, 4_941_976);
    }
}
