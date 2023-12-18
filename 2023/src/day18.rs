use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u64> {
    let points = input.lines().filter_map(|line| {
        let mut split = line.split(' ');

        split.next().zip(split.next()?.parse().ok())
    });

    polygon_area(points)
}

fn part2(input: &str) -> Result<u64> {
    let points = input.lines().filter_map(|line| {
        line.rsplit(' ')
            .next()
            .and_then(|color| color.strip_prefix("(#")?.strip_suffix(')'))
            .and_then(|color| {
                let len = i64::from_str_radix(color.get(..5)?, 16).ok()?;

                let dir = match color.get(5..)? {
                    "0" => "R",
                    "1" => "D",
                    "2" => "L",
                    "3" => "U",
                    _ => return None,
                };

                Some((dir, len))
            })
    });

    polygon_area(points)
}

fn polygon_area<'a, I: Iterator<Item = (&'a str, i64)>>(points: I) -> Result<u64> {
    let mut x = 0;
    let mut y = 0;

    let mut area = 0;
    let mut circum = 0;

    for (direction, len) in points {
        let mut nx = x;
        let mut ny = y;

        match direction {
            "R" => nx += len,
            "L" => nx -= len,
            "U" => ny -= len,
            "D" => ny += len,
            _ => eyre::bail!("invalid direction `{direction}`"),
        }

        area += (y + ny) * (x - nx);
        circum += len;

        x = nx;
        y = ny;
    }

    Ok(((area.abs() + circum) / 2 + 1) as u64)
}
