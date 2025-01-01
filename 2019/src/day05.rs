use crate::{computer::Computer, Error, Solution};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<i64, i64>, Error> {
    let mut computer = Computer::new(input)?;
    let p1 = computer
        .insert(1)
        .run()?
        .output_iter()
        .last()
        .ok_or_else(|| error!("No output produced for part 1"))?;
    let mut computer = Computer::new(input)?;
    let p2 = computer
        .insert(5)
        .run()?
        .pop()
        .ok_or_else(|| error!("No output produced for part 2"))?;
    //println!("Memory:\n{:?}", computer.memory);
    Ok(Solution::new(*p1, p2))
} // 0.77ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test05() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut computer = Computer::new(input).unwrap();
        assert_eq!(computer.insert(7).run().unwrap().pop().unwrap(), 1);
        let mut computer = Computer::new(input).unwrap();
        assert_eq!(computer.insert(9).run().unwrap().pop().unwrap(), 0);
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,\
            125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = Computer::new(input).unwrap();
        assert_eq!(computer.insert(7).run().unwrap().pop().unwrap(), 999);
        let mut computer = Computer::new(input).unwrap();
        assert_eq!(computer.insert(8).run().unwrap().pop().unwrap(), 1000);
        let mut computer = Computer::new(input).unwrap();
        assert_eq!(computer.insert(9).run().unwrap().pop().unwrap(), 1001);
        crate::util::tests::test_full_problem(5, solve, 7_265_618, 7_731_427);
    }
}
