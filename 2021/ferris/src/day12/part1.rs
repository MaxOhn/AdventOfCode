use std::collections::HashMap;

pub fn run(input: &[u8]) -> i64 {
    let (map, start, end) = parse_input(input);

    let mut paths = Vec::with_capacity(8192);
    let mut stack = vec![(start, vec![start])];

    while let Some((cave, path)) = stack.pop() {
        for &cave in map.get(cave) {
            if cave == end {
                paths.push(path.clone());
            } else if !cave.is_small() {
                stack.push((cave, path.clone()));
            } else if is_valid(&path, cave) {
                let mut path_ = path.clone();
                path_.push(cave);
                stack.push((cave, path_));
            }
        }
    }

    paths.len() as i64
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

fn parse_input(input: &[u8]) -> (Map, Id, Id) {
    let input = unsafe { std::str::from_utf8_unchecked(input) };

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
        let (left, right) = line.split_once('-').unwrap();

        let left = get_id(left);
        let right = get_id(right);

        map.get_mut(left).push(right);
        map.get_mut(right).push(left);
    }

    let start = *ids.get("start").unwrap();

    for values in &mut map.0 {
        values.retain(|&value| value != start);
    }

    let end = *ids.get("end").unwrap();

    (map, start, end)
}

#[inline(always)]
fn is_valid(path: &[Id], cave: Id) -> bool {
    !path.contains(&cave)
}
