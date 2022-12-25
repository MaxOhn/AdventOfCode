use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = input
        .lines()
        .map(str::as_bytes)
        .map(to_snafu)
        .try_fold(0, |sum, res| res.map(|n| sum + n))
        .map(from_snafu)?;

    Ok(Solution::new().part1(p1).part2("x".to_string()))
}

fn to_snafu(line: &[u8]) -> Result<i64> {
    line.iter().try_fold(0, |n, byte| {
        let term = match byte {
            b'0'..=b'2' => (byte & 0xF) as i64,
            b'-' => -1,
            b'=' => -2,
            _ => bail!("invalid digit `{}`", *byte as char),
        };

        Ok(n * 5 + term)
    })
}

fn from_snafu(mut n: i64) -> String {
    let mut snafu = Vec::new();

    while n > 0 {
        match n % 5 {
            digit @ 0..=2 => {
                snafu.push(digit as u8 + b'0');
            }
            3 => {
                snafu.push(b'=');
                n += 2;
            }
            4 => {
                snafu.push(b'-');
                n += 1;
            }
            _ => unreachable!(),
        }

        n /= 5;
    }

    snafu.reverse();

    unsafe { String::from_utf8_unchecked(snafu) }
}
