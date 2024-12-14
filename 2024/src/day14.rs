use std::ops::ControlFlow;

use aoc_rust::Solution;
use eyre::Result;
use nom::{
    bytes::complete as by,
    character::complete as ch,
    combinator::all_consuming,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

const W: i16 = 101;
const H: i16 = 103;
const STEPS: u8 = 100;

fn part1(input: &str) -> u32 {
    input
        .par_lines()
        .filter_map(|line| {
            let (_, mut robot) = Robot::parse(line).unwrap();

            (0..STEPS).for_each(|_| robot.run());

            let (x, y) = robot.pos();

            if y == H / 2 || x == W / 2 {
                return None;
            }

            let qx = usize::from(x > W / 2);
            let qy = usize::from(y > H / 2);

            Some((qx + 2 * qy) as usize)
        })
        .fold(
            || [0; 4],
            |mut quadrants, i| {
                quadrants[i] += 1;

                quadrants
            },
        )
        .reduce(
            || [0; 4],
            |mut reduced, quadrant| {
                for (reduced, quadrant) in reduced.iter_mut().zip(quadrant) {
                    *reduced += quadrant;
                }

                reduced
            },
        )
        .into_iter()
        .product()
}

fn part2(input: &str) -> usize {
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, robot) = Robot::parse(line).unwrap();

            robot
        })
        .collect();

    (1..)
        .find(|_| {
            robots.iter_mut().for_each(Robot::run);
            robots.sort_unstable_by_key(|robot| (robot.y, robot.x));

            robots.chunk_by(|a, b| a.y == b.y).any(|chunk| {
                let in_a_row = chunk.windows(2).try_fold(0_i8, |in_a_row, w| {
                    if w[0].x + 1 == w[1].x {
                        ControlFlow::Continue(in_a_row + 1)
                    } else if in_a_row >= 10 {
                        ControlFlow::Break(in_a_row)
                    } else {
                        ControlFlow::Continue(0)
                    }
                });

                let in_a_row = match in_a_row {
                    ControlFlow::Continue(n) => n,
                    ControlFlow::Break(n) => n,
                };

                in_a_row >= 10
            })
        })
        .unwrap()
}

struct Robot {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
}

impl Robot {
    fn parse(line: &str) -> IResult<&str, Self> {
        let xy = || separated_pair(ch::i16, by::tag(","), ch::i16);
        let eq = |prefix| preceded(by::tag(prefix), preceded(by::tag("="), xy()));
        let pv = separated_pair(eq("p"), by::tag(" "), eq("v"));
        let (rest, ((x, y), (vx, vy))) = all_consuming(pv)(line)?;

        Ok((rest, Self { x, y, vx, vy }))
    }

    fn run(&mut self) {
        self.x = (self.x + self.vx).rem_euclid(W);
        self.y = (self.y + self.vy).rem_euclid(H);
    }

    fn pos(&self) -> (i16, i16) {
        (self.x, self.y)
    }
}
