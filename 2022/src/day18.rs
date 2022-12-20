use std::{
    cmp::Ordering,
    collections::{
        hash_map::{Entry, HashMap},
        HashSet,
    },
    ops::Add,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut cubes = parse_cubes(input)?;

    let p1 = part1(&cubes)?;
    let p2 = part2(&mut cubes)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_cubes(input: &str) -> Result<HashSet<Pos3>> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');

            let x = iter
                .next()
                .and_then(|x| x.parse().ok())
                .wrap_err("invalid x")?;
            let y = iter
                .next()
                .and_then(|y| y.parse().ok())
                .wrap_err("invalid y")?;
            let z = iter
                .next()
                .and_then(|z| z.parse().ok())
                .wrap_err("invalid z")?;

            Ok(Pos3 { x, y, z })
        })
        .collect()
}

fn generate_events(cubes: &HashSet<Pos3>) -> (Vec<Event>, Vec<Event>, Vec<Event>) {
    let mut hori = Vec::new();
    let mut verti = Vec::new();
    let mut depth = Vec::new();

    for Pos3 { x, y, z } in cubes.iter().copied() {
        hori.extend([Event::start(x, y, z), Event::end(x, y, z)]);
        verti.extend([Event::start(y, x, z), Event::end(y, x, z)]);
        depth.extend([Event::start(z, x, y), Event::end(z, x, y)]);
    }

    hori.sort_unstable();
    verti.sort_unstable();
    depth.sort_unstable();

    (hori, verti, depth)
}

fn part1(cubes: &HashSet<Pos3>) -> Result<i32> {
    let (hori, verti, depth) = generate_events(cubes);
    let mut bufs = Buffers::default();

    let sides_hori = count_sides_for_axis(&hori, &mut bufs, |_, _, _| ())?;
    let sides_verti = count_sides_for_axis(&verti, &mut bufs, |_, _, _| ())?;
    let sides_depth = count_sides_for_axis(&depth, &mut bufs, |_, _, _| ())?;

    Ok(sides_hori + sides_verti + sides_depth)
}

fn part2(cubes: &mut HashSet<Pos3>) -> Result<i32> {
    let mut minx = i8::MAX;
    let mut maxx = 0;

    let mut miny = i8::MAX;
    let mut maxy = 0;

    let mut minz = i8::MAX;
    let mut maxz = 0;

    for Pos3 { x, y, z } in cubes.iter().copied() {
        minx = minx.min(x);
        maxx = maxx.max(x);

        miny = miny.min(y);
        maxy = maxy.max(y);

        minz = minz.min(z);
        maxz = maxz.max(z);
    }

    // First do part 1 to get the hull of the cubes
    let (hori, verti, depth) = generate_events(&*cubes);

    let mut bufs = Buffers::default();
    let f = |hull: &mut Vec<Pos3>, axis: i8, pos: Pos| hull.push(Pos3::new(axis, pos.x, pos.y));

    let sides_hori = count_sides_for_axis(&hori, &mut bufs, f)?;
    let mut hull_set: HashSet<_> = bufs.hull.drain(..).collect();

    let sides_verti = count_sides_for_axis(&verti, &mut bufs, f)?;
    hull_set.extend(
        bufs.hull
            .drain(..)
            .map(|pos| Pos3::new(pos.y, pos.x, pos.z)),
    );

    let sides_depth = count_sides_for_axis(&depth, &mut bufs, f)?;
    hull_set.extend(
        bufs.hull
            .drain(..)
            .map(|pos| Pos3::new(pos.y, pos.z, pos.x)),
    );

    // For each cube in the hull, determine through
    // flooding if its outside or inside
    let mut outer = HashSet::new();
    let mut inner = HashSet::new();
    let mut seen = HashSet::new();
    let mut stack = bufs.hull;

    'next: for &pos in hull_set.iter() {
        stack.push(pos);

        while let Some(pos) = stack.pop() {
            if outer.contains(&pos) {
                outer.extend(seen.drain());
                stack.clear();

                continue 'next;
            } else if inner.contains(&pos) {
                inner.extend(seen.drain());
                stack.clear();

                continue 'next;
            } else if pos.x < minx
                || pos.x > maxx
                || pos.y < miny
                || pos.y > maxy
                || pos.z < minz
                || pos.z > maxz
            {
                outer.extend(seen.drain());
                stack.clear();

                continue 'next;
            } else if cubes.contains(&pos) || !seen.insert(pos) {
                continue;
            }

            const DIRECTIONS: [Pos3; 6] = [
                Pos3::new(-1, 0, 0),
                Pos3::new(1, 0, 0),
                Pos3::new(0, -1, 0),
                Pos3::new(0, 1, 0),
                Pos3::new(0, 0, -1),
                Pos3::new(0, 0, 1),
            ];

            let neighbors = DIRECTIONS.iter().map(|&offset| pos + offset);
            stack.extend(neighbors);
        }

        inner.extend(seen.drain());
    }

    // Collect *all* blocks within the min-max points
    let all = (minx..=maxx).flat_map(|x| {
        (miny..=maxy).flat_map(move |y| (minz..=maxz).map(move |z| Pos3 { x, y, z }))
    });

    cubes.extend(all);

    // Remove the cubes belonging to inner chambers
    cubes.retain(|pos| !inner.contains(pos));

    // Use p1 and subtract the sides of the block
    // except the ones on the outside of the block
    let p1 = sides_hori + sides_verti + sides_depth;
    let block_sides = part1(cubes)?;

    let wx = (maxx - minx) as i32 + 1;
    let wy = (maxy - miny) as i32 + 1;
    let wz = (maxz - minz) as i32 + 1;

    let hori = 2 * wy * wz;
    let verti = 2 * wx * wz;
    let depth = 2 * wx * wy;
    let outer = hori + verti + depth;

    Ok(p1 - (block_sides - outer))
}

fn count_sides_for_axis<F>(events: &[Event], bufs: &mut Buffers, f: F) -> Result<i32>
where
    F: Fn(&mut Vec<Pos3>, i8, Pos),
{
    let Buffers { curr_cubes, hull } = bufs;
    let mut open_sides = 0;

    for Event { axis, pos, kind } in events {
        match kind {
            EventKind::Start => match curr_cubes.entry(*pos) {
                Entry::Occupied(e) => *e.into_mut() += 1,
                Entry::Vacant(e) => {
                    e.insert(1);
                    open_sides += 1;

                    (f)(hull, *axis - 1, *pos);
                }
            },
            EventKind::End => match curr_cubes.entry(*pos) {
                Entry::Occupied(e) if *e.get() > 1 => *e.into_mut() -= 1,
                Entry::Occupied(e) => {
                    e.remove();
                    open_sides += 1;

                    (f)(hull, *axis + 1, *pos);
                }
                Entry::Vacant(_) => bail!("unexpected end event"),
            },
        }
    }

    Ok(open_sides)
}

#[derive(Default)]
struct Buffers {
    curr_cubes: HashMap<Pos, u8>,
    hull: Vec<Pos3>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i8,
    y: i8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Pos3 {
    const fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }
}

impl Add for Pos3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Event {
    axis: i8,
    pos: Pos,
    kind: EventKind,
}

impl Event {
    fn start(axis: i8, x: i8, y: i8) -> Self {
        Self {
            axis,
            pos: Pos { x, y },
            kind: EventKind::Start,
        }
    }

    fn end(axis: i8, x: i8, y: i8) -> Self {
        Self {
            axis,
            pos: Pos { x, y },
            kind: EventKind::End,
        }
    }
}

impl Ord for Event {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.kind, other.kind) {
            (EventKind::Start, EventKind::End) if self.axis == other.axis + 1 => Ordering::Less,
            (EventKind::End, EventKind::Start) if self.axis + 1 == other.axis => Ordering::Greater,
            _ => self
                .axis
                .cmp(&other.axis)
                .then_with(|| self.kind.cmp(&other.kind)) // !!!!
                .then_with(|| self.pos.cmp(&other.pos)),
        }
    }
}

impl PartialOrd for Event {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    Start,
    End,
}
