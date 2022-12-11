use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let mut screen = Crt::new();

    for line in input.lines() {
        match line.split_once(' ') {
            Some((op, val)) => {
                ensure!(op == "addx", "invalid line");

                process_cycle(x, &mut cycle, &mut sum, &mut screen);
                process_cycle(x, &mut cycle, &mut sum, &mut screen);

                x += val.parse::<i32>().map_err(|_| eyre!("invalid val"))?;
            }
            None => {
                ensure!(line == "noop", "invalid line");

                process_cycle(x, &mut cycle, &mut sum, &mut screen);
            }
        }
    }

    Ok(Solution::new().part1(sum).part2(screen.to_string()))
}

fn process_cycle(x: i32, cycle: &mut i32, sum: &mut i32, screen: &mut Crt) {
    screen.draw(x, *cycle);
    *cycle += 1;

    if [20, 60, 100, 140, 180, 220].contains(cycle) {
        *sum += *cycle * x;
    }
}

struct Crt {
    inner: [bool; 240],
}

impl Crt {
    const W: usize = 40;

    fn new() -> Self {
        Self {
            inner: [false; 240],
        }
    }

    fn draw(&mut self, x: i32, cycle: i32) {
        self.inner[cycle as usize] |= ((x - 1)..=(x + 1)).contains(&(cycle % 40));
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut chunks = self.inner.chunks_exact(Self::W);
        const OFF: &str = " ";
        const ON: &str = "█";

        if let Some(chunk) = chunks.next() {
            for &px in chunk {
                f.write_str(if px { ON } else { OFF })?;
            }
        }

        for chunk in chunks {
            f.write_str("\n")?;

            for &px in chunk {
                f.write_str(if px { ON } else { OFF })?;
            }
        }

        Ok(())
    }
}
