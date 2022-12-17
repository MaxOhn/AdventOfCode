use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn part1(input: &str) -> Result<u64> {
    let rocks = ROCKS.iter().cycle().take(2022);
    let mut jets = input.bytes().cycle();

    let mut total_height = 0;
    let mut stopped = HashSet::new();
    let mut buf = Vec::new();

    for rock in rocks {
        let offset = Pos {
            x: 2,
            y: total_height + 3,
        };

        buf.extend(rock.shape.iter().map(|&pos| pos + offset));

        for jet in &mut jets {
            match jet {
                b'>' => shift_right(&mut buf, &stopped),
                b'<' => shift_left(&mut buf, &stopped),
                _ => bail!("invalid jet"),
            }

            if !shift_down(&mut buf, &stopped) {
                break;
            }
        }

        let rock_height = buf.iter().map(|pos| pos.y).max().unwrap() + 1;
        total_height = rock_height.max(total_height);

        stopped.extend(buf.drain(..));
    }

    Ok(total_height)
}

pub fn part2(input: &str) -> Result<u64> {
    const THRESHOLD: usize = 10_000;

    let mut heights = Vec::with_capacity(THRESHOLD);

    let mut jets = input.bytes().cycle();

    let mut total_height = 0;
    let mut stopped = HashSet::new();
    let mut buf = Vec::new();

    for rock in ROCKS.iter().cycle().take(THRESHOLD) {
        let offset = Pos {
            x: 2,
            y: total_height + 3,
        };

        buf.extend(rock.shape.iter().map(|&pos| pos + offset));

        for jet in &mut jets {
            match jet {
                b'>' => shift_right(&mut buf, &stopped),
                b'<' => shift_left(&mut buf, &stopped),
                _ => bail!("invalid jet"),
            }

            if !shift_down(&mut buf, &stopped) {
                break;
            }
        }

        let rock_height = buf.iter().map(|pos| pos.y).max().unwrap() + 1;
        total_height = rock_height.max(total_height);

        stopped.extend(buf.drain(..));
        heights.push(total_height);
    }

    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    for skip in 0..THRESHOLD / 2 {
        let suffix = &heights[skip..];

        'cycle_len: for cycle_len in 2..(THRESHOLD - skip) / 2 {
            buf1.clear();

            let mut chunks = suffix.chunks_exact(cycle_len);
            let Some(first) = chunks.next() else { continue };

            let offset = first[0];
            buf1.extend(first.iter().map(|height| height - offset));

            for chunk in chunks {
                buf2.clear();
                let offset = chunk[0];
                buf2.extend(chunk.iter().map(|height| height - offset));

                if buf1 != buf2 {
                    continue 'cycle_len;
                }
            }

            println!("Found cycle: skip={skip} | cycle_len={cycle_len}");

            let height_per_cycle = heights[skip + cycle_len] - heights[skip];

            const ROCKS_COUNT: u64 = 1_000_000_000_000;

            let rocks_in_cycles = ROCKS_COUNT - skip as u64;
            let cycles = rocks_in_cycles / cycle_len as u64;
            let remaining_rocks = rocks_in_cycles % cycle_len as u64;

            let init_height = heights[skip - 1];
            let cycles_height = cycles * height_per_cycle;
            let remaining_height = heights[skip + remaining_rocks as usize] - heights[skip];

            let total_height = init_height + cycles_height + remaining_height;

            return Ok(total_height);
        }
    }

    bail!("missing cycle")
}

fn shift_left(rock: &mut [Pos], stopped: &HashSet<Pos>) {
    let any_hit = rock.iter().any(|pos| {
        if pos.x == 0 {
            return true;
        }

        let next = *pos - Pos { x: 1, y: 0 };

        stopped.contains(&next)
    });

    if !any_hit {
        rock.iter_mut().for_each(|pos| pos.x -= 1);
    }
}

fn shift_right(rock: &mut [Pos], stopped: &HashSet<Pos>) {
    let any_hit = rock.iter().any(|pos| {
        if pos.x == 6 {
            return true;
        }

        let next = *pos + Pos { x: 1, y: 0 };

        stopped.contains(&next)
    });

    if !any_hit {
        rock.iter_mut().for_each(|pos| pos.x += 1);
    }
}

fn shift_down(rock: &mut [Pos], stopped: &HashSet<Pos>) -> bool {
    let any_hit = rock.iter().any(|pos| {
        if pos.y == 0 {
            return true;
        }

        let next = *pos - Pos { x: 0, y: 1 };

        stopped.contains(&next)
    });

    if any_hit {
        false
    } else {
        rock.iter_mut().for_each(|pos| pos.y -= 1);

        true
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Rock {
    shape: &'static [Pos],
}

const ROCKS: [Rock; 5] = [
    // ####
    Rock {
        shape: &[
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 2, y: 0 },
            Pos { x: 3, y: 0 },
        ],
    },
    // .#.
    // ###
    // .#.
    Rock {
        shape: &[
            Pos { x: 1, y: 2 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
            Pos { x: 2, y: 1 },
            Pos { x: 1, y: 0 },
        ],
    },
    // ..#
    // ..#
    // ###
    Rock {
        shape: &[
            Pos { x: 2, y: 2 },
            Pos { x: 2, y: 1 },
            Pos { x: 2, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 0 },
        ],
    },
    // #
    // #
    // #
    // #
    Rock {
        shape: &[
            Pos { x: 0, y: 3 },
            Pos { x: 0, y: 2 },
            Pos { x: 0, y: 1 },
            Pos { x: 0, y: 0 },
        ],
    },
    // ##
    // ##
    Rock {
        shape: &[
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
        ],
    },
];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: u8,
    y: u64,
}

impl Add for Pos {
    type Output = Pos;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
