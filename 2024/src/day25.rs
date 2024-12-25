use aoc_rust::Solution;
use eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for grid in input.split("\n\n") {
        let (schematic, is_lock) = Schematic::parse(grid);

        if is_lock {
            locks.push(schematic);
        } else {
            keys.push(schematic);
        }
    }

    locks.sort_unstable_by_key(|lock| lock.extremum);
    keys.sort_unstable_by_key(|key| key.extremum);

    locks
        .par_iter()
        .map(|lock| {
            let mut count = 0;

            for key in keys.iter() {
                if lock.extremum + key.extremum >= 6 {
                    break;
                }

                if lock.heights.iter().zip(key.heights).all(|(a, b)| a + b < 6) {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

struct Schematic {
    heights: [i8; 5],
    extremum: i8,
}

impl Schematic {
    fn parse(input: &str) -> (Self, bool) {
        let mut chunks = input.as_bytes().chunks(6);

        let chunk = chunks.next().unwrap();
        let mut heights = [-1; 5];

        for (byte, height) in chunk.iter().zip(heights.iter_mut()) {
            *height += i8::from(*byte == b'#');
        }

        let is_lock = heights == [0; 5];

        for chunk in chunks.take(6) {
            for (byte, height) in chunk.iter().zip(heights.iter_mut()) {
                *height += i8::from(*byte == b'#');
            }
        }

        let extremum = if is_lock {
            *heights.iter().min().unwrap()
        } else {
            *heights.iter().max().unwrap()
        };

        (Self { heights, extremum }, is_lock)
    }
}

fn part2(_: &str) -> Box<str> {
    Box::from("x")
}
