use std::collections::HashSet;
use std::fmt;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Not};
use std::time::Instant;
use util::NumHasherBuilder;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty = 0,
    Occupied = 1,
    Floor = 2,
}

#[derive(Clone)]
struct Seats {
    width: usize,
    seats: Vec<Seat>,
}

fn main() {
    let seats = parse_seats();

    let p1 = part1(seats.clone());
    let p2 = part2(seats);

    assert_eq!(p1, 2166);
    assert_eq!(p2, 1955);
}

fn parse_seats() -> Seats {
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let _ = input.read_line(&mut line);

    let mut seats: Seats = line
        .trim_end()
        .chars()
        .map(|c| {
            if c == 'L' {
                Seat::Occupied
            } else {
                Seat::Floor
            }
        })
        .collect();

    let width = seats.width;
    seats.reserve(width * 90);
    line.clear();

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();
        let mut i = 0;

        while i < width {
            let seat = match unsafe { *bytes.get_unchecked(i) } {
                b'L' => Seat::Occupied,
                b'.' => Seat::Floor,
                _ => unsafe { unreachable_unchecked() },
            };

            seats.push(seat);
            i += 1;
        }

        line.clear();
    }

    seats
}

const EMP: Seat = Seat::Empty;
const OCC: Seat = Seat::Occupied;
const FLOOR: Seat = Seat::Floor;

fn part1(mut seats: Seats) -> usize {
    let start = Instant::now();

    let mut flipped = HashSet::with_capacity_and_hasher(4096, NumHasherBuilder);
    let mut stationary = HashSet::with_capacity_and_hasher(seats.len(), NumHasherBuilder);

    loop {
        // First row
        let mut i = 0;
        while i < seats.width {
            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR || stationary.contains(&i) {
                i += 1;
                continue;
            }

            let mut count = (unsafe { *seats.get_unchecked(i + seats.width) } == OCC) as u8;

            // Right
            let j = i + 1;
            if j < seats.width {
                count += (unsafe { *seats.get_unchecked(j) } == OCC) as u8
                    + (unsafe { *seats.get_unchecked(j + seats.width) } == OCC) as u8;
            }

            // Left
            if i > 0 {
                let j = i - 1;

                if flipped.contains(&j) {
                    count += (unsafe { *seats.get_unchecked(j) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j) == OCC }) as u8;
                }
                count += (unsafe { *seats.get_unchecked(j + seats.width) } == OCC) as u8;
            }

            if (seat == EMP && count == 0) || (seat == OCC && count >= 4) {
                unsafe { *seats.get_unchecked_mut(i) = !seat };
                flipped.insert(i);
            } else {
                stationary.insert(i);
            }

            i += 1;
        }

        let last = seats.len() - seats.width;

        // Middle rows
        while i < last {
            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR || stationary.contains(&i) {
                i += 1;
                continue;
            }

            let mut count = (unsafe { *seats.get_unchecked(i + seats.width) } == OCC) as u8;

            if flipped.contains(&(i - seats.width)) {
                count += (unsafe { *seats.get_unchecked(i - seats.width) == EMP }) as u8;
            } else {
                count += (unsafe { *seats.get_unchecked(i - seats.width) == OCC }) as u8;
            }

            // Right
            let j = i + 1;
            if j % seats.width > 0 {
                count += (unsafe { *seats.get_unchecked(j) } == OCC) as u8
                    + (unsafe { *seats.get_unchecked(j + seats.width) } == OCC) as u8;

                if flipped.contains(&(j - seats.width)) {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == OCC }) as u8;
                }
            }

            // Left
            if i % seats.width > 0 {
                let j = i - 1;
                count += (unsafe { *seats.get_unchecked(j + seats.width) } == OCC) as u8;

                if flipped.contains(&(j - seats.width)) {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == OCC }) as u8;
                }

                if flipped.contains(&j) {
                    count += (unsafe { *seats.get_unchecked(j) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j) == OCC }) as u8;
                }
            }

            if (seat == EMP && count == 0) || (seat == OCC && count >= 4) {
                unsafe { *seats.get_unchecked_mut(i) = !seat };
                flipped.insert(i);
            } else {
                stationary.insert(i);
            }

            i += 1;
        }

        // Last row
        while i < seats.len() {
            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR || stationary.contains(&i) {
                i += 1;
                continue;
            }

            let mut count = 0;

            if flipped.contains(&(i - seats.width)) {
                count += (unsafe { *seats.get_unchecked(i - seats.width) == EMP }) as u8;
            } else {
                count += (unsafe { *seats.get_unchecked(i - seats.width) == OCC }) as u8;
            }

            // Right
            let j = i + 1;
            if j < seats.len() {
                count += (unsafe { *seats.get_unchecked(j) } == OCC) as u8;

                if flipped.contains(&(j - seats.width)) {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == OCC }) as u8;
                }
            }

            // Left
            if i % seats.width > 0 {
                let j = i - 1;

                if flipped.contains(&(j - seats.width)) {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j - seats.width) == OCC }) as u8;
                }

                if flipped.contains(&j) {
                    count += (unsafe { *seats.get_unchecked(j) == EMP }) as u8;
                } else {
                    count += (unsafe { *seats.get_unchecked(j) == OCC }) as u8;
                }
            }

            if (seat == EMP && count == 0) || (seat == OCC && count >= 4) {
                unsafe { *seats.get_unchecked_mut(i) = !seat };
                flipped.insert(i);
            } else {
                stationary.insert(i);
            }

            i += 1;
        }
        if flipped.is_empty() {
            let count = seats
                .iter()
                .filter(|seats| **seats == Seat::Occupied)
                .count();

            println!("Part 1: {} [{:?}]", count, start.elapsed()); // 14ms
            return count;
        }

        flipped.clear();
    }
}

static OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn part2(mut seats: Seats) -> usize {
    let start = Instant::now();

    let width = seats.width as isize;
    let height = seats.len() as isize / width;

    let mut alt = seats.clone();
    let mut stationary = HashSet::with_capacity_and_hasher(4096, NumHasherBuilder);

    loop {
        let mut i = 0;

        while i < seats.len() {
            if stationary.contains(&i) {
                unsafe { *alt.get_unchecked_mut(i) = *seats.get_unchecked(i) }
                i += 1;
                continue;
            }

            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR {
                stationary.insert(i);
                i += 1;
                continue;
            }

            let directions = OFFSETS.iter().map(|(dx, dy)| {
                let mut cx = (i % seats.width) as isize;
                let mut cy = (i / seats.width) as isize;
                std::iter::from_fn(move || {
                    cx += dx;
                    cy += dy;

                    if 0 <= cx && cx < width && 0 <= cy && cy < height {
                        Some((cx + width * cy) as usize)
                    } else {
                        None
                    }
                })
            });

            let mut count = 0;

            'outer: for direction_iter in directions {
                for j in direction_iter {
                    match unsafe { *seats.get_unchecked(j) } {
                        Seat::Empty => continue 'outer,
                        Seat::Occupied => {
                            count += 1;
                            continue 'outer;
                        }
                        Seat::Floor => (),
                    }
                }
            }

            if seat == EMP && count == 0 {
                unsafe { *alt.get_unchecked_mut(i) = OCC }
            } else if seat == OCC && count >= 5 {
                unsafe { *alt.get_unchecked_mut(i) = EMP }
            } else {
                unsafe { *alt.get_unchecked_mut(i) = *seats.get_unchecked(i) }
                stationary.insert(i);
            }

            i += 1;
        }

        if stationary.len() == seats.len() {
            let count = seats
                .iter()
                .filter(|seats| **seats == Seat::Occupied)
                .count();

            println!("Part 2: {} [{:?}]", count, start.elapsed()); //
            return count;
        }

        std::mem::swap(&mut seats, &mut alt);
    }
}

#[allow(dead_code)]
fn part2_old(mut seats: Seats) -> usize {
    let start = Instant::now();
    let mut flipped = HashSet::with_capacity_and_hasher(4096, NumHasherBuilder);
    let mut stationary = HashSet::with_capacity_and_hasher(seats.len(), NumHasherBuilder);

    let emp_check = |count: u8| count == 0;
    let occ_check = |count: u8| count >= 5;

    loop {
        // First row
        let mut i = 0;
        while i < seats.width {
            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR || stationary.contains(&i) {
                i += 1;
                continue;
            }

            let mut count = down(&seats, i);

            // Right
            if i + 1 < seats.width {
                count += right(&seats, i) + down_right(&seats, i);
            }

            // Left
            if i > 0 {
                count += left(&seats, &flipped, i) + down_left(&seats, i);
            }

            if (seat == EMP && emp_check(count)) || (seat == OCC && occ_check(count)) {
                unsafe { *seats.get_unchecked_mut(i) = !seat };
                flipped.insert(i);
            } else {
                stationary.insert(i);
            }

            i += 1;
        }

        // Other rows
        while i < seats.len() {
            let seat = unsafe { *seats.get_unchecked(i) };

            if seat == FLOOR || stationary.contains(&i) {
                i += 1;
                continue;
            }

            let mut count = down(&seats, i) + up(&seats, &flipped, i);

            // Right
            if (i + 1) % seats.width > 0 {
                count += right(&seats, i) + down_right(&seats, i) + up_right(&seats, &flipped, i);
            }

            // Left
            if i % seats.width > 0 {
                count +=
                    down_left(&seats, i) + left(&seats, &flipped, i) + up_left(&seats, &flipped, i);
            }

            if (seat == EMP && emp_check(count)) || (seat == OCC && occ_check(count)) {
                unsafe { *seats.get_unchecked_mut(i) = !seat };
                flipped.insert(i);
            } else {
                stationary.insert(i);
            }

            i += 1;
        }

        if flipped.is_empty() {
            let count = seats
                .iter()
                .filter(|seats| **seats == Seat::Occupied)
                .count();

            println!("Part 2 old: {} [{:?}]", count, start.elapsed()); // 24ms
            return count;
        }

        flipped.clear();
    }
}

// ---- Part 2 helpers -----

#[inline(always)]
fn right(seats: &Seats, i: usize) -> u8 {
    let mut j = i + 1;

    loop {
        match unsafe { *seats.get_unchecked(j) } {
            Seat::Empty => return 0,
            Seat::Occupied => return 1,
            Seat::Floor => {
                let next = j + 1;

                if j % seats.width > next % seats.width {
                    return 0;
                }

                j = next;
            }
        }
    }
}

#[inline(always)]
fn down(seats: &Seats, i: usize) -> u8 {
    let mut j = i + seats.width;

    loop {
        match unsafe { *seats.get_unchecked(j) } {
            Seat::Empty => return 0,
            Seat::Occupied => return 1,
            Seat::Floor => {
                let next = j + seats.width;

                if next >= seats.len() {
                    return 0;
                }

                j = next;
            }
        }
    }
}

#[inline(always)]
fn down_right(seats: &Seats, i: usize) -> u8 {
    let mut j = i + seats.width + 1;

    loop {
        match unsafe { *seats.get_unchecked(j) } {
            Seat::Empty => return 0,
            Seat::Occupied => return 1,
            Seat::Floor => {
                if (j + 1) % seats.width == 0 {
                    return 0;
                }

                j += seats.width + 1;

                if j >= seats.len() {
                    return 0;
                }
            }
        }
    }
}

#[inline(always)]
fn down_left(seats: &Seats, i: usize) -> u8 {
    let mut j = i + seats.width - 1;

    loop {
        match unsafe { *seats.get_unchecked(j) } {
            Seat::Empty => return 0,
            Seat::Occupied => return 1,
            Seat::Floor => {
                if j % seats.width == 0 {
                    return 0;
                }

                j += seats.width - 1;

                if j >= seats.len() {
                    return 0;
                }
            }
        }
    }
}

#[inline(always)]
fn left(seats: &Seats, flipped: &HashSet<usize, NumHasherBuilder>, i: usize) -> u8 {
    let mut j = i - 1;

    loop {
        let seat = unsafe { *seats.get_unchecked(j) };

        if seat == FLOOR {
            if j % seats.width == 0 {
                return 0;
            }

            j -= 1;
        } else {
            return if flipped.contains(&j) {
                (seat == EMP) as u8
            } else {
                (seat == OCC) as u8
            };
        }
    }
}

#[inline(always)]
fn up(seats: &Seats, flipped: &HashSet<usize, NumHasherBuilder>, i: usize) -> u8 {
    let mut j = i - seats.width;

    loop {
        let seat = unsafe { *seats.get_unchecked(j) };

        if seat == FLOOR {
            if j < seats.width {
                return 0;
            }

            j -= seats.width;
        } else {
            return if flipped.contains(&j) {
                (seat == EMP) as u8
            } else {
                (seat == OCC) as u8
            };
        }
    }
}

#[inline(always)]
fn up_right(seats: &Seats, flipped: &HashSet<usize, NumHasherBuilder>, i: usize) -> u8 {
    let mut j = i + 1 - seats.width;

    loop {
        let seat = unsafe { *seats.get_unchecked(j) };

        if seat == FLOOR {
            if j < seats.width || (j + 1) % seats.width == 0 {
                return 0;
            }

            j = j + 1 - seats.width;
        } else {
            return if flipped.contains(&j) {
                (seat == EMP) as u8
            } else {
                (seat == OCC) as u8
            };
        }
    }
}

#[inline(always)]
fn up_left(seats: &Seats, flipped: &HashSet<usize, NumHasherBuilder>, i: usize) -> u8 {
    let mut j = i - 1 - seats.width;

    loop {
        let seat = unsafe { *seats.get_unchecked(j) };

        if seat == FLOOR {
            if j < seats.width || j % seats.width == 0 {
                return 0;
            }

            j = j - 1 - seats.width;
        } else {
            return if flipped.contains(&j) {
                (seat == EMP) as u8
            } else {
                (seat == OCC) as u8
            };
        }
    }
}

// ----- Impls -----

impl Not for Seat {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Empty => Self::Occupied,
            Self::Occupied => Self::Empty,
            Self::Floor => unsafe { unreachable_unchecked() },
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seat = match self {
            Self::Empty => "L",
            Self::Occupied => "#",
            Self::Floor => ".",
        };

        f.write_str(seat)
    }
}

impl Deref for Seats {
    type Target = Vec<Seat>;

    fn deref(&self) -> &Self::Target {
        &self.seats
    }
}

impl DerefMut for Seats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seats
    }
}

impl FromIterator<Seat> for Seats {
    fn from_iter<I: IntoIterator<Item = Seat>>(iter: I) -> Self {
        let iter = iter.into_iter();

        let width = iter
            .size_hint()
            .1
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        let mut seats = Vec::with_capacity(width);

        for seat in iter {
            seats.push(seat);
        }

        Seats { width, seats }
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = self.seats.chunks_exact(self.width);

        if let Some(first) = rows.next() {
            for seat in first {
                write!(f, "{}", seat)?;
            }

            for row in rows {
                f.write_str("\n")?;

                for seat in row {
                    write!(f, "{}", seat)?;
                }
            }
        }

        Ok(())
    }
}
