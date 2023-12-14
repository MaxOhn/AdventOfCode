use aoc_rust::Solution;
use eyre::{Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<usize> {
    let mut dish = Vec::new();

    for line in input.lines() {
        let line = line
            .bytes()
            .map(Rock::try_from)
            .collect::<Result<Vec<_>>>()?;

        dish.push(line);
    }

    move_north(&mut dish);

    Ok(load(&dish))
}

fn part2(input: &str) -> Result<usize> {
    let mut dish = Vec::new();

    for line in input.lines() {
        let line = line
            .bytes()
            .map(Rock::try_from)
            .collect::<Result<Vec<_>>>()?;

        dish.push(line);
    }

    let mut history = Vec::new();
    history.push(dish.clone());
    let mut cycle_start = None;

    for _ in 0..1_000_000_000 {
        cycle(&mut dish);

        if let Some(i) = history.iter().position(|entry| entry == &dish) {
            cycle_start = Some(i);
            break;
        }

        history.push(dish.clone());
    }

    let cycle_start = cycle_start.unwrap();
    let cycle_len = history.len() - cycle_start;

    let target = 1_000_000_000;
    let cycle_count = (target - cycle_start) / cycle_len;
    let remaining = target - cycle_start - cycle_count * cycle_len;

    for _ in 0..remaining {
        cycle(&mut dish);
    }

    Ok(load(&dish))
}

fn load(dish: &[Vec<Rock>]) -> usize {
    let mut load = 0;

    for (row, i) in dish.iter().rev().zip(1..) {
        let rounded = row.iter().filter(|&&rock| rock == Rock::Rounded).count();
        load += rounded * i;
    }

    load
}

fn cycle(dish: &mut [Vec<Rock>]) {
    move_north(dish);
    move_west(dish);
    move_south(dish);
    move_east(dish);
}

fn move_north(dish: &mut [Vec<Rock>]) {
    for y in 1..dish.len() {
        for x in 0..dish[y].len() {
            if !matches!(dish[y][x], Rock::Rounded) {
                continue;
            }

            let mut offset = 1;

            while y >= offset {
                if dish[y - offset][x] == Rock::Empty {
                    dish[y - offset + 1][x] = Rock::Empty;
                    dish[y - offset][x] = Rock::Rounded;
                } else {
                    break;
                }

                offset += 1;
            }
        }
    }
}

fn move_south(dish: &mut [Vec<Rock>]) {
    for y in (0..dish.len() - 1).rev() {
        for x in 0..dish[y].len() {
            if !matches!(dish[y][x], Rock::Rounded) {
                continue;
            }

            let mut offset = 1;

            while y + offset < dish.len() {
                if dish[y + offset][x] == Rock::Empty {
                    dish[y + offset - 1][x] = Rock::Empty;
                    dish[y + offset][x] = Rock::Rounded;
                } else {
                    break;
                }

                offset += 1;
            }
        }
    }
}

fn move_west(dish: &mut [Vec<Rock>]) {
    for y in 0..dish.len() {
        for x in 1..dish[y].len() {
            if !matches!(dish[y][x], Rock::Rounded) {
                continue;
            }

            let mut offset = 1;

            while x >= offset {
                if dish[y][x - offset] == Rock::Empty {
                    dish[y][x - offset + 1] = Rock::Empty;
                    dish[y][x - offset] = Rock::Rounded;
                } else {
                    break;
                }

                offset += 1;
            }
        }
    }
}

fn move_east(dish: &mut [Vec<Rock>]) {
    for y in 0..dish.len() {
        for x in (0..dish[y].len() - 1).rev() {
            if !matches!(dish[y][x], Rock::Rounded) {
                continue;
            }

            let mut offset = 1;

            while x + offset < dish[y].len() {
                if dish[y][x + offset] == Rock::Empty {
                    dish[y][x + offset - 1] = Rock::Empty;
                    dish[y][x + offset] = Rock::Rounded;
                } else {
                    break;
                }

                offset += 1;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rock {
    Rounded,
    Cubes,
    Empty,
}

impl TryFrom<u8> for Rock {
    type Error = Report;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'O' => Ok(Self::Rounded),
            b'#' => Ok(Self::Cubes),
            b'.' => Ok(Self::Empty),
            _ => eyre::bail!("invalid rock byte `{byte}`"),
        }
    }
}
