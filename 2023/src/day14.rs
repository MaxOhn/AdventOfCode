use aoc_rust::Solution;
use eyre::Result;

use self::dish::Dish;

pub fn run(input: &str) -> Result<Solution> {
    let dish: Dish = input.trim().parse()?;

    let p1 = part1(dish.clone());
    let p2 = part2(dish);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(mut dish: Dish) -> usize {
    dish.move_north().load()
}

fn part2(mut dish: Dish) -> usize {
    const TARGET: usize = 1_000_000_000;

    let mut history = Vec::new();
    history.push(dish.clone());
    let mut cycle_start = None;

    for _ in 0..TARGET {
        dish.cycle();

        if let Some(i) = history.iter().position(|entry| entry == &dish) {
            cycle_start = Some(i);
            break;
        }

        history.push(dish.clone());
    }

    let remaining = match cycle_start {
        Some(cycle_start) => {
            let cycle_len = history.len() - cycle_start;
            let count = (TARGET - cycle_start) / cycle_len;

            TARGET - cycle_start - count * cycle_len
        }
        _ => 0,
    };

    for _ in 0..remaining {
        dish.cycle();
    }

    dish.load()
}

mod dish {
    use std::str::FromStr;

    use eyre::Report;

    #[derive(Clone, PartialEq)]
    pub struct Dish {
        width: usize,
        dish: Box<[Rock]>,
    }

    impl Dish {
        pub fn load(&self) -> usize {
            self.dish
                .chunks_exact(self.width)
                .rev()
                .zip(1..)
                .fold(0, |load, (row, i)| {
                    load + i * row.iter().filter(|&&rock| rock == Rock::Rounded).count()
                })
        }

        pub fn cycle(&mut self) {
            self.move_north().move_west().move_south().move_east();
        }

        pub fn move_north(&mut self) -> &mut Self {
            let w = self.width;

            for y in (w..self.dish.len()).step_by(w) {
                for x in 0..w {
                    if self.dish[y + x] != Rock::Rounded {
                        continue;
                    }

                    let mut offset = w;

                    while y >= offset && self.dish[y - offset + x] == Rock::Empty {
                        offset += w;
                    }

                    self.dish.swap(y + x, y + w - offset + x);
                }
            }

            self
        }

        pub fn move_south(&mut self) -> &mut Self {
            let w = self.width;

            for y in (0..self.dish.len() - w).step_by(w).rev() {
                for x in 0..w {
                    if self.dish[y + x] != Rock::Rounded {
                        continue;
                    }

                    let mut offset = w;

                    while y + offset < self.dish.len() && self.dish[y + offset + x] == Rock::Empty {
                        offset += w;
                    }

                    self.dish.swap(y + x, y + offset - w + x);
                }
            }

            self
        }

        pub fn move_west(&mut self) -> &mut Self {
            let w = self.width;

            for y in (0..self.dish.len()).step_by(w) {
                for x in 1..w {
                    if self.dish[y + x] != Rock::Rounded {
                        continue;
                    }

                    let mut offset = 1;

                    while x >= offset && self.dish[y + x - offset] == Rock::Empty {
                        offset += 1;
                    }

                    self.dish.swap(y + x, y + x + 1 - offset);
                }
            }

            self
        }

        pub fn move_east(&mut self) -> &mut Self {
            let w = self.width;

            for y in (0..self.dish.len()).step_by(w) {
                for x in (0..w - 1).rev() {
                    if self.dish[y + x] != Rock::Rounded {
                        continue;
                    }

                    let mut offset = 1;

                    while x + offset < w && self.dish[y + x + offset] == Rock::Empty {
                        offset += 1;
                    }

                    self.dish.swap(y + x, y + x + offset - 1);
                }
            }

            self
        }
    }

    impl FromStr for Dish {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map_or(0, str::len);

            let dish = s
                .lines()
                .flat_map(|line| line.bytes().map(Rock::try_from))
                .collect::<Result<_, Report>>()?;

            Ok(Self { width, dish })
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum Rock {
        Rounded,
        Cube,
        Empty,
    }

    impl TryFrom<u8> for Rock {
        type Error = Report;

        fn try_from(byte: u8) -> Result<Self, Self::Error> {
            match byte {
                b'O' => Ok(Self::Rounded),
                b'#' => Ok(Self::Cube),
                b'.' => Ok(Self::Empty),
                _ => eyre::bail!("invalid rock byte `{byte}`"),
            }
        }
    }
}
