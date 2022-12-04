use crate::prelude::Solution;

pub fn run(input: &str) -> Solution {
    let (p1, p2) = input.lines().fold((0, 0), |(mut p1, mut p2), line| {
        let (front, back) = line.split_once(',').unwrap();

        let (a, b) = front.split_once('-').unwrap();
        let (c, d) = back.split_once('-').unwrap();

        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        let c: u32 = c.parse().unwrap();
        let d: u32 = d.parse().unwrap();

        p1 += ((a >= c && b <= d) || (c >= a && d <= b)) as usize;
        p2 += ((a <= c && b >= c) || (c <= a && d >= a) || (b >= d && a <= d) || (d >= b && c <= b))
            as usize;

        (p1, p2)
    });

    Solution::new().part1(p1).part2(p2)
}
