use std::{collections::HashSet, mem, str::FromStr};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>>>()?;

    // let p1 = part1::<10>(&sensors)?;
    // let p2 = part2_border_intersection::<20>(&sensors)?;

    let p1 = part1::<2_000_000>(&sensors)?;
    // let p2 = part2_quadrants::<4_000_000>(&sensors)?;
    let p2 = part2_border_intersection::<4_000_000>(&sensors)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1<const Y: i32>(sensors: &[Sensor]) -> Result<i32> {
    let lines = Lines::generate(sensors, Y)?;
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

#[allow(unused)]
fn part2_quadrants<const MAX: i32>(sensors: &[Sensor]) -> Result<i64> {
    let mut stack = vec![Quadrant {
        top_l: Pos::new(0, 0),
        bot_r: Pos::new(MAX, MAX),
    }];

    while let Some(Quadrant { top_l, bot_r }) = stack.pop() {
        if top_l == bot_r {
            return Ok(top_l.tuning_frequency());
        }

        let midx = top_l.x + (bot_r.x - top_l.x) / 2;
        let midy = top_l.y + (bot_r.y - top_l.y) / 2;

        let next_top_l = Quadrant {
            top_l,
            bot_r: Pos::new(midx, midy),
        };

        let next_top_r = Quadrant {
            top_l: Pos::new(midx + 1, top_l.y),
            bot_r: Pos::new(bot_r.x, midy),
        };

        let next_bot_l = Quadrant {
            top_l: Pos::new(top_l.x, midy + 1),
            bot_r: Pos::new(midx, bot_r.y),
        };

        let next_bot_r = Quadrant {
            top_l: Pos::new(midx + 1, midy + 1),
            bot_r,
        };

        for quadrant in [next_top_l, next_top_r, next_bot_l, next_bot_r] {
            if sensors.iter().all(|sensor| quadrant.valid(sensor)) {
                stack.push(quadrant);
            }
        }
    }

    bail!("no matching pos")
}
#[allow(unused)]
fn part2_border_intersection<const MAX: i32>(sensors: &[Sensor]) -> Result<i64> {
    let mut verticals = Vec::with_capacity(sensors.len() * 2);
    let mut horizontals = Vec::with_capacity(sensors.len() * 2);

    for Sensor { pos, radius, .. } in sensors {
        let xleft = pos.x - radius - 1;
        let xright = pos.x + radius + 1;

        let left = xleft - pos.y;
        let right = xright - pos.y;

        let top = xleft + pos.y;
        let bot = xright + pos.y;

        verticals.extend([left, right]);
        horizontals.extend([top, bot]);
    }

    verticals.sort_unstable();
    horizontals.sort_unstable();

    let mut buf = Vec::with_capacity(verticals.len() / 2);

    for window in verticals.windows(2) {
        let [a, b] = window else { unreachable!() };

        if a == b {
            buf.push(*a);
        }
    }

    mem::swap(&mut buf, &mut verticals);
    buf.clear();

    for window in horizontals.windows(2) {
        let [a ,b] = window else { unreachable!() };

        if a == b {
            buf.push(*a);
        }
    }

    mem::swap(&mut buf, &mut horizontals);

    verticals.dedup();
    horizontals.dedup();

    for v in verticals {
        for h in horizontals.iter() {
            let y = (h - v) / 2;
            let x = v + y;

            if x < 0 || x > MAX || y < 0 || y > MAX {
                continue;
            }

            let intersection = Pos::new(x, y);

            if sensors.iter().all(|s| !intersection.is_in_range(s)) {
                return Ok(intersection.tuning_frequency());
            }
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

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Sensor at ").wrap_err("missing prefix")?;
        let (sensor, rest) = rest.split_once(':').wrap_err("missing colon")?;

        let beacon = rest
            .strip_prefix(" closest beacon is at ")
            .wrap_err("missing infix")?;

        let sensor_pos = sensor.parse()?;
        let beacon_pos = beacon.parse()?;

        Ok(Sensor {
            pos: sensor_pos,
            beacon: beacon_pos,
            radius: sensor_pos.dist(beacon_pos),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn is_in_range(self, sensor: &Sensor) -> bool {
        self.dist(sensor.pos) <= sensor.radius
    }

    fn tuning_frequency(self) -> i64 {
        self.x as i64 * 4_000_000 + self.y as i64
    }
}

impl FromStr for Pos {
    type Err = Report;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").wrap_err("missing comma")?;

        let x = x
            .strip_prefix("x=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid x")?;

        let y = y
            .strip_prefix("y=")
            .map(str::parse)
            .and_then(Result::ok)
            .wrap_err("invalid y")?;

        Ok(Self { x, y })
    }
}

#[derive(Copy, Clone)]
struct Line {
    min: i32,
    max: i32,
}

struct Lines;

impl Lines {
    fn generate(sensors: &[Sensor], y: i32) -> Result<Vec<Line>> {
        let mut lines = Vec::new();
        let mut events = Vec::with_capacity(sensors.len() * 2);

        for sensor in sensors {
            let Sensor {
                pos,
                beacon: _,
                radius,
            } = sensor;

            let y_dist = (pos.y - y).abs();
            let diff = radius - y_dist;

            if diff < 0 {
                continue;
            }

            let start = LineEvent {
                x: pos.x - diff,
                kind: LineEventKind::Start,
            };

            let end = LineEvent {
                x: pos.x + diff,
                kind: LineEventKind::End,
            };

            events.extend([start, end]);
        }

        events.sort_unstable();

        let mut active = 0;
        let mut start = 0;

        let mut events = events.into_iter();

        while let Some(LineEvent { x, kind }) = events.next() {
            match kind {
                LineEventKind::Start => {
                    active += 1;

                    if active == 1 {
                        start = x;
                    }
                }
                LineEventKind::End if active > 1 => active -= 1,
                LineEventKind::End => {
                    lines.push(Line { min: start, max: x });

                    let Some(LineEvent { x, kind }) = events.next() else { break };

                    match kind {
                        LineEventKind::Start => start = x,
                        LineEventKind::End => bail!("end event while no active lines"),
                    }
                }
            }
        }

        Ok(lines)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LineEvent {
    x: i32,
    kind: LineEventKind,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LineEventKind {
    Start,
    End,
}

#[derive(Copy, Clone)]
struct Quadrant {
    top_l: Pos,
    bot_r: Pos,
}

impl Quadrant {
    // whether the quadrant lies entirely within the sensor's range
    fn valid(&self, sensor: &Sensor) -> bool {
        let Self { top_l, bot_r } = *self;
        let top_r = Pos::new(bot_r.x, top_l.y);
        let bot_l = Pos::new(top_l.x, bot_r.y);

        let a = sensor.pos.dist(top_l);
        let b = sensor.pos.dist(top_r);
        let c = sensor.pos.dist(bot_l);
        let d = sensor.pos.dist(bot_r);

        a.max(b).max(c).max(d) > sensor.radius
    }
}
