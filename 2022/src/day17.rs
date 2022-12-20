use std::{
    collections::{hash_map::Entry, HashMap},
    iter,
};

use ahash::RandomState;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

pub fn part1(input: &str) -> Result<u64> {
    let mut cave = vec![0b00000001; 9000];
    cave[0] = u8::MAX;
    let mut max_height = 0;

    let mut jets = input.bytes().cycle();

    for mut rock in ROCKS.iter().cycle().take(2022).copied() {
        let mut rock_bottom = max_height + 4;

        for jet in &mut jets {
            match jet {
                b'<' => shift_left(&mut rock, &cave, rock_bottom),
                b'>' => shift_right(&mut rock, &cave, rock_bottom),
                _ => bail!("invalid jet"),
            }

            if shift_down(rock, &mut cave, rock_bottom) {
                rock_bottom -= 1;
            } else {
                let rock_height = rock.iter().rev().take_while(|&&row| row > 0).count();
                max_height = max_height.max(rock_height + rock_bottom - 1);

                break;
            }
        }
    }

    Ok(max_height as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    const ROCKS_COUNT: u64 = 1_000_000_000_000;

    let mut cave = vec![0b00000001; 10_000];
    cave[0] = u8::MAX;
    let mut max_height = 0;

    let rocks = ROCKS.iter().copied().zip(0..).cycle();
    let mut jets = input.bytes().zip(0..).cycle();

    let mut rock_count = 0;
    let mut height_from_cycles = 0;
    let mut cache = HashMap::with_capacity_and_hasher(2048, RandomState::new());

    for (mut rock, rock_idx) in rocks {
        rock_count += 1;

        if rock_count > ROCKS_COUNT {
            break;
        }

        let mut rock_bottom = max_height + 4;

        for (jet, jet_idx) in &mut jets {
            match jet {
                b'<' => shift_left(&mut rock, &cave, rock_bottom),
                b'>' => shift_right(&mut rock, &cave, rock_bottom),
                _ => bail!("invalid jet"),
            }

            if shift_down(rock, &mut cave, rock_bottom) {
                rock_bottom -= 1;
                continue;
            }

            let rock_height = rock.iter().rev().take_while(|&&row| row > 0).count();

            if rock_height + rock_bottom - 1 > max_height {
                max_height = rock_height + rock_bottom - 1;

                if cave.len() - max_height < 10 {
                    cave.extend(iter::repeat(0b00000001).take(1000));
                }
            }

            if max_height < 8 {
                break;
            }

            let last_8 = cave[max_height - 8..max_height]
                .try_into()
                .map(u64::from_le_bytes)
                .unwrap();

            match cache.entry((rock_idx, jet_idx, last_8)) {
                Entry::Occupied(e) => {
                    let (old_rock_count, old_height) = e.get();
                    let rocks_per_cycle = rock_count - old_rock_count;
                    let remaining_cycles = (ROCKS_COUNT - rock_count) / rocks_per_cycle;
                    rock_count += rocks_per_cycle * remaining_cycles;
                    height_from_cycles += remaining_cycles * (max_height - old_height) as u64;
                }
                Entry::Vacant(e) => {
                    e.insert((rock_count, max_height));
                }
            }

            break;
        }
    }

    Ok(max_height as u64 + height_from_cycles)
}

fn shift_left(rock: &mut Rock, cave: &[Row], rock_bottom: usize) {
    let hits_other = rock
        .iter()
        .zip(cave[rock_bottom..rock_bottom + 4].iter().rev())
        .any(|(&rock, row)| (0b1000_0000 <= rock) || ((rock << 1) & row) > 0);

    if !hits_other {
        rock.iter_mut().for_each(|row| *row <<= 1);
    }
}

fn shift_right(rock: &mut Rock, cave: &[Row], rock_bottom: usize) {
    let hits_other = rock
        .iter()
        .zip(cave[rock_bottom..rock_bottom + 4].iter().rev())
        // out of bounds check handled through row's right-most wall
        .any(|(rock, row)| ((rock >> 1) & row) > 0);

    if !hits_other {
        rock.iter_mut().for_each(|row| *row >>= 1);
    }
}

fn shift_down(rock: Rock, cave: &mut [Row], rock_bottom: usize) -> bool {
    let hits_down = rock
        .iter()
        .zip(cave[rock_bottom - 1..rock_bottom + 3].iter().rev())
        .map(|(rock, row)| rock & row)
        .any(|byte| byte > 0);

    if hits_down {
        rock.iter()
            .zip(cave[rock_bottom..rock_bottom + 4].iter_mut().rev())
            .for_each(|(rock, row)| *row |= rock);

        false
    } else {
        true
    }
}

type Row = u8;
type Rock = [Row; 4];

#[rustfmt::skip]
const ROCKS: [Rock; 5] = [
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00111100,
    ],
    [
        0b00000000,
        0b00010000,
        0b00111000,
        0b00010000,
    ],
    [
        0b00000000,
        0b00001000,
        0b00001000,
        0b00111000,
    ],
    [
        0b00100000,
        0b00100000,
        0b00100000,
        0b00100000,
    ],
    [
        0b00000000,
        0b00000000,
        0b00110000,
        0b00110000,
    ],
];
