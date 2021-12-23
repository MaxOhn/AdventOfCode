use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    error::Error,
    fmt,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    time::Instant,
};

use rustc_hash::FxHashMap as HashMap;

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
    let p1 = part1()?;
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 87ms

    let start = Instant::now();
    let p2 = part2()?;
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 118ms

    assert_eq!(p1, 16_506);
    assert_eq!(p2, 48_304);

    Ok(())
}

fn part1() -> Result<u32, Box<dyn Error>> {
    Ok(solve(Burrow::<2>::from_input()?))
}

fn part2() -> Result<u32, Box<dyn Error>> {
    Ok(solve(Burrow::<4>::from_input()?))
}

fn solve<const N: usize>(burrow: Burrow<N>) -> u32 {
    let mut dists = Distances::default();
    dists.set(&burrow, 0);

    let mut heap = BinaryHeap::new();
    heap.push(State::new(burrow));

    let mut successor_buf = Vec::new();

    while let Some(State { burrow, .. }) = heap.pop() {
        let energy = burrow.energy();

        if burrow.is_sorted() {
            // println!("Path:");
            // for burrow in path {
            //     println!("{}\n---", burrow);
            // }

            return energy;
        } else if energy > dists.get(&burrow) {
            continue;
        }

        burrow.successors(&mut successor_buf);

        for successor in successor_buf.drain(..) {
            let energy = successor.energy();

            if energy < dists.get(&successor) {
                dists.set(&successor, energy);

                // let mut path = path.clone();
                // path.push(successor.clone());

                let state = State {
                    burrow: successor,
                    // path,
                };

                heap.push(state);
            }
        }
    }

    unreachable!()
}

#[derive(Eq, PartialEq)]
struct State<const N: usize> {
    burrow: Burrow<N>,
    // path: Vec<Burrow<N>>,
}

impl<const N: usize> State<N> {
    fn new(burrow: Burrow<N>) -> Self {
        Self {
            // path: vec![burrow.clone()],
            burrow,
        }
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.burrow.cmp(&other.burrow)
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct Distances<const N: usize>(HashMap<Burrow<N>, u32>);

impl<const N: usize> Distances<N> {
    fn get(&mut self, burrow: &Burrow<N>) -> u32 {
        if let Some(dist) = self.0.get(burrow) {
            *dist
        } else {
            *self.0.entry(burrow.to_owned()).or_insert(u32::MAX)
        }
    }

    fn set(&mut self, burrow: &Burrow<N>, dist: u32) {
        if let Some(value) = self.0.get_mut(burrow) {
            *value = dist;
        } else {
            self.0.insert(burrow.to_owned(), dist);
        }
    }
}

#[derive(Clone, Eq)]
pub struct Burrow<const N: usize> {
    hallway: [Field; 11],
    rooms: [Room<N>; 4],
    energy: u32,
}

static ROOM_OPENINGS: [usize; 4] = [2, 4, 6, 8];

impl Burrow<2> {
    pub fn from_input() -> Result<Self, Box<dyn Error>> {
        let file = File::open("./input")?;
        let mut input = BufReader::new(file);

        let mut line = String::new();
        let mut burrow = Burrow::new();

        input.read_line(&mut line)?;
        input.read_line(&mut line)?;
        line.clear();
        input.read_line(&mut line)?;

        burrow.rooms[0].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[3]));
        burrow.rooms[1].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[5]));
        burrow.rooms[2].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[7]));
        burrow.rooms[3].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[9]));

        line.clear();
        input.read_line(&mut line)?;

        burrow.rooms[0].positions[1].amphipod = Some(Amphipod::from_byte(line.as_bytes()[3]));
        burrow.rooms[1].positions[1].amphipod = Some(Amphipod::from_byte(line.as_bytes()[5]));
        burrow.rooms[2].positions[1].amphipod = Some(Amphipod::from_byte(line.as_bytes()[7]));
        burrow.rooms[3].positions[1].amphipod = Some(Amphipod::from_byte(line.as_bytes()[9]));

        Ok(burrow)
    }
}

impl Burrow<4> {
    pub fn from_input() -> Result<Self, Box<dyn Error>> {
        let file = File::open("./input")?;
        let mut input = BufReader::new(file);

        let mut line = String::new();
        let mut burrow = Burrow::new();

        input.read_line(&mut line)?;
        input.read_line(&mut line)?;
        line.clear();
        input.read_line(&mut line)?;

        burrow.rooms[0].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[3]));
        burrow.rooms[1].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[5]));
        burrow.rooms[2].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[7]));
        burrow.rooms[3].positions[0].amphipod = Some(Amphipod::from_byte(line.as_bytes()[9]));

        line.clear();
        input.read_line(&mut line)?;

        burrow.rooms[0].positions[3].amphipod = Some(Amphipod::from_byte(line.as_bytes()[3]));
        burrow.rooms[1].positions[3].amphipod = Some(Amphipod::from_byte(line.as_bytes()[5]));
        burrow.rooms[2].positions[3].amphipod = Some(Amphipod::from_byte(line.as_bytes()[7]));
        burrow.rooms[3].positions[3].amphipod = Some(Amphipod::from_byte(line.as_bytes()[9]));

        burrow.rooms[0].positions[1].amphipod = Some(Amphipod::D);
        burrow.rooms[1].positions[1].amphipod = Some(Amphipod::C);
        burrow.rooms[2].positions[1].amphipod = Some(Amphipod::B);
        burrow.rooms[3].positions[1].amphipod = Some(Amphipod::A);

        burrow.rooms[0].positions[2].amphipod = Some(Amphipod::D);
        burrow.rooms[1].positions[2].amphipod = Some(Amphipod::B);
        burrow.rooms[2].positions[2].amphipod = Some(Amphipod::A);
        burrow.rooms[3].positions[2].amphipod = Some(Amphipod::C);

        Ok(burrow)
    }
}

impl<const N: usize> Burrow<N> {
    fn new() -> Self {
        Self {
            hallway: [Field::default(); 11],
            rooms: [
                Room::new(Amphipod::A),
                Room::new(Amphipod::B),
                Room::new(Amphipod::C),
                Room::new(Amphipod::D),
            ],
            energy: 0,
        }
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn is_sorted(&self) -> bool {
        self.rooms.iter().all(Room::is_sorted)
    }

    fn room(&self, kind: Amphipod) -> &Room<N> {
        let idx = match kind {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        };

        &self.rooms[idx]
    }

    fn room_mut(&mut self, kind: Amphipod) -> &mut Room<N> {
        let idx = match kind {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        };

        &mut self.rooms[idx]
    }

    pub fn successors(&self, buf: &mut Vec<Burrow<N>>) {
        for (room_idx, room) in self.rooms.iter().enumerate() {
            'room: for i in 0..room.positions.len() {
                if let Some(_) = room.positions[i].amphipod.as_ref() {
                    if !room.is_clear() {
                        if room
                            .positions
                            .iter()
                            .take(i)
                            .any(|field| field.amphipod.is_some())
                        {
                            continue 'room;
                        }

                        let hallway_idx = 2 + 2 * room_idx;

                        // Going into hallway left
                        for j in (0..hallway_idx).rev() {
                            if ROOM_OPENINGS.contains(&j) {
                                continue;
                            }

                            if self.hallway[j].amphipod.is_none() {
                                let mut burrow = self.to_owned();

                                let amphipod =
                                    burrow.rooms[room_idx].positions[i].amphipod.take().unwrap();

                                burrow.energy +=
                                    (i + 1 + (hallway_idx - j)) as u32 * amphipod.energy();
                                burrow.hallway[j].amphipod = Some(amphipod);
                                buf.push(burrow);
                            } else {
                                break;
                            }
                        }

                        // Going into hallway right
                        for j in hallway_idx + 1..self.hallway.len() {
                            if ROOM_OPENINGS.contains(&j) {
                                continue;
                            }

                            if self.hallway[j].amphipod.is_none() {
                                let mut burrow = self.to_owned();

                                let amphipod =
                                    burrow.rooms[room_idx].positions[i].amphipod.take().unwrap();

                                burrow.energy +=
                                    (i + 1 + (j - hallway_idx)) as u32 * amphipod.energy();
                                burrow.hallway[j].amphipod = Some(amphipod);
                                buf.push(burrow);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        'hallway: for hallway_idx in 0..self.hallway.len() {
            if let Some(amphipod) = self.hallway[hallway_idx].amphipod.as_ref() {
                let room = self.room(*amphipod);

                if !room.is_clear() || room.positions[0].amphipod.is_some() {
                    continue;
                }

                let target_idx = match amphipod {
                    Amphipod::A => 2,
                    Amphipod::B => 4,
                    Amphipod::C => 6,
                    Amphipod::D => 8,
                };

                if target_idx < hallway_idx {
                    for j in target_idx + 1..hallway_idx {
                        if self.hallway[j].amphipod.is_some() {
                            continue 'hallway;
                        }
                    }
                } else {
                    for j in hallway_idx + 1..target_idx {
                        if self.hallway[j].amphipod.is_some() {
                            continue 'hallway;
                        }
                    }
                }

                let last_free = room
                    .positions
                    .iter()
                    .enumerate()
                    .take_while(|(_, field)| field.amphipod.is_none())
                    .last();

                if let Some((idx, _)) = last_free {
                    let mut burrow = self.to_owned();
                    let amphipod = burrow.hallway[hallway_idx].amphipod.take().unwrap();

                    let steps =
                        idx as u32 + 1 + (hallway_idx as isize - target_idx as isize).abs() as u32;

                    burrow.energy += steps * amphipod.energy();
                    burrow.room_mut(amphipod).positions[idx].amphipod = Some(amphipod);
                    buf.push(burrow);
                }
            }
        }
    }
}

impl<const N: usize> PartialEq for Burrow<N> {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}

impl<const N: usize> Hash for Burrow<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hallway.hash(state);
        self.rooms.hash(state);
    }
}

impl<const N: usize> Ord for Burrow<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy().cmp(&self.energy())
    }
}

impl<const N: usize> PartialOrd for Burrow<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Field {
    amphipod: Option<Amphipod>,
}

#[derive(Copy, Clone, Debug, Eq)]
struct Room<const N: usize> {
    kind: Amphipod,
    positions: [Field; N],
}

impl<const N: usize> PartialEq for Room<N> {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
    }
}

impl<const N: usize> Hash for Room<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
    }
}

impl<const N: usize> Room<N> {
    fn new(kind: Amphipod) -> Self {
        Self {
            kind,
            positions: [Field::default(); N],
        }
    }

    fn is_valid(&self, default: bool) -> bool {
        self.positions
            .iter()
            .all(|pos| pos.amphipod.as_ref().map_or(default, |a| a == &self.kind))
    }

    fn is_sorted(&self) -> bool {
        self.is_valid(false)
    }

    fn is_clear(&self) -> bool {
        self.is_valid(true)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'A' => Self::A,
            b'B' => Self::B,
            b'C' => Self::C,
            b'D' => Self::D,
            _ => unreachable!(),
        }
    }

    fn energy(self) -> u32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

// Display impls

impl<const N: usize> fmt::Display for Burrow<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#############\n#")?;

        for field in &self.hallway {
            write!(f, "{}", field)?;
        }

        writeln!(
            f,
            "# Energy: {}\n###{}#{}#{}#{}###",
            self.energy(),
            self.rooms[0].positions[0],
            self.rooms[1].positions[0],
            self.rooms[2].positions[0],
            self.rooms[3].positions[0],
        )?;

        for i in 1..N {
            writeln!(
                f,
                "  #{}#{}#{}#{}#",
                self.rooms[0].positions[i],
                self.rooms[1].positions[i],
                self.rooms[2].positions[i],
                self.rooms[3].positions[i],
            )?;
        }

        write!(f, "  #########")
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.amphipod {
            Some(ref amphipod) => fmt::Display::fmt(amphipod, f),
            None => f.write_str("."),
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
