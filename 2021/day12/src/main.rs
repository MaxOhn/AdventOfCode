use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

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
    let start_ = Instant::now();
    let (map, start, end) = parse_input()?;
    println!("Setup: {:?}", start_.elapsed()); // 63µs

    let start_ = Instant::now();
    let p1 = part1(&map, start, end);
    println!("Part 1: {} [{:?}]", p1, start_.elapsed()); // 2.9ms

    let start_ = Instant::now();
    let p2 = part2(&map, start, end);
    println!("Part 2: {} [{:?}]", p2, start_.elapsed()); // 485ms

    assert_eq!(p1, 5756);
    assert_eq!(p2, 144_603);

    Ok(())
}

fn part1(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = Vec::with_capacity(8192);
    let mut stack = vec![(start, vec![start])];

    while let Some((node, path)) = stack.pop() {
        for &node in map.get(node) {
            if node == end {
                paths.push(path.clone());
            } else if is_valid_1(&path, node) {
                let mut path_ = path.clone();
                path_.push(node);
                stack.push((node, path_));
            }
        }
    }

    paths.len()
}

fn part2(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = Vec::with_capacity(200_000);
    let mut buf = HashMap::new();
    let mut stack = vec![(start, vec![start])];

    while let Some((node, path)) = stack.pop() {
        for &node in map.get(node) {
            if node == end {
                paths.push(path.clone());
            } else if is_valid_2(&path, node, &mut buf) {
                let mut path_ = path.clone();
                path_.push(node);
                stack.push((node, path_));
            }
        }
    }

    paths.len()
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Id(u8);

impl Id {
    const SMALL: u8 = 0b1000_0000;

    fn new(mut id: u8, name: &str) -> Self {
        id |= ((name.as_bytes()[0] > b'Z') as u8) << 7;

        Self(id)
    }

    fn small(self) -> bool {
        self.0 >= Self::SMALL
    }
}

// Assuming there are no more than 20 caves
struct Map([Vec<Id>; 20]);

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

    fn get(&self, id: Id) -> &[Id] {
        unsafe { self.0.get_unchecked((id.0 & !Id::SMALL) as usize) }
    }

    fn get_mut(&mut self, id: Id) -> &mut Vec<Id> {
        unsafe { self.0.get_unchecked_mut((id.0 & !Id::SMALL) as usize) }
    }
}

fn parse_input() -> Result<(Map, Id, Id), Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);
    let mut line = String::new();

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

    while input.read_line(&mut line)? != 0 {
        let (left, right) = line.trim_end().split_once('-').unwrap();

        let left = get_id(left);
        let right = get_id(right);

        map.get_mut(left).push(right);
        map.get_mut(right).push(left);

        line.clear();
    }

    let start = *ids.get("start").unwrap();

    for values in &mut map.0 {
        values.retain(|&value| value != start);
    }

    let end = *ids.get("end").unwrap();

    Ok((map, start, end))
}

fn is_valid_1(path: &[Id], node: Id) -> bool {
    !(node.small() && path.contains(&node))
}

fn is_valid_2(path: &[Id], node: Id, buf: &mut HashMap<Id, usize>) -> bool {
    if node.small() {
        buf.clear();

        for &elem in path {
            if elem.small() {
                *buf.entry(elem).or_default() += 1;
            }
        }

        return !(buf.contains_key(&node) && buf.iter().any(|(_, count)| *count == 2));
    }

    true
}
