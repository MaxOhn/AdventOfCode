use std::collections::HashSet;
use std::fmt;
use std::io::{stdout, Write};
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let start = Instant::now();
    let p1 = part1(&input);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&input);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, "c6697b55");
    assert_eq!(p2, "8c35d1ab");
}

fn part1(input: &str) -> String {
    let mut password = Password([None; 8]);
    let mut i = 0;
    let mut j = 0;

    while j < 8 {
        let hash = md5::compute(format!("{}{}", input, i));

        if i % 1_000 == 0 {
            password.intermediate_print(&hash.0, 1);
        }

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            password.0[j] = Some(hash[2]);
            j += 1;
        }

        i += 1;
    }

    let _ = stdout().write(b"\r");

    format!("{:x}", password)
}

fn part2(input: &str) -> String {
    let mut password = Password([None; 8]);
    let mut i = 0;
    let mut missing: HashSet<_> = (0..8).collect();

    while !missing.is_empty() {
        let hash = md5::compute(format!("{}{}", input, i));

        if i % 1_000 == 0 {
            password.intermediate_print(&hash.0, 2);
        }

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 8 {
            let idx = hash[2] as usize;

            if missing.remove(&idx) {
                password.0[idx] = Some(hash[3] >> 4);
            }
        }

        i += 1;
    }

    let _ = stdout().write(b"\r");

    format!("{:x}", password)
}

struct Password([Option<u8>; 8]);

impl Password {
    fn intermediate_print(&self, filler: &[u8], part: u8) {
        let mut stdout = stdout();

        let _ = write!(stdout, "\rPart {}: ", part);

        for (i, &value) in self.0.iter().enumerate() {
            let val = value.unwrap_or(filler[(8 + i)] % 16);
            let _ = write!(stdout, "{:x}", val);
        }
    }
}

impl fmt::LowerHex for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &value in &self.0 {
            match value {
                Some(val) => write!(f, "{:x}", val)?,
                None => f.write_str("-")?,
            }
        }

        Ok(())
    }
}
