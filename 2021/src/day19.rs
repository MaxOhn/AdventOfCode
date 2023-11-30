// #![allow(dead_code, unused_mut)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    slice::Iter,
};

use aoc_rust::Solution;
use eyre::Result;

use crate::util::Pos3;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let mut scanner_queue = VecDeque::new();
    let mut id = 0;

    while lines.next().is_some() {
        let mut scanner = Scanner::new(id);
        id += 1;

        while let Some(line) = lines.next() {
            if line.len() <= 1 {
                break;
            }

            let mut split = line
                .trim_end()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap);

            let pos = Pos3 {
                x: split.next().unwrap(),
                y: split.next().unwrap(),
                z: split.next().unwrap(),
            };

            scanner.reports.push(pos);
        }

        scanner_queue.push_back(scanner);
    }

    let mut corrected = Vec::with_capacity(scanner_queue.len());
    corrected.push(scanner_queue.pop_front().unwrap());
    let mut compared = HashSet::new();

    'outer: while let Some(to_correct) = scanner_queue.pop_front() {
        for correct in &corrected {
            if !compared.insert((correct.id, to_correct.id)) {
                continue;
            }

            for &orientation in ORIENTATIONS.iter() {
                let mut adjusted = to_correct.apply_orientation(orientation);

                if let Some(offset) = correct.enough_overlaps(&adjusted) {
                    adjusted.apply_offset(offset);
                    adjusted.pos = offset;
                    corrected.push(adjusted);

                    continue 'outer;
                }
            }

            // * Alternative approach
            // let mut transformations = Transformations::new();

            // for _ in 0..24 {
            //     to_correct.apply_transformation(&mut transformations);

            //     if let Some(offset) = correct.enough_overlaps(&to_correct) {
            //         to_correct.apply_offset(offset);
            //         to_correct.pos = offset;
            //         corrected.push(to_correct);

            //         continue 'outer;
            //     }
            // }
        }

        scanner_queue.push_back(to_correct);
    }

    let p1 = part1(&corrected);
    let p2 = part2(&corrected);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(scanners: &[Scanner]) -> usize {
    let mut beacons: HashSet<&Pos3<i32>> = HashSet::new();

    for scanner in scanners {
        beacons.extend(&scanner.reports);
    }

    beacons.len()
}

fn part2(scanners: &[Scanner]) -> i32 {
    let mut max = 0;

    for i in 0..scanners.len() - 1 {
        for j in i + 1..scanners.len() {
            let dist = scanners[i].pos.manhatten_dist(&scanners[j].pos);
            max = max.max(dist);
        }
    }

    max
}

#[derive(Clone, Default)]
struct Scanner {
    id: u8,
    pos: Pos3<i32>,
    reports: Vec<Pos3<i32>>,
}

impl Scanner {
    fn new(id: u8) -> Self {
        Self {
            id,
            pos: Pos3::default(),
            reports: Vec::new(),
        }
    }

    fn apply_orientation(&self, orientation: Orientation) -> Self {
        let reports = self
            .reports
            .iter()
            .map(|pos| Pos3 {
                x: pos[orientation.permutation[0]] * orientation.rotation[0],
                y: pos[orientation.permutation[1]] * orientation.rotation[1],
                z: pos[orientation.permutation[2]] * orientation.rotation[2],
            })
            .collect();

        Self {
            id: self.id,
            pos: self.pos,
            reports,
        }
    }

    fn enough_overlaps(&self, other: &Self) -> Option<Pos3<i32>> {
        let mut offsets: HashMap<Pos3<_>, usize> = HashMap::new();

        for &this in self.reports.iter() {
            for &that in other.reports.iter() {
                *offsets.entry(this - that).or_default() += 1;
            }
        }

        offsets
            .into_iter()
            .find_map(|(offset, count)| (count >= 12).then(|| offset))
    }

    fn apply_offset(&mut self, offset: Pos3<i32>) {
        self.reports.iter_mut().for_each(|pos| *pos += offset);
    }

    #[allow(unused)]
    fn apply_transformation(&mut self, transformations: &mut Transformations) {
        transformations.apply(&mut self.reports);
    }
}

static ORIENTATIONS: [Orientation; 24] = [
    Orientation::new([1, -1, 1], [0, 2, 1]),
    Orientation::new([1, -1, -1], [1, 2, 0]),
    Orientation::new([-1, -1, -1], [0, 2, 1]),
    Orientation::new([-1, -1, 1], [1, 2, 0]),
    Orientation::new([1, -1, 1], [2, 1, 0]),
    Orientation::new([1, 1, 1], [2, 0, 1]),
    Orientation::new([1, 1, -1], [2, 1, 0]),
    Orientation::new([1, -1, -1], [2, 0, 1]),
    Orientation::new([1, -1, 1], [1, 0, 2]),
    Orientation::new([-1, -1, 1], [0, 1, 2]),
    Orientation::new([-1, 1, 1], [1, 0, 2]),
    Orientation::new([1, 1, 1], [0, 1, 2]),
    Orientation::new([-1, 1, -1], [1, 2, 0]),
    Orientation::new([1, 1, -1], [0, 2, 1]),
    Orientation::new([1, 1, 1], [1, 2, 0]),
    Orientation::new([-1, 1, 1], [0, 2, 1]),
    Orientation::new([-1, 1, -1], [0, 1, 2]),
    Orientation::new([-1, -1, -1], [1, 0, 2]),
    Orientation::new([1, -1, -1], [0, 1, 2]),
    Orientation::new([1, 1, -1], [1, 0, 2]),
    Orientation::new([-1, 1, -1], [2, 0, 1]),
    Orientation::new([-1, 1, 1], [2, 1, 0]),
    Orientation::new([-1, -1, 1], [2, 0, 1]),
    Orientation::new([-1, -1, -1], [2, 1, 0]),
];

#[derive(Copy, Clone, Debug)]
struct Orientation {
    rotation: [i32; 3],
    permutation: [usize; 3],
}

impl Orientation {
    const fn new(rotation: [i32; 3], permutation: [usize; 3]) -> Self {
        Self {
            rotation,
            permutation,
        }
    }
}

#[derive(Copy, Clone)]
enum Transformation {
    Roll,
    Turn,
}

impl Transformation {
    fn apply(self, elems: &mut [Pos3<i32>]) {
        match self {
            Transformation::Roll => {
                for elem in elems {
                    std::mem::swap(&mut elem.y, &mut elem.z);
                    elem.z *= -1;
                }
            }
            Transformation::Turn => {
                for elem in elems {
                    std::mem::swap(&mut elem.x, &mut elem.y);
                    elem.x *= -1;
                }
            }
        }
    }
}

static CYCLE: [Transformation; 12] = [
    Transformation::Roll,
    Transformation::Turn,
    Transformation::Turn,
    Transformation::Turn,
    Transformation::Roll,
    Transformation::Turn,
    Transformation::Turn,
    Transformation::Turn,
    Transformation::Roll,
    Transformation::Turn,
    Transformation::Turn,
    Transformation::Turn,
];

struct Transformations {
    iter: Iter<'static, Transformation>,
    mid: bool,
}

impl Transformations {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            iter: CYCLE.iter(),
            mid: true,
        }
    }

    fn apply(&mut self, elems: &mut [Pos3<i32>]) {
        let transformation = match self.iter.next().copied() {
            Some(transformation) => transformation,
            None if self.mid => {
                self.mid = false;
                self.iter = CYCLE.iter();
                Transformation::Roll.apply(elems);
                Transformation::Turn.apply(elems);
                Transformation::Roll.apply(elems);

                *self.iter.next().unwrap()
            }
            None => return,
        };

        transformation.apply(elems);
    }
}
