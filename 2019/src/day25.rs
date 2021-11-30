use crate::{
    Solution,
    Error,
    computer::Computer,
};

use std::io::{self, BufRead};

pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let mut robot = Computer::new(input)?;
    robot.run()?.print();
    /* Play yourself
    // Take sand, fixed point, wreath, and space law space brochure
    while robot.get_instruction() {
        robot.run()?.print();
    }
    */
    robot
        .write_instruction("east")?
        .write_instruction("take sand")?
        .write_instruction("west")?
        .write_instruction("west")?
        .write_instruction("north")?
        .write_instruction("take wreath")?
        .write_instruction("east")?
        .write_instruction("take fixed point")?
        .write_instruction("west")?
        .write_instruction("south")?
        .write_instruction("south")?
        .write_instruction("east")?
        .write_instruction("east")?
        .write_instruction("east")?
        .write_instruction("take space law space brochure")?
        .write_instruction("south")?
        .write_instruction("south")?
        .write_instruction("west")?
        .run()?
        .print(); // Answer: 16778274
    Ok(Solution::new(0, 0))
} // 73.37ms

trait ShipRobot {
    fn get_instruction(&mut self) -> bool;
    fn write_instruction(&mut self, instruction: &str) -> Result<&mut Self, Error>;
    fn print(&mut self);
}

impl ShipRobot for Computer {
    fn write_instruction(&mut self, instruction: &str) -> Result<&mut Self, Error> {
        for c in instruction.chars() {
            self.insert(c as i64);
        }
        self.insert(10).run()
    }

    fn print(&mut self) {
        while let Some(output) = self.pop() {
            print!("{}", output as u8 as char);
        }
    }

    fn get_instruction(&mut self) -> bool {
        let mut line = String::new();
        let stdin = io::stdin();
        if stdin.lock().read_line(&mut line).is_err() || line.trim() == "break" {
            return false;
        }
        for c in line.trim().chars() {
            self.insert(c as i64);
        }
        self.insert(10);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test25() {
        crate::util::tests::test_full_problem(25, solve, 0, 0);
    }
}