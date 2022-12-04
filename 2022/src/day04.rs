use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    input
        .lines()
        .try_fold((0, 0), |(mut p1, mut p2), line| {
            let (front, back) = line.split_once(',').wrap_err("missing comma")?;

            let (Some((a, b)), Some((c, d))) = (front.split_once('-'), back.split_once('-')) else {
                return Err(Report::msg("missing dash"));
            };

            let (Ok(a), Ok(b), Ok(c), Ok(d)) = (a.parse(), b.parse(), c.parse(), d.parse()) else {
                return Err(Report::msg("invalid number"));
            };

            p1 += p1_check(a, b, c, d) as usize;
            p2 += p2_check(a, b, c, d) as usize;

            Ok((p1, p2))
        })
        .map(|(p1, p2)| Solution::new().part1(p1).part2(p2))
}

fn p1_check(a: u16, b: u16, c: u16, d: u16) -> bool {
    (a >= c && b <= d) || (c >= a && d <= b)
}

fn p2_check(a: u16, b: u16, c: u16, d: u16) -> bool {
    (a <= c && b >= c) || (c <= a && d >= a) || (b >= d && a <= d) || (d >= b && c <= b)
}
