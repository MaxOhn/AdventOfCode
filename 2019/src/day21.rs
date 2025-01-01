use crate::{computer::Computer, Error, Solution};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<i64, i64>, Error> {
    let mut computer = Computer::new(input)?;
    computer
        // !C && D
        .insert_not('C', 'J')
        .insert_and('D', 'J')
        // !A
        .insert_not('A', 'T')
        .insert_or('T', 'J')
        .insert_walk()
        .run()?;
    let p1 = computer
        .output_iter()
        .last()
        .ok_or_else(|| error!("Computer did not prodocude an output for part1"))?;
    let mut computer = Computer::new(input)?;
    computer
        // !B && !E && D
        .insert_not('B', 'J')
        .insert_not('E', 'T')
        .insert_and('D', 'T')
        .insert_and('T', 'J')
        // !A
        .insert_not('A', 'T')
        .insert_or('T', 'J')
        // !C && D && E
        .insert_not('C', 'T')
        .insert_and('D', 'T')
        .insert_and('E', 'T')
        .insert_or('T', 'J')
        // !C && D && H
        .insert_not('C', 'T')
        .insert_and('D', 'T')
        .insert_and('H', 'T')
        .insert_or('T', 'J')
        .insert_run()
        .run()?;
    let p2 = computer
        .output_iter()
        .last()
        .ok_or_else(|| error!("Computer did not prodocude an output for part2"))?;
    Ok(Solution::new(*p1, *p2))
} // 275.18ms

trait SpringScript {
    fn insert_not(&mut self, i1: char, i2: char) -> &mut Self;
    fn insert_and(&mut self, i1: char, i2: char) -> &mut Self;
    fn insert_or(&mut self, i1: char, i2: char) -> &mut Self;
    fn insert_walk(&mut self) -> &mut Self;
    fn insert_run(&mut self) -> &mut Self;
    #[allow(unused)]
    fn print_output(&mut self) -> &mut Self;
}

impl SpringScript for Computer {
    fn insert_not(&mut self, i1: char, i2: char) -> &mut Self {
        self.insert(78)
            .insert(79)
            .insert(84)
            .insert(32)
            .insert(i1 as i64)
            .insert(32)
            .insert(i2 as i64)
            .insert(10)
    }

    fn insert_and(&mut self, i1: char, i2: char) -> &mut Self {
        self.insert(65)
            .insert(78)
            .insert(68)
            .insert(32)
            .insert(i1 as i64)
            .insert(32)
            .insert(i2 as i64)
            .insert(10)
    }

    fn insert_or(&mut self, i1: char, i2: char) -> &mut Self {
        self.insert(79)
            .insert(82)
            .insert(32)
            .insert(i1 as i64)
            .insert(32)
            .insert(i2 as i64)
            .insert(10)
    }

    fn insert_walk(&mut self) -> &mut Self {
        self.insert(87).insert(65).insert(76).insert(75).insert(10)
    }

    fn insert_run(&mut self) -> &mut Self {
        self.insert(82).insert(85).insert(78).insert(10)
    }

    fn print_output(&mut self) -> &mut Self {
        while let Some(output) = self.pop() {
            print!("{}", output as u8 as char)
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test21() {
        crate::util::tests::test_full_problem(21, solve, 19349939, 1142412777);
    }
}
