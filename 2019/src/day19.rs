use crate::{computer::Computer, Error, Solution};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<usize, i64>, Error> {
    let mut p1 = 0;
    for x in 0..50 {
        for y in 0..50 {
            if gets_pulled(input, x, y)? {
                p1 += 1;
            }
        }
    }
    let mut x = 3;
    let mut y = 4;
    loop {
        while gets_pulled(input, x, y)? {
            x += 1;
        }
        if x > 99 && gets_pulled(input, x - 100, y + 99)? {
            return Ok(Solution::new(p1, (x - 100) * 10_000 + y));
        }
        y += 1;
    }
} // 1.4s

fn gets_pulled(input: &str, x: i64, y: i64) -> Result<bool, Error> {
    let mut drone = Computer::new(input)?;
    Ok(drone
        .insert(x)
        .insert(y)
        .run()?
        .pop()
        .ok_or_else(|| error!("Expected output for beam feedback, none found"))?
        == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test19() {
        crate::util::tests::test_full_problem(19, solve, 189, 7621042);
    }
}
