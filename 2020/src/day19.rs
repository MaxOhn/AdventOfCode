use hashbrown::{HashMap, HashSet};

use aoc_rust::Solution;
use eyre::Result;

#[derive(Copy, Clone, Debug)]
struct Rule {
    left: u8,
    right: RightSide,
}

#[derive(Copy, Clone, Debug)]
enum RightSide {
    Terminal(char),
    Nonterminal(u8, u8),
}

pub fn run(input: &str) -> Result<Solution> {
    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

#[allow(dead_code)]
fn part1(input: &str) -> u16 {
    let mut rules = Vec::with_capacity(32);
    let mut substitutions = HashMap::with_capacity(1);

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.len() <= 1 {
            break;
        }

        parse_line(line.as_bytes(), &mut rules, &mut substitutions);
    }

    let mut substitute = || {
        for (left, subs) in substitutions.iter() {
            for i in (0..rules.len()).rev() {
                let rule = get!(rules, i);

                if let RightSide::Nonterminal(first, second) = rule.right {
                    if first == *left {
                        for &sub in subs.iter() {
                            let rule = Rule {
                                left: rule.left,
                                right: RightSide::Nonterminal(sub, second),
                            };

                            rules.push(rule);
                        }
                    }

                    if second == *left {
                        for &sub in subs.iter() {
                            let rule = Rule {
                                left: rule.left,
                                right: RightSide::Nonterminal(first, sub),
                            };

                            rules.push(rule);
                        }
                    }
                }
            }
        }
    };

    substitute();
    substitute();

    let mut valid = 0;

    for line in lines {
        let accepted = unsafe { cyk(line.trim_end(), &rules) };
        valid += accepted as u16;
    }

    valid
}

fn part2(input: &str) -> u16 {
    let mut rules = Vec::with_capacity(32);
    let mut substitutions = HashMap::with_capacity(1);

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.len() <= 1 {
            break;
        }

        if line == "8: 42\n" {
            parse_line(b"8: 42 | 42 8\n", &mut rules, &mut substitutions);
        } else if line == "11: 42 31\n" {
            parse_line(b"11: 42 31\n", &mut rules, &mut substitutions);
            parse_line(b"11: 42 511\n", &mut rules, &mut substitutions);
            parse_line(b"511: 11 31\n", &mut rules, &mut substitutions);
        } else {
            parse_line(line.as_bytes(), &mut rules, &mut substitutions);
        }
    }

    let mut substitute = || {
        for (left, subs) in substitutions.iter() {
            for i in (0..rules.len()).rev() {
                let rule = get!(rules, i);

                if let RightSide::Nonterminal(first, second) = rule.right {
                    if first == *left {
                        for &sub in subs.iter() {
                            let rule = Rule {
                                left: rule.left,
                                right: RightSide::Nonterminal(sub, second),
                            };

                            rules.push(rule);
                        }
                    }

                    if second == *left {
                        for &sub in subs.iter() {
                            let rule = Rule {
                                left: rule.left,
                                right: RightSide::Nonterminal(first, sub),
                            };

                            rules.push(rule);
                        }
                    }
                }
            }
        }
    };

    substitute();
    substitute();

    let mut valid = 0;

    for line in lines {
        let accepted = unsafe { cyk(line.trim_end(), &rules) };
        valid += accepted as u16;
    }

    valid
}

fn parse_line(bytes: &[u8], rules: &mut Vec<Rule>, substitutions: &mut HashMap<u8, HashSet<u8>>) {
    let mut i = 0;
    let mut n = 0;

    loop {
        match get!(bytes, i) {
            b':' => break,
            digit => n = n * 10 + (digit & 0x0F) as u8,
        }

        i += 1;
    }

    i += 2;

    if get!(bytes, i) == b'"' {
        let rule = Rule {
            left: n,
            right: RightSide::Terminal(get!(bytes, i + 1) as char),
        };

        rules.push(rule);
        return;
    }

    parse_right_side(bytes, n, &mut i, rules, substitutions);
}

unsafe fn cyk(word: &str, rules: &[Rule]) -> bool {
    let mut table = Vec::with_capacity(word.len());

    for i in 0..word.len() {
        table.push(vec![HashSet::new(); i + 1]);
    }

    for (i, letter) in word.chars().enumerate() {
        for rule in rules {
            if let RightSide::Terminal(c) = rule.right {
                if c == letter {
                    table
                        .get_unchecked_mut(i)
                        .get_unchecked_mut(i)
                        .insert(rule.left);
                }
            }
        }
    }

    for i in 1..word.len() {
        for j in 0..word.len() - i {
            for k in j..i + j {
                let mut to_add = HashSet::new();

                for &s1 in table.get_unchecked(k).get_unchecked(j).iter() {
                    for &s2 in table.get_unchecked(i + j).get_unchecked(k + 1).iter() {
                        for rule in rules {
                            if let RightSide::Nonterminal(a, b) = rule.right {
                                if a == s1 && b == s2 {
                                    to_add.insert(rule.left);
                                }
                            }
                        }
                    }
                }

                table
                    .get_unchecked_mut(j + i)
                    .get_unchecked_mut(j)
                    .extend(to_add);
            }
        }
    }

    table
        .get_unchecked(word.len() - 1)
        .get_unchecked(0)
        .contains(&0)
}

fn parse_right_side(
    bytes: &[u8],
    left: u8,
    i: &mut usize,
    rules: &mut Vec<Rule>,
    substitutions: &mut HashMap<u8, HashSet<u8>>,
) {
    let mut first = 0;

    loop {
        match get!(bytes, *i) {
            b' ' | b'\n' => break,
            digit => first = first * 10 + (digit & 0x0F) as u8,
        }

        *i += 1;
    }

    if get!(bytes, *i) == b'\n' {
        substitutions
            .entry(left)
            .or_insert_with(HashSet::new)
            .insert(first);

        return;
    }

    *i += 1;

    if get!(bytes, *i) == b'|' {
        substitutions
            .entry(left)
            .or_insert_with(HashSet::new)
            .insert(first);

        *i += 2;

        return parse_right_side(bytes, left, i, rules, substitutions);
    }

    let mut second = 0;

    loop {
        match get!(bytes, *i) {
            b' ' | b'\n' => break,
            digit => second = second * 10 + (digit & 0x0F) as u8,
        }

        *i += 1;
    }

    let rule = Rule {
        left,
        right: RightSide::Nonterminal(first, second),
    };

    rules.push(rule);

    if get!(bytes, *i) == b'\n' {
        return;
    }

    *i += 1;

    if get!(bytes, *i) == b'|' {
        *i += 2;
        parse_right_side(bytes, left, i, rules, substitutions);
    }
}
