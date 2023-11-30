use std::{
    collections::{HashMap, HashSet},
    mem,
};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut steps = Vec::new();

    let mut x_values = HashSet::new();
    let mut y_values = HashSet::new();
    let mut z_values = HashSet::new();

    for line in input.lines() {
        let step = Cube::from_str(line.trim_end());

        x_values.insert(step.x_min);
        x_values.insert(step.x_max);
        y_values.insert(step.y_min);
        y_values.insert(step.y_max);
        z_values.insert(step.z_min);
        z_values.insert(step.z_max);

        steps.push(step);
    }

    let p1 = part1(&steps);
    let p2 = part2(&steps, x_values, y_values, z_values);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(steps: &[Cube]) -> usize {
    let mut map = HashMap::new();

    for step in steps.iter() {
        if step.x_min < -50
            || step.y_min < -50
            || step.z_min < -50
            || step.x_max > 50
            || step.y_max > 50
            || step.z_max > 50
        {
            continue;
        }

        for x in step.x_min..step.x_max {
            for y in step.y_min..step.y_max {
                for z in step.z_min..step.z_max {
                    map.insert((x, y, z), step.on);
                }
            }
        }
    }

    map.values().filter(|b| **b).count()
}

type Int = u16;

fn part2(
    steps: &[Cube],
    x_values: HashSet<i64>,
    y_values: HashSet<i64>,
    z_values: HashSet<i64>,
) -> i64 {
    let (x_indices, x_lengths) = process_axis(x_values);
    let (y_indices, y_lengths) = process_axis(y_values);
    let (z_indices, z_lengths) = process_axis(z_values);

    let mut coords = HashSet::new();
    let mut answer = 0;

    for (i, step) in steps.iter().enumerate() {
        let mb =
            (mem::size_of_val(&coords) + coords.capacity() * mem::size_of::<Int>()) / 1000 / 1000;

        println!("{}/{} ({}mb)", i + 1, steps.len(), mb);

        for x in x_indices[&step.x_min]..x_indices[&step.x_max] {
            for y in y_indices[&step.y_min]..y_indices[&step.y_max] {
                for z in z_indices[&step.z_min]..z_indices[&step.z_max] {
                    if step.on {
                        coords.insert((x, y, z));
                    } else {
                        coords.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    for (x, y, z) in coords {
        answer += x_lengths[x as usize] * y_lengths[y as usize] * z_lengths[z as usize];
    }

    answer
}

fn process_axis(values: HashSet<i64>) -> (HashMap<i64, Int>, Vec<i64>) {
    let mut values: Vec<_> = values.into_iter().collect();
    values.sort_unstable();

    let mut indices = HashMap::with_capacity(values.len());

    let lengths: Vec<_> = values
        .iter()
        .zip(values.iter().skip(1))
        .enumerate()
        .map(|(i, (curr, next))| {
            indices.insert(*curr, i as Int);

            next - curr
        })
        .collect();

    let last = *values.last().unwrap();
    let idx = values.len() - 1;

    indices.insert(last, idx as Int);

    (indices, lengths)
}

#[derive(Debug)]
struct Cube {
    on: bool,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Cube {
    fn from_str(line: &str) -> Self {
        let (on, pos) = line.split_once(' ').unwrap();
        let on = on == "on";

        let mut split = pos
            .split(',')
            .map(|s| s.split('=').last())
            .flatten()
            .map(|s| s.split_once(".."))
            .flatten();

        let x_range = split.next().unwrap();
        let y_range = split.next().unwrap();
        let z_range = split.next().unwrap();

        let x_min = x_range.0.parse().unwrap();
        let x_max = x_range.1.parse::<i64>().unwrap() + 1;

        let y_min = y_range.0.parse().unwrap();
        let y_max = y_range.1.parse::<i64>().unwrap() + 1;

        let z_min = z_range.0.parse().unwrap();
        let z_max = z_range.1.parse::<i64>().unwrap() + 1;

        Self {
            on,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}
