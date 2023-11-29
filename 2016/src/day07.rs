use std::collections::HashMap;

use aoc_rust::Solution;

pub fn run(input: &str) -> eyre::Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();

            let mut found_outside = false;
            let mut in_brackets = false;
            let mut i = 3;

            while i < bytes.len() {
                let byte = get!(bytes, i);

                if byte == b'[' {
                    in_brackets = true;
                    i += 4;
                    continue;
                }

                if byte == b']' {
                    in_brackets = false;
                    i += 4;
                    continue;
                }

                if found_outside && !in_brackets
                    || byte == get!(bytes, i - 1)
                    || get!(bytes, i - 1) != get!(bytes, i - 2)
                    || byte != get!(bytes, i - 3)
                {
                    i += 1;
                    continue;
                }

                if in_brackets {
                    return false;
                }

                found_outside = true;
            }

            found_outside
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut letters = HashMap::new();
    input
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();

            let mut in_brackets = false;
            let mut i = 2;

            while i < bytes.len() {
                let byte = get!(bytes, i);

                if byte == b'[' {
                    in_brackets = true;
                    i += 3;
                    continue;
                }

                if byte == b']' {
                    in_brackets = false;
                    i += 3;
                    continue;
                }

                if byte == get!(bytes, i - 1) || byte != get!(bytes, i - 2) {
                    i += 1;
                    continue;
                }

                let (key, val) = if in_brackets {
                    ((get!(bytes, i - 1), byte), 2)
                } else {
                    ((byte, get!(bytes, i - 1)), 1)
                };

                *letters.entry(key).or_insert(0) |= val;
                i += 1;
            }

            let valid = letters.values().any(|val: &u8| *val == 3);
            letters.clear();

            valid
        })
        .count()
}
