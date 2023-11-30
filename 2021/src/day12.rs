use std::collections::HashMap;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let (map, start, end) = parse_input(input)?;

    let p1 = part1(&map, start, end);
    let p2 = part2(&map, start, end);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = 0;
    let mut stack = vec![(start, vec![start], 1)];

    while let Some((cave, path, factor)) = stack.pop() {
        for &(cave, count) in map.get(cave) {
            if cave == end {
                paths += factor * count;
            } else if is_valid_1(&path, cave) {
                let mut path_ = path.clone();
                path_.push(cave);
                stack.push((cave, path_, factor * count));
            }
        }
    }

    paths
}

fn part2(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = 0;
    let mut buf = HashMap::new();
    let mut stack = vec![(start, vec![start], 1)];

    while let Some((cave, path, factor)) = stack.pop() {
        for &(cave, count) in map.get(cave) {
            if cave == end {
                paths += factor * count;
            } else if is_valid_2(&path, cave, &mut buf) {
                let mut path_ = path.clone();
                path_.push(cave);
                stack.push((cave, path_, factor * count));
            }
        }
    }

    paths
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Id(u8);

impl Id {
    const SMALL: u8 = 0b1000_0000;

    fn new(mut id: u8, name: &str) -> Self {
        id |= ((name.as_bytes()[0] > b'Z') as u8) << 7;

        Self(id)
    }

    fn is_small(self) -> bool {
        self.0 >= Self::SMALL
    }
}

// Assuming there are no more than 20 caves
struct Map([Vec<(Id, usize)>; 20]);

impl Map {
    fn new() -> Self {
        let arr = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        Self(arr)
    }

    fn get(&self, id: Id) -> &[(Id, usize)] {
        unsafe { self.0.get_unchecked((id.0 & !Id::SMALL) as usize) }
    }

    fn get_mut(&mut self, id: Id) -> &mut Vec<(Id, usize)> {
        unsafe { self.0.get_unchecked_mut((id.0 & !Id::SMALL) as usize) }
    }
}

fn parse_input(input: &str) -> Result<(Map, Id, Id)> {
    let mut ids = HashMap::new();
    let mut curr_id = 0;

    let mut get_id = |s: &str| match ids.get(s) {
        Some(id) => *id,
        None => {
            let id = Id::new(curr_id, s);
            ids.insert(s.to_owned(), id);
            curr_id += 1;

            id
        }
    };

    let mut map = Map::new();

    for line in input.lines() {
        let (left, right) = line.trim_end().split_once('-').unwrap();

        let left = get_id(left);
        let right = get_id(right);

        map.get_mut(left).push((right, 1));
        map.get_mut(right).push((left, 1));
    }

    let start = *ids.get("start").unwrap();

    for values in &mut map.0 {
        values.retain(|(value, _)| *value != start);
    }

    let end = *ids.get("end").unwrap();

    // Remove big caves and connect small caves directly instead
    for from in (0..map.0.len() as u8).map(Id) {
        for (cave, _) in map.get(from).to_vec() {
            if !cave.is_small() {
                // Remove path to big cave
                {
                    let reachable = map.get_mut(from);
                    let idx = reachable.iter().position(|(id, _)| *id == cave).unwrap();
                    reachable.remove(idx);
                }

                // Add paths to everything that was reachable from the big cave
                for to in map.get(cave).iter().map(|(id, _)| *id).collect::<Vec<_>>() {
                    extend(&mut map, from, to);
                }
            }
        }
    }

    // The paths currently contain duplicates.
    // Filter out those duplicates and use the count element instead.
    for id in (0..map.0.len() as u8).map(Id) {
        let mut reduced = Vec::new();

        while let Some((curr, _)) = map.get_mut(id).pop() {
            let count = map.get(id).iter().filter(|(id, _)| *id == curr).count();
            reduced.push((curr, count + 1));
            map.get_mut(id).retain(|(id, _)| *id != curr);
        }

        map.0[id.0 as usize] = reduced;
    }

    Ok((map, start, end))
}

// Add path in map between two caves.
// If the destination is a big cave, add paths to its reachable caves instead.
fn extend(map: &mut Map, from: Id, to: Id) {
    if to.is_small() {
        map.get_mut(from).push((to, 1));
    } else {
        for cave in map.get(to).iter().map(|(id, _)| *id).collect::<Vec<_>>() {
            extend(map, from, cave);
        }
    }
}

fn is_valid_1(path: &[Id], cave: Id) -> bool {
    !path.contains(&cave)
}

fn is_valid_2(path: &[Id], cave: Id, buf: &mut HashMap<Id, usize>) -> bool {
    buf.clear();

    for &cave in path {
        *buf.entry(cave).or_default() += 1;
    }

    !(buf.contains_key(&cave) && buf.iter().any(|(_, count)| *count == 2))
}
