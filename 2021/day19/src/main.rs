use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    slice::Iter,
    time::Instant,
};

use util::Pos3;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut scanner_queue = VecDeque::new();

    while input.read_line(&mut line)? != 0 {
        let mut scanner = Scanner::new();
        line.clear();

        while input.read_line(&mut line)? > 1 {
            let mut split = line.trim_end().split(',').map(|n| n.parse().unwrap());

            let pos = Pos3 {
                x: split.next().unwrap(),
                y: split.next().unwrap(),
                z: split.next().unwrap(),
            };

            scanner.reports.push(pos);
            line.clear();
        }

        scanner_queue.push_back(scanner);
    }

    let mut corrected = Vec::with_capacity(scanner_queue.len());
    corrected.push(scanner_queue.pop_front().unwrap());

    'outer: while let Some(mut to_correct) = scanner_queue.pop_front() {
        for correct in &corrected {
            if let Some(offset) = correct.enough_overlaps(&to_correct) {
                to_correct.apply_offset(offset);
                to_correct.pos = offset;
                corrected.push(to_correct);

                continue 'outer;
            }

            for orientation in Orientations::new() {
                let mut adjusted = to_correct.apply_orientation(orientation);

                if let Some(offset) = correct.enough_overlaps(&adjusted) {
                    adjusted.apply_offset(offset);
                    adjusted.pos = offset;
                    corrected.push(adjusted);

                    continue 'outer;
                }
            }
        }

        scanner_queue.push_back(to_correct);
    }

    let p1 = part1(&corrected);
    let p2 = part2(&corrected);
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed);

    assert_eq!(p1, 353);
    assert_eq!(p2, 10_832);

    Ok(())
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

#[derive(Default)]
struct Scanner {
    pos: Pos3<i32>,
    reports: Vec<Pos3<i32>>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            pos: Pos3::default(),
            reports: Vec::new(),
        }
    }

    fn apply_orientation(&self, orientation: Orientation) -> Self {
        let reports = self
            .reports
            .iter()
            .copied()
            .map(|mut pos| {
                for i in 0..3 {
                    pos[i] *= orientation.rotation[i];
                }

                Pos3 {
                    x: pos[orientation.permutation[0]],
                    y: pos[orientation.permutation[1]],
                    z: pos[orientation.permutation[2]],
                }
            })
            .collect();

        Self {
            pos: self.pos,
            reports,
        }
    }

    fn enough_overlaps(&self, other: &Self) -> Option<Pos3<i32>> {
        for i in 0..self.reports.len() {
            for j in 0..other.reports.len() {
                let offset = self.reports[i] - other.reports[j];
                let mut overlaps = 0;

                for k in 0..self.reports.len() {
                    for l in 0..other.reports.len() {
                        if self.reports[k] == other.reports[l] + offset {
                            overlaps += 1;

                            if overlaps == 12 {
                                return Some(offset);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn apply_offset(&mut self, offset: Pos3<i32>) {
        self.reports.iter_mut().for_each(|pos| *pos += offset);
    }
}

static ROTATIONS: [[i32; 3]; 8] = [
    [1, 1, 1],
    [1, 1, -1],
    [1, -1, 1],
    [1, -1, -1],
    [-1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, -1, -1],
];

static PERMUTATIONS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

#[derive(Debug)]
struct Orientation {
    rotation: [i32; 3],
    permutation: [usize; 3],
}

struct Orientations {
    rotations: Iter<'static, [i32; 3]>,
    permutations: Iter<'static, [usize; 3]>,
    permutation: [usize; 3],
}

impl Orientations {
    fn new() -> Self {
        let mut permutations = PERMUTATIONS.iter();

        Self {
            rotations: ROTATIONS.iter(),
            permutation: *permutations.next().unwrap(),
            permutations,
        }
    }
}

// skips the first i.e. "no orientation change", do that manually instead
impl Iterator for Orientations {
    type Item = Orientation;

    fn next(&mut self) -> Option<Self::Item> {
        let rotation = match self.rotations.next() {
            Some(rotation) => *rotation,
            None => {
                self.permutation = *self.permutations.next()?;
                self.rotations = ROTATIONS.iter();

                *self.rotations.next()?
            }
        };

        let permutation = self.permutation;

        Some(Orientation {
            rotation,
            permutation,
        })
    }
}
