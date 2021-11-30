use crate::{Error, Solution};

use std::cmp::Ordering;

pub fn solve(input: String) -> Result<Solution<i32, i32>, Error> {
    let input_split: Vec<i32> = input
        .split('-')
        .map(|n| n.parse().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;
    let (min, max) = (input_split[0], input_split[1]);
    let (mut p1, mut p2) = (0, 0);
    for x in min..=max {
        if check_p1(x) {
            p1 += 1;
            if check_p2(x) {
                p2 += 1;
            }
        }
    }
    Ok(Solution::new(p1, p2))
} // 39.78ms

fn check_p1(n: i32) -> bool {
    let mut has_double = false;
    let mut last = n % 10;
    let mut m = n;
    while m > 0 {
        m /= 10;
        let k = m % 10;
        if k > last {
            return false;
        }
        if k == last {
            has_double = true;
        }
        last = k;
    }
    has_double
}

fn check_p2(n: i32) -> bool {
    let mut has_double = false;
    let mut last = n % 10;
    let mut count = 1;
    let mut m = n;
    while m > 0 {
        m /= 10;
        let k = m % 10;
        match k.cmp(&last) {
            Ordering::Greater => return false,
            Ordering::Equal => count += 1,
            Ordering::Less => {
                if count == 2 {
                    has_double = true;
                }
                count = 1;
            }
        }
        last = k;
    }
    has_double || count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test04() {
        assert!(check_p1(111111));
        assert!(!check_p1(223450));
        assert!(!check_p1(123789));
        assert!(check_p2(112233));
        assert!(!check_p2(123444));
        assert!(check_p2(111122));
        crate::util::tests::test_full_problem(4, solve, 895, 591);
    }
}
