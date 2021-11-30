use crate::{Error, Solution};

use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve(input: String) -> Result<Solution<i32, String>, Error> {
    let (mut twice, mut thrice) = (0, 0);
    for line in input.lines() {
        let mut characters = HashMap::with_capacity(26);
        line.chars().for_each(|c| {
            *characters.entry(c).or_insert(0) += 1;
        });
        if characters.values().any(|v| *v == 2) {
            twice += 1;
        }
        if characters.values().any(|v| *v == 3) {
            thrice += 1;
        }
    }
    for (i, w1) in input.lines().enumerate() {
        for w2 in input.lines().skip(i + 1) {
            if w1.chars().zip(w2.chars()).filter(|(a, b)| a != b).count() == 1 {
                let p2 = w1
                    .chars()
                    .zip(w2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();
                return Ok(Solution::new(twice * thrice, p2));
            }
        }
    }
    bail!("Found no matching words");
} // 114.77ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test02() {
        assert_eq!(
            solve(String::from(
                "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"
            ))
            .unwrap()
            .part1,
            12
        );
        assert_eq!(
            solve(String::from(
                "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"
            ))
            .unwrap()
            .part2,
            "fgij"
        );
    }
}
