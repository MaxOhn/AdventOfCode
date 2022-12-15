use std::{collections::HashSet, mem, str::FromStr};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>>>()?;

    // let p1 = part1(&sensors, 10)?;
    // let p2 = part2(&sensors, 20)?;

    let p1 = part1(&sensors, 2_000_000)?;
    let p2 = part2(&sensors, 4_000_000)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(sensors: &[Sensor], target_y: i32) -> Result<i32> {
    let mut buf = Vec::new();
    let Lines(lines) = Lines::generate(sensors, target_y, &mut buf)?;
    let mut p1 = 0;

    let mut removed_beacons = HashSet::new();

    for line in lines {
        p1 += line.max - line.min + 1;

        for Sensor { pos, beacon } in sensors {
            if (pos.y == target_y && pos.x >= line.min && pos.x <= line.max)
                || ((beacon.y == target_y && beacon.x >= line.min && beacon.x <= line.max)
                    && removed_beacons.insert(beacon))
            {
                p1 -= 1;
            }
        }
    }

    Ok(p1)
}

fn part2(sensors: &[Sensor], max: i32) -> Result<i64> {
    let mut buf = Vec::new();

    for y in 0..=max {
        let lines = Lines::generate(sensors, y, &mut buf)?;

        if let Some(x) = lines.first_missing(0, max) {
            return Ok(x as i64 * 4_000_000 + y as i64);
        }
    }

    bail!("no matching pos")
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn radius(&self) -> i32 {
        self.pos.manhatten_dist(self.beacon)
    }
}

impl FromStr for Sensor {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Sensor at ").wrap_err("missing prefix")?;
        let (sensor, rest) = rest.split_once(':').wrap_err("missing colon")?;
        let (sx, sy) = sensor.split_once(", ").wrap_err("missing comma")?;

        let rest = rest
            .strip_prefix(" closest beacon is at ")
            .wrap_err("missing infix")?;

        let (bx, by) = rest.split_once(", ").wrap_err("missing comma")?;

        let sx = sx
            .strip_prefix("x=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid x")?;

        let sy = sy
            .strip_prefix("y=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid y")?;

        let bx = bx
            .strip_prefix("x=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid x")?;

        let by = by
            .strip_prefix("y=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid y")?;

        let sensor_pos = Pos { x: sx, y: sy };
        let beacon_pos = Pos { x: bx, y: by };

        Ok(Sensor {
            pos: sensor_pos,
            beacon: beacon_pos,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn manhatten_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Line {
    min: i32,
    max: i32,
}

impl Line {
    fn contains(self, other: Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(self, other: Self) -> bool {
        (self.min <= other.min && self.max >= other.min)
            || (self.min <= other.max && self.max >= other.max)
            || (other.min <= self.min && other.max >= self.min)
            || (other.min <= self.max && other.max >= self.max)
    }
}

struct Lines(Vec<Line>);

impl Lines {
    fn first_missing(&self, min: i32, max: i32) -> Option<i32> {
        let first = self.0.first()?;

        if first.min > min {
            Some(min)
        } else if first.max < max {
            Some(first.max + 1)
        } else {
            None
        }
    }

    fn generate(sensors: &[Sensor], y: i32, buf: &mut Vec<Line>) -> Result<Self> {
        for sensor in sensors {
            let Sensor { pos, .. } = sensor;

            let beacon_dist = sensor.radius();
            let y_dist = (pos.y - y).abs();
            let diff = beacon_dist - y_dist;

            if diff < 0 {
                continue;
            }

            let min = pos.x - diff;
            let max = pos.x + diff;

            let line = Line { min, max };
            buf.push(line);
        }

        let mut stitched: Vec<Line> = Vec::new();

        enum Op {
            Noop,
            Replace(usize),
            Stitch(usize),
        }

        while let Some(new) = buf.pop() {
            let op = stitched.iter().enumerate().find_map(|(i, old)| {
                if old.contains(new) {
                    Some(Op::Noop)
                } else if new.contains(*old) {
                    Some(Op::Replace(i))
                } else if old.overlap(new) {
                    Some(Op::Stitch(i))
                } else {
                    None
                }
            });

            match op {
                Some(Op::Noop) => {}
                Some(Op::Replace(i)) => {
                    stitched.swap_remove(i);

                    buf.push(new);
                }
                Some(Op::Stitch(i)) => {
                    let mut old = stitched.swap_remove(i);
                    old.min = old.min.min(new.min);
                    old.max = old.max.max(new.max);

                    buf.push(old);
                }
                None => stitched.push(new),
            }
        }

        if stitched.len() == 1 {
            return Ok(Self(stitched));
        }

        buf.append(&mut stitched);

        let mut drain = buf.drain(..);
        let mut prev = drain.next().wrap_err("empty lines")?;

        for next in drain {
            if next.max + 1 == prev.min {
                prev = Line {
                    min: next.min,
                    max: prev.max,
                };
            } else {
                stitched.push(mem::replace(&mut prev, next));
            }
        }

        stitched.push(prev);
        stitched.sort_unstable_by_key(|line| line.min);

        Ok(Self(stitched))
    }
}
