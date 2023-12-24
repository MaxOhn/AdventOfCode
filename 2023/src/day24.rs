use std::str::FromStr;

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result};
use nalgebra::{matrix, vector, Vector3, LU};

pub fn run(input: &str) -> Result<Solution> {
    let hail = input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Hailstone>>>()?;

    let p1 = part1(&hail);
    let p2 = part2(&hail)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(hail: &[Hailstone]) -> u32 {
    const MIN_AREA: f64 = 200_000_000_000_000.0;
    const MAX_AREA: f64 = 400_000_000_000_000.0;

    let mut collide = 0;

    for (i, u) in hail.iter().enumerate() {
        for v in hail.iter().skip(i + 1) {
            let [a, b, c, d] = [u.vel[0], u.pos[0], v.vel[0], v.pos[0]];
            let [e, f, g, h] = [u.vel[1], u.pos[1], v.vel[1], v.pos[1]];

            let ratio = -c / g;

            let [i, j, l] = [a + ratio * e, b + ratio * f, d + ratio * h];
            let t = -(j - l) / i;

            let ratio2 = -a / e;
            let [n, o, p] = [b + ratio2 * f, c + ratio2 * g, d + ratio2 * h];
            let t2 = -(p - n) / o;

            let mut x = u.vel[0] * t + u.pos[0];
            let mut y = u.vel[1] * t + u.pos[1];

            if t < 0.0 || t2 < 0.0 {
                x = f64::INFINITY;
                y = f64::INFINITY;
            }

            if MIN_AREA <= x && x <= MAX_AREA && MIN_AREA <= y && y <= MAX_AREA {
                collide += 1;
            }
        }
    }

    collide
}

fn part2(hail: &[Hailstone]) -> Result<u64> {
    let Some([a, b, c]) = hail.get(..3) else {
        eyre::bail!("need at least 3 hailstones");
    };

    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect

    let m = matrix![
        -(a.vel[1] - b.vel[1]), a.vel[0] - b.vel[0], 0.0, a.pos[1] - b.pos[1], -(a.pos[0] - b.pos[0]), 0.0;
        -(a.vel[1] - c.vel[1]), a.vel[0] - c.vel[0], 0.0, a.pos[1] - c.pos[1], -(a.pos[0] - c.pos[0]), 0.0;

        0.0, -(a.vel[2] - b.vel[2]), a.vel[1] - b.vel[1], 0.0, a.pos[2] - b.pos[2], -(a.pos[1] - b.pos[1]);
        0.0, -(a.vel[2] - c.vel[2]), a.vel[1] - c.vel[1], 0.0, a.pos[2] - c.pos[2], -(a.pos[1] - c.pos[1]);

        -(a.vel[2] - b.vel[2]), 0.0, a.vel[0] - b.vel[0], a.pos[2] - b.pos[2], 0.0, -(a.pos[0] - b.pos[0]);
        -(a.vel[2] - c.vel[2]), 0.0, a.vel[0] - c.vel[0], a.pos[2] - c.pos[2], 0.0, -(a.pos[0] - c.pos[0]);
    ];

    let mut rhs = vector![
        (a.pos[1] * a.vel[0] - b.pos[1] * b.vel[0]) - (a.pos[0] * a.vel[1] - b.pos[0] * b.vel[1]),
        (a.pos[1] * a.vel[0] - c.pos[1] * c.vel[0]) - (a.pos[0] * a.vel[1] - c.pos[0] * c.vel[1]),
        (a.pos[2] * a.vel[1] - b.pos[2] * b.vel[1]) - (a.pos[1] * a.vel[2] - b.pos[1] * b.vel[2]),
        (a.pos[2] * a.vel[1] - c.pos[2] * c.vel[1]) - (a.pos[1] * a.vel[2] - c.pos[1] * c.vel[2]),
        (a.pos[2] * a.vel[0] - b.pos[2] * b.vel[0]) - (a.pos[0] * a.vel[2] - b.pos[0] * b.vel[2]),
        (a.pos[2] * a.vel[0] - c.pos[2] * c.vel[0]) - (a.pos[0] * a.vel[2] - c.pos[0] * c.vel[2]),
    ];

    if !LU::new(m).solve_mut(&mut rhs) {
        eyre::bail!("matrix has no inverse");
    }

    // still an off-by-one :/
    Ok(rhs.rows_range(0..3).sum().round() as u64)
}

#[derive(Debug)]
struct Hailstone {
    pos: Vector3<f64>,
    vel: Vector3<f64>,
}

impl FromStr for Hailstone {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (front, back) = s.split_once('@').wrap_err("missing `@`")?;

        Ok(Self {
            pos: parse_vector(front)?,
            vel: parse_vector(back)?,
        })
    }
}

fn parse_vector(s: &str) -> Result<Vector3<f64>> {
    let mut split = s
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .flat_map(Result::ok);

    Ok(vector![
        split.next().wrap_err("missing x")?,
        split.next().wrap_err("missing y")?,
        split.next().wrap_err("missing z")?,
    ])
}
