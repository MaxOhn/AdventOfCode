use std::{
    collections::{HashMap, VecDeque},
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
    println!("Setup: {:?}", start_.elapsed()); // 65µs

    let start_ = Instant::now();
    let p1 = part1(&map, start, end);
    println!("Part 1: {} [{:?}]", p1, start_.elapsed()); // 3.4ms

    let start_ = Instant::now();
    let p2 = part2(&map, start, end);
    println!("Part 2: {} [{:?}]", p2, start_.elapsed()); // 510ms

    assert_eq!(p1, 5756);
    assert_eq!(p2, 144_603);

    Ok(())
}

type Map = HashMap<Id, Vec<Id>>;

fn part1(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = Vec::with_capacity(8192);
    let mut queue = VecDeque::with_capacity(4092);
    queue.push_front((start, vec![start]));

    while let Some((node, path)) = queue.pop_back() {
        for &node in map.get(&node).unwrap() {
            if node == end {
                paths.push(path.clone());
            } else if is_valid_1(&path, node) {
                let mut path_ = path.clone();
                path_.push(node);
                queue.push_front((node, path_));
            }
        }
    }

    paths.len()
}

fn part2(map: &Map, start: Id, end: Id) -> usize {
    let mut paths = Vec::with_capacity(200_000);
    let mut buf = HashMap::with_capacity(8);
    let mut queue = VecDeque::with_capacity(75_000);
    queue.push_front((start, vec![start]));

    while let Some((node, path)) = queue.pop_back() {
        for &node in map.get(&node).unwrap() {
            if node == end {
                paths.push(path.clone());
            } else if is_valid_2(&path, node, &mut buf) {
                let mut path_ = path.clone();
                path_.push(node);
                queue.push_front((node, path_));
            }
        }
    }

    paths.len()
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Id(u8);

impl Id {
    fn new(mut id: u8, name: &str) -> Self {
        id |= ((name.as_bytes()[0] > b'Z') as u8) << 7;

        Self(id)
    }

    fn small(self) -> bool {
        self.0 >= 0b1000_0000
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

    let mut map: HashMap<_, Vec<_>> = HashMap::new();

    while input.read_line(&mut line)? != 0 {
        let (left, right) = line.trim_end().split_once('-').unwrap();

        let left = get_id(left);
        let right = get_id(right);

        map.entry(left).or_default().push(right);
        map.entry(right).or_default().push(left);

        line.clear();
    }

    let end = *ids.get("end").unwrap();
    map.remove(&end);

    let start = *ids.get("start").unwrap();

    for values in map.values_mut() {
        values.retain(|&value| value != start);
    }

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

        if buf.contains_key(&node) && buf.iter().any(|(_, count)| *count == 2) {
            return false;
        }
    }

    true
}
