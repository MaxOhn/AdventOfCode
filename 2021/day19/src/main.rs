use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    mem,
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
    let mut id = 0;

    while input.read_line(&mut line)? != 0 {
        let mut scanner = Scanner::new(id);
        id += 1;
        line.clear();

        while input.read_line(&mut line)? > 1 {
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
            line.clear();
        }

        scanner_queue.push_back(scanner);
    }

    let mut corrected = Vec::with_capacity(scanner_queue.len());
    corrected.push(scanner_queue.pop_front().unwrap());
    let mut compared = HashSet::new();

    'outer: while let Some(mut to_correct) = scanner_queue.pop_front() {
        for correct in &corrected {
            if !compared.insert((correct.id, to_correct.id)) {
                continue;
            }

            // do identity orientation first
            if let Some(offset) = correct.enough_overlaps(&to_correct) {
                to_correct.apply_offset(offset);
                to_correct.pos = offset;
                corrected.push(to_correct);

                continue 'outer;
            }

            let mut transformations = Transformations::new();

            // apply the 23 remaining orientations
            for _ in 0..23 {
                to_correct.apply_transformation(&mut transformations);

                if let Some(offset) = correct.enough_overlaps(&to_correct) {
                    to_correct.apply_offset(offset);
                    to_correct.pos = offset;
                    corrected.push(to_correct);

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
    println!("Elapsed: {:?}", elapsed); // 322ms

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

    fn apply_transformation(&mut self, transformations: &mut Transformations) {
        transformations.apply(&mut self.reports);
    }
}

// https://stackoverflow.com/questions/16452383/how-to-get-all-24-rotations-of-a-3-dimensional-array
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
                    mem::swap(&mut elem.y, &mut elem.z);
                    elem.z *= -1;
                }
            }
            Transformation::Turn => {
                for elem in elems {
                    mem::swap(&mut elem.x, &mut elem.y);
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
