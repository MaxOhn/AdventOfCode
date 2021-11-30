use crate::{Error, Solution};

use std::cell::Cell;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let input: Vec<i32> = input
        .lines()
        .map(|line| line.parse().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;
    let p1 = input.iter().sum();
    let p2 = Cell::new(0);
    let mut seen = HashSet::new();
    input
        .iter()
        .cycle()
        .take_while(|_| seen.insert(p2.get()))
        .for_each(|n| p2.set(p2.get() + n));
    Ok(Solution::new(p1, p2.get()))
} // 236.41ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            solve(String::from("+1\n-2\n+3\n+1")).unwrap(),
            Solution::new(3, 2)
        );
    }
}
