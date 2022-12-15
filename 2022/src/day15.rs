use std::{collections::HashSet, mem, str::FromStr};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>>>()?;

    // let p1 = part1::<10>(&sensors)?;
    // let p2 = part2::<20>(&sensors)?;

    let p1 = part1::<2_000_000>(&sensors)?;
    let p2 = part2::<4_000_000>(&sensors)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1<const Y: i32>(sensors: &[Sensor]) -> Result<i32> {
    let mut buf = LinesBuf::default();
    let Lines(lines) = Lines::generate(sensors, Y, &mut buf)?;
    let mut p1 = 0;

    let mut removed_beacons = HashSet::new();

    for line in lines {
        p1 += line.max - line.min + 1;

        for Sensor { pos, beacon, .. } in sensors {
            if (pos.y == Y && pos.x >= line.min && pos.x <= line.max)
                || ((beacon.y == Y && beacon.x >= line.min && beacon.x <= line.max)
                    && removed_beacons.insert(beacon))
            {
                p1 -= 1;
            }
        }
    }

    Ok(p1)
}

fn part2<const MAX: i32>(sensors: &[Sensor]) -> Result<i64> {
    let mut buf = LinesBuf::default();

    for y in 0..=MAX {
        let lines = Lines::generate(sensors, y, &mut buf)?;

        if let Some(x) = lines.first_missing(0, MAX) {
            return Ok(x as i64 * 4_000_000 + y as i64);
        }
    }

    bail!("no matching pos")
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
    radius: i32,
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
            radius: sensor_pos.manhatten_dist(beacon_pos),
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

#[derive(Copy, Clone)]
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

#[derive(Default)]
struct LinesBuf {
    buf: Vec<Line>,
    lines: Vec<Line>,
}

struct Lines<'b>(&'b [Line]);

impl<'b> Lines<'b> {
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

    fn generate(sensors: &[Sensor], y: i32, buf: &'b mut LinesBuf) -> Result<Self> {
        let LinesBuf { buf, lines } = buf;
        lines.clear();

        // Get the range for each sensor on the given y
        for sensor in sensors {
            let Sensor { pos, .. } = sensor;

            let beacon_dist = sensor.radius;
            let y_dist = (pos.y - y).abs();
            let diff = beacon_dist - y_dist;

            if diff < 0 {
                continue;
            }

            let min = pos.x - diff;
            let max = pos.x + diff;

            buf.push(Line { min, max });
        }

        enum Op {
            Noop,
            Replace(usize),
            Stitch(usize),
        }

        // Stitch the lines together through overlaps
        while let Some(new) = buf.pop() {
            let op = lines.iter().enumerate().find_map(|(i, old)| {
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
                    lines.swap_remove(i);
                    buf.push(new);
                }
                Some(Op::Stitch(i)) => {
                    let mut old = lines.swap_remove(i);
                    old.min = old.min.min(new.min);
                    old.max = old.max.max(new.max);
                    buf.push(old);
                }
                None => lines.push(new),
            }
        }

        if lines.len() == 1 {
            return Ok(Self(lines));
        }

        // Combine remaining lines e.g. 2..=4 and 5..=6 becomes 2..=6
        buf.append(lines);

        let mut drain = buf.drain(..);
        let mut prev = drain.next().wrap_err("empty lines")?;

        for next in drain {
            if next.max + 1 == prev.min {
                prev = Line {
                    min: next.min,
                    max: prev.max,
                };
            } else {
                lines.push(mem::replace(&mut prev, next));
            }
        }

        lines.push(prev);
        lines.reverse();

        Ok(Self(lines))
    }
}
