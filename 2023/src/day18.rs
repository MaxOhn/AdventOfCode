use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    // let p1 = 0;
    let p2 = part2(input);
    // let p2 = 0;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut x = 0;
    let mut y = 0;

    let mut points = Vec::new();
    points.push((x, y));

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let count = split.next().unwrap().parse::<i32>().unwrap();

        let _c = split
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')');

        // for _ in 1..=count {
        match dir {
            "U" => y -= count,
            "D" => y += count,
            "L" => x -= count,
            "R" => x += count,
            _ => unreachable!("{dir}"),
        }

        points.push((x, y));
        // }
    }

    polygon_area(&points)
}

fn polygon_area(points: &[Point]) -> u64 {
    let mut area = 0;
    let mut circum = 0;

    for window in points.windows(2) {
        let [(x1, y1), (x2, y2)] = window else {
            unreachable!()
        };

        let u = (y1 + y2) as i64;
        let v = (x1 - x2) as i64;
        area += u * v;

        // println!("{},{} ~ {},{} => {} * {} = {}", x1, y1, x2, y2, u, v, u * v);

        circum += ((y1 - y2) + (x1 - x2)).abs() as i64;
    }

    ((area.abs() + circum) / 2) as u64
}

fn flood_fill(points: &mut Points) -> u64 {
    let min_x = points.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let edge = Edge {
        min_x,
        max_x,
        min_y,
        max_y,
    };

    let mut new = Points::default();
    let mut seen = Points::default();
    let mut stack = Vec::new();

    for x in min_x + 1..max_x {
        for y in min_y + 1..max_y {
            if fill(x, y, &mut new, &points, &seen, &edge, &mut stack) {
                points.extend(new.drain());
            } else {
                seen.extend(new.drain());
            }
        }
    }

    points.len() as u64
}

type Point = (i32, i32);
type Points = fxhash::FxHashSet<Point>;

#[derive(Debug)]
struct Edge {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn fill(
    x: i32,
    y: i32,
    new: &mut Points,
    points: &Points,
    seen: &Points,
    edge: &Edge,
    stack: &mut Vec<Point>,
) -> bool {
    stack.clear();
    stack.push((x, y));

    while let Some((x, y)) = stack.pop() {
        if x < edge.min_x
            || x > edge.max_x
            || y < edge.min_y
            || y > edge.max_y
            || seen.contains(&(x, y))
        {
            return false;
        } else if points.contains(&(x, y)) || !new.insert((x, y)) {
            continue;
        }

        stack.push((x + 1, y));
        stack.push((x - 1, y));
        stack.push((x, y + 1));
        stack.push((x, y - 1));
    }

    true
}

fn part2(input: &str) -> u64 {
    let mut x = 0;
    let mut y = 0;

    let mut points = Vec::new();
    points.push((x, y));

    for line in input.lines() {
        let mut split = line.split(' ');
        let _dir = split.next().unwrap();
        let _count = split.next().unwrap().parse::<i32>().unwrap();

        let c: [u8; 6] = split
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')')
            .as_bytes()
            .try_into()
            .unwrap();

        let dir = match c[5] {
            b'0' => "R",
            b'1' => "D",
            b'2' => "L",
            b'3' => "U",
            _ => unreachable!(),
        };

        let count = i32::from_str_radix(std::str::from_utf8(&c[..5]).unwrap(), 16).unwrap();

        println!("{line:?}: {dir} {count}");

        // for _ in 1..=count {
        match dir {
            "U" => y -= count,
            "D" => y += count,
            "L" => x -= count,
            "R" => x += count,
            _ => unreachable!("{dir}"),
        }

        points.push((x, y));
        // }-
    }

    polygon_area(&points)
}
