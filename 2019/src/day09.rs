use crate::{computer::Computer, Error, Solution};

use itertools::Itertools;

pub fn solve(input: String) -> Result<Solution<String, String>, Error> {
    let p1 = solve_with_input(input.clone(), Some(1))?;
    let p2 = solve_with_input(input, Some(2))?;
    Ok(Solution::new(p1, p2))
}

fn solve_with_input(input: String, computer_input: Option<i64>) -> Result<String, Error> {
    let mut computer = Computer::new(input)?;
    computer.run()?;
    if let Some(input) = computer_input {
        computer.insert(input).run()?;
    }
    let result = computer
        .output_iter()
        .map(|output| output.to_string())
        .join(",");
    Ok(result)
} // 103.28ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test09() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_owned();
        let answer = solve_with_input(input.clone(), None).unwrap();
        assert_eq!(input, answer);
        let input = "1102,34915192,34915192,7,4,7,99,0".to_owned();
        let answer = solve_with_input(input, None)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        assert!(1_000_000_000_000_000 <= answer && answer < 10_000_000_000_000_000);
        let input = "104,1125899906842624,99".to_owned();
        let answer = solve_with_input(input, None)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        assert_eq!(1_125_899_906_842_624, answer);
        crate::util::tests::test_full_problem(
            9,
            solve,
            3_345_854_957_i64.to_string(),
            68_938.to_string(),
        );
    }
}
