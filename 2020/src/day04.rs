use std::collections::HashSet;

use aoc_rust::Solution;
use eyre::Result;

macro_rules! parse {
    ($n:ident) => {
        match $n.parse() {
            Ok(n) => n,
            Err(_) => continue,
        }
    };
}

pub fn run(input: &str) -> Result<Solution> {
    let mut p1_count = 0;
    let mut p1 = 0;

    let mut p2_count = 0;
    let mut p2 = 0;

    let mut ecl = HashSet::with_capacity(7);
    ecl.insert("amb");
    ecl.insert("blu");
    ecl.insert("brn");
    ecl.insert("gry");
    ecl.insert("grn");
    ecl.insert("hzl");
    ecl.insert("oth");

    for line in input.lines() {
        if line.trim_end().is_empty() {
            p1 += (p1_count == 7) as u16;
            p2 += (p2_count == 7) as u16;

            p1_count = 0;
            p2_count = 0;
        } else {
            let keys = line
                .split(' ')
                .map(|kvp| unsafe { (kvp.get_unchecked(..2), kvp.get_unchecked(4..).trim_end()) });

            'outer: for (key, value) in keys {
                p1_count += (key != "ci") as u16;

                match key {
                    "by" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 1920 && year <= 2002) as u16;
                    }
                    "iy" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 2010 && year <= 2020) as u16;
                    }
                    "ey" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 2020 && year <= 2030) as u16;
                    }
                    "hg" => {
                        let (num, unit) = value.split_at(value.len() - 2);
                        let num: u8 = parse!(num);

                        let valid = match unit {
                            "cm" => num >= 150 && num <= 193,
                            "in" => num >= 59 && num <= 76,
                            _ => continue,
                        };

                        p2_count += valid as u16;
                    }
                    "hc" => {
                        let value = value.as_bytes();

                        if value.len() != 7 || unsafe { *value.get_unchecked(0) } != b'#' {
                            continue;
                        }

                        for i in 1..7 {
                            let byte = unsafe { *value.get_unchecked(i) };
                            if byte < b'0' || byte > b'f' || (byte > b'9' && byte < b'a') {
                                continue 'outer;
                            }
                        }

                        p2_count += 1;
                    }
                    "ec" => p2_count += ecl.contains(value) as u16,
                    "pi" => p2_count += (value.len() == 9 && value.parse::<u32>().is_ok()) as u16,
                    _ => {}
                }
            }
        }
    }

    Ok(Solution::new().part1(p1).part2(p2))
}
