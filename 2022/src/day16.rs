use std::{
    cmp::Ordering,
    collections::{
        btree_map::Entry as BTreeEntry, hash_map::Entry as HashEntry, BTreeMap, BTreeSet,
        BinaryHeap, HashMap,
    },
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let valves = Valves::from_str(input)?;

    let p1 = part1_bitset(&valves);

    Ok(Solution::new().part1(p1))

    // run_floyd_warshall(input)
}

fn part1_bitset(valves: &Valves) -> i16 {
    struct State {
        valve: u8,
        pressure: i16,
        time: i16,
        opened: Opened,
    }

    let mut opened = Opened::new(valves.len());
    opened.open(0);

    let start = State {
        valve: 0,
        pressure: 0,
        time: 30,
        opened,
    };

    let mut stack = vec![start];
    let mut best = 0;

    while let Some(state) = stack.pop() {
        if state.time < 0 {
            continue;
        }

        if state.pressure > best {
            best = state.pressure;
        }

        if state.time == 0 {
            continue;
        }

        for (shortest_path, next_valve) in valves.shortest_paths(state.valve).iter().zip(0..) {
            if state.opened.is_open(next_valve) {
                continue;
            }

            let next_time = state.time - *shortest_path as i16 - 1;
            let next_pressure = state.pressure + next_time * valves.flow_rates[next_valve as usize];
            let mut next_opened = state.opened;
            next_opened.open(next_valve);

            let state = State {
                valve: next_valve,
                pressure: next_pressure,
                time: next_time,
                opened: next_opened,
            };

            stack.push(state);
        }
    }

    best
}

#[derive(Copy, Clone)]
struct Opened {
    bitset: u32,
}

impl Opened {
    fn new(count: usize) -> Self {
        Self {
            bitset: !((1 << count) - 1),
        }
    }

    fn open(&mut self, idx: u8) {
        self.bitset |= 1 << idx;
    }

    fn is_open(self, idx: u8) -> bool {
        (self.bitset & (1 << idx)) > 0
    }
}

struct Valves {
    flow_rates: Box<[i16]>,
    shortest_paths: Box<[u8]>,
}

impl Valves {
    fn len(&self) -> usize {
        self.flow_rates.len()
    }

    fn shortest_paths(&self, valve: u8) -> &[u8] {
        let len = self.len();
        let idx = (valve as usize) * len;

        &self.shortest_paths[idx..idx + len]
    }
}

impl FromStr for Valves {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut indices = HashMap::new();
        indices.insert("AA", 0);
        let mut next_idx = 1;

        let mut get_idx = |name| match indices.entry(name) {
            HashEntry::Occupied(e) => *e.get(),
            HashEntry::Vacant(e) => {
                let idx = *e.insert(next_idx);
                next_idx += 1;

                idx
            }
        };

        let mut valves = input
            .lines()
            .map(|line| {
                let rest = line.strip_prefix("Valve ").wrap_err("invalid line")?;
                let (name, rest) = rest.split_once(' ').wrap_err("invalid name")?;
                let idx = get_idx(name);

                let rest = rest
                    .strip_prefix("has flow rate=")
                    .wrap_err("invalid line `flow rate`")?;

                let (n, rest) = rest.split_once(';').wrap_err("invalid line `;`")?;
                let flow_rate: i16 = n.parse().wrap_err("invalid flow rate")?;

                let tunnels = rest
                    .strip_prefix(" tunnel leads to valve ")
                    .or_else(|| rest.strip_prefix(" tunnels lead to valves "))
                    .wrap_err("invalid line")?;

                Ok((idx, flow_rate, tunnels))
            })
            .collect::<Result<Vec<_>>>()
            .wrap_err("invalid input")?;

        valves.sort_unstable_by_key(|(idx, ..)| *idx);

        let mut width = valves.len();
        let mut adjacency = vec![u8::MAX; width * width];

        let mut flow_rates: Vec<_> = valves
            .into_iter()
            .map(|(i, flow_rate, tunnels)| {
                let idx = i * width + i;
                adjacency[idx] = 0;

                for j in tunnels.split(", ").map(&mut get_idx) {
                    let idx = i * width + j;
                    adjacency[idx] = 1;
                }

                flow_rate
            })
            .collect();

        for k in 0..width {
            for i in 0..width {
                for j in 0..width {
                    let i_j_idx = i * width + j;
                    let i_k_idx = i * width + k;
                    let k_j_idx = k * width + j;

                    let i_k = adjacency[i_k_idx];
                    let k_j = adjacency[k_j_idx];
                    let i_j = &mut adjacency[i_j_idx];

                    if let Some(i_k_j) = i_k.checked_add(k_j).filter(|i_k_j| *i_j > *i_k_j) {
                        *i_j = i_k_j;
                    }
                }
            }
        }

        let mut removed = Vec::new();

        for (name, idx) in indices {
            let adjusted = idx - removed.iter().filter(|&n| *n < idx).count();

            if flow_rates[adjusted] > 0 || name == "AA" {
                continue;
            }

            let start = adjusted * width;
            let end = start + width;

            adjacency.drain(start..end);

            for j in (0..width - 1).rev() {
                let i = j * width + adjusted;
                adjacency.remove(i);
            }

            width -= 1;
            removed.push(idx);
            flow_rates.remove(adjusted);
        }

        assert!(
            flow_rates.len() <= 32,
            "got {} valves after reducing but we can't have more than 32",
            flow_rates.len(),
        );

        Ok(Self {
            flow_rates: flow_rates.into(),
            shortest_paths: adjacency.into(),
        })
    }
}

type Valves_ = HashMap<ValveName, Valve>;

fn parse_valves(input: &str) -> Result<Valves_> {
    input
        .lines()
        .map(|line| {
            let rest = line.strip_prefix("Valve ").wrap_err("invalid line")?;
            let (name, rest) = rest.split_once(' ').wrap_err("invalid name")?;

            let rest = rest
                .strip_prefix("has flow rate=")
                .wrap_err("invalid line `flow rate`")?;

            let (n, rest) = rest.split_once(';').wrap_err("invalid line `;`")?;
            let flow_rate = n.parse().wrap_err("invalid flow rate")?;

            let rest = rest
                .strip_prefix(" tunnel leads to valve ")
                .or_else(|| rest.strip_prefix(" tunnels lead to valves "))
                .wrap_err("invalid line")?;

            let name = ValveName::from_str(name).wrap_err("invalid valve name")?;

            let tunnels = rest
                .split(", ")
                .map(ValveName::from_str)
                .collect::<Result<_, _>>()
                .wrap_err("invalid tunnel name")?;

            let valve = Valve {
                name,
                flow_rate,
                tunnels,
            };

            Ok((name, valve))
        })
        .collect()
}

struct Graph {
    matrix: Box<[u8]>,
    vertex_count: usize,
    indices: BTreeMap<ValveName, usize>,
}

impl Graph {
    fn from_valves(valves: &Valves_) -> Self {
        // Adjacency list
        let mut reduced: HashMap<ValveName, HashMap<ValveName, u8>> = valves
            .values()
            .map(|valve| {
                let name = valve.name;
                let mut costs: HashMap<_, _> =
                    valve.tunnels.iter().map(|tunnel| (*tunnel, 1)).collect();
                costs.entry(name).and_modify(|cost| *cost = 0).or_insert(0);

                (name, costs)
            })
            .collect();

        let mut changed = true;

        // Short-circuit all 0-flowchart valves by adding
        // their tunnels to valves who reach them
        while changed {
            changed = false;

            for valve in valves.values() {
                if valve.flow_rate > 0 || valve.name.as_str() == "AA" {
                    continue;
                }

                for costs in reduced.values_mut() {
                    let Some(new_cost) = costs.get(&valve.name) else { continue };
                    let new_cost = *new_cost + 1;

                    for tunnel in valve.tunnels.iter() {
                        if let Some(cost) = costs.get_mut(tunnel) {
                            if *cost > new_cost {
                                *cost = new_cost;
                                changed = true;
                            }
                        } else {
                            costs.insert(*tunnel, new_cost);
                            changed = true;
                        }
                    }
                }
            }
        }

        // Remove 0-flowrate valves
        for valve in valves.values() {
            if valve.flow_rate > 0 || valve.name.as_str() == "AA" {
                continue;
            }

            reduced.remove(&valve.name);

            for costs in reduced.values_mut() {
                costs.remove(&valve.name);
            }
        }

        // Convert adjacency list to adjacency matrix
        let len = reduced.len();
        let mut matrix = vec![255; len * len].into_boxed_slice();

        let mut next_idx = 0;

        // sort stuff, not necessary but makes things nicer
        let mut set = BTreeSet::new();

        for valve in valves.values() {
            if valve.flow_rate > 0 || valve.name.as_str() == "AA" {
                set.insert(valve.name);
            }
        }

        let mut indices: BTreeMap<_, _> = set
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();

        let mut get_idx = |name| match indices.entry(name) {
            BTreeEntry::Occupied(e) => *e.get(),
            BTreeEntry::Vacant(e) => {
                let i = *e.insert(next_idx);
                next_idx += 1;

                i
            }
        };

        for (name, costs) in reduced {
            let i = get_idx(name);

            for (tunnel, cost) in costs {
                let j = get_idx(tunnel);
                let idx = i * len + j;
                matrix[idx] = cost;
            }
        }

        Self {
            matrix,
            vertex_count: len,
            indices,
        }
    }

    // Apply Floyd-Warshall on the adjacency matrix
    // so that it contains the shortest path from
    // each vertex to any other vertex
    fn floyd_warshall(&mut self) {
        for k in 0..self.vertex_count {
            for i in 0..self.vertex_count {
                for j in 0..self.vertex_count {
                    let i_j_idx = i * self.vertex_count + j;
                    let i_k_idx = i * self.vertex_count + k;
                    let k_j_idx = k * self.vertex_count + j;

                    let i_j = self.matrix[i_j_idx];
                    let i_k = self.matrix[i_k_idx];
                    let k_j = self.matrix[k_j_idx];

                    if let Some(i_k_j) = i_k.checked_add(k_j).filter(|i_k_j| *i_k_j < i_j) {
                        self.matrix[i_j_idx] = i_k_j;
                    }
                }
            }
        }
    }

    fn shortest_path(&self, from: ValveName, to: ValveName) -> u8 {
        let i = self.indices[&from];
        let j = self.indices[&to];
        let len = self.indices.len();

        self.matrix[i * len + j]
    }
}

pub fn run_floyd_warshall(input: &str) -> Result<Solution> {
    let mut valves = parse_valves(input)?;
    let mut graph = Graph::from_valves(&valves);
    graph.floyd_warshall();

    valves.retain(|_, valve| valve.flow_rate > 0 || valve.name.as_str() == "AA");

    let p1 = part1_floyd_warshall(&valves, &graph);
    let p2 = part2_floyd_warshall(&valves, &graph)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1_floyd_warshall(valves: &Valves_, graph: &Graph) -> i16 {
    struct State {
        valve: ValveName,
        time: i16,
        pressure: i16,
        remaining: Vec<ValveName>,
    }

    let mut remaining: Vec<_> = valves.values().map(|valve| valve.name).collect();
    remaining.retain(|name| name.as_str() != "AA");

    let start = State {
        valve: b"AA".into(),
        time: 30,
        pressure: 0,
        remaining,
    };

    let mut stack = vec![start];
    let mut best = 0;

    while let Some(state) = stack.pop() {
        let State {
            valve,
            time,
            pressure,
            remaining,
        } = state;

        if time < 0 {
            continue;
        }

        if pressure > best {
            best = pressure;
        }

        for i in 0..remaining.len() {
            let mut remaining = remaining.clone();
            let next = remaining.swap_remove(i);

            let shortest_path = graph.shortest_path(valve, next);
            let Some(Valve { flow_rate, .. }) = valves.get(&next) else { continue };

            let time = time - shortest_path as i16 - 1;
            let pressure = pressure + time * *flow_rate;

            let state = State {
                valve: next,
                time,
                pressure,
                remaining,
            };

            stack.push(state);
        }
    }

    best
}

fn part2_floyd_warshall(valves: &Valves_, graph: &Graph) -> Result<i16> {
    #[derive(Copy, Clone, Eq)]
    struct Runner {
        target: ValveName,
        flow_rate: i16,
        steps_left: u8,
    }

    impl PartialEq for Runner {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.target == other.target && self.steps_left == other.steps_left
        }
    }

    impl Runner {
        fn max() -> Self {
            Self {
                target: b"AA".into(),
                flow_rate: 0,
                steps_left: u8::MAX,
            }
        }

        fn is_done(&self) -> bool {
            self.steps_left > 100
        }
    }

    #[derive(PartialEq, Eq)]
    struct State {
        you: Runner,
        elephant: Runner,
        time: i16,
        pressure: i16,
        remaining: Vec<ValveName>,
    }

    impl State {
        fn value(&self) -> i16 {
            let gain_you = (self.time - self.you.steps_left as i16) * self.you.flow_rate;
            let gain_elephant =
                (self.time - self.elephant.steps_left as i16) * self.elephant.flow_rate;

            gain_you.max(0) + gain_elephant.max(0) + self.pressure
        }
    }

    impl Ord for State {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            self.value().cmp(&other.value())
        }
    }

    impl PartialOrd for State {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut remaining: Vec<_> = valves.values().map(|valve| valve.name).collect();
    remaining.retain(|name| name.as_str() != "AA");

    let you = Runner {
        target: b"AA".into(),
        flow_rate: 0,
        steps_left: 0,
    };

    let elephant = Runner {
        target: b"AA".into(),
        flow_rate: 0,
        steps_left: 0,
    };

    let start = State {
        you,
        elephant,
        time: 26,
        pressure: 0,
        remaining,
    };

    let mut heap = BinaryHeap::new();
    heap.push(start);

    let mut best = 0;
    let mut iters = 0_u64;

    while let Some(state) = heap.pop() {
        iters += 1;

        if iters % 10_000_000 == 0 {
            println!("ITERS={iters} | LEN={} | BEST={best}", heap.len());
        }

        let State {
            you,
            elephant,
            time,
            pressure,
            remaining,
        } = state;

        if time < 0 {
            continue;
        }

        if pressure > best {
            best = pressure;
            println!("New Best: {best}");
        }

        if remaining.is_empty() && you.is_done() && elephant.is_done() {
            continue;
        }

        match (you.steps_left, elephant.steps_left) {
            (0, 0) => {
                let mut pressure = pressure;

                let Some(you_target) = valves.get(&you.target) else { continue };
                let Some(elephant_target) = valves.get(&elephant.target) else { continue };

                pressure += time * (you_target.flow_rate + elephant_target.flow_rate);
                let time = time - 1;

                match remaining.as_slice() {
                    [] => {}
                    [next] => {
                        let next = *next;
                        let shortest_path = graph.shortest_path(you.target, next);

                        let you = Runner {
                            target: next,
                            flow_rate: valves.get(&next).wrap_err("missing next")?.flow_rate,
                            steps_left: 0,
                        };

                        let state = State {
                            you,
                            elephant: Runner::max(),
                            time: time - shortest_path as i16,
                            pressure,
                            remaining: Vec::new(),
                        };

                        heap.push(state);
                    }
                    [_, _, ..] => {
                        for i in 0..remaining.len() - 1 {
                            for j in i + 1..remaining.len() {
                                let mut remaining = remaining.clone();
                                let you_next = remaining.swap_remove(j);
                                let elephant_next = remaining.swap_remove(i);

                                let shortest_path = graph.shortest_path(you.target, you_next);

                                let mut you = Runner {
                                    target: you_next,
                                    flow_rate: valves
                                        .get(&you_next)
                                        .wrap_err("missing you_next")?
                                        .flow_rate,
                                    steps_left: shortest_path,
                                };

                                let shortest_path =
                                    graph.shortest_path(elephant.target, elephant_next);

                                let mut elephant = Runner {
                                    target: elephant_next,
                                    flow_rate: valves
                                        .get(&elephant_next)
                                        .wrap_err("missing elephant_next")?
                                        .flow_rate,
                                    steps_left: shortest_path,
                                };

                                let min = you.steps_left.min(elephant.steps_left);
                                you.steps_left -= min;
                                elephant.steps_left -= min;
                                let time = time - min as i16;

                                let state1 = State {
                                    you,
                                    elephant,
                                    time,
                                    pressure,
                                    remaining: remaining.clone(),
                                };

                                let state2 = State {
                                    you: elephant,
                                    elephant: you,
                                    time,
                                    pressure,
                                    remaining,
                                };

                                heap.extend([state1, state2]);
                            }
                        }
                    }
                }
            }
            (_, 0) => {
                let you = Runner {
                    target: you.target,
                    flow_rate: you.flow_rate,
                    steps_left: you.steps_left - 1,
                };

                let Some(Valve { flow_rate, .. }) = valves.get(&elephant.target) else { continue };
                let pressure = pressure + time * *flow_rate;
                let time = time - 1;

                if remaining.is_empty() {
                    let state = State {
                        you,
                        elephant: Runner::max(),
                        time,
                        pressure,
                        remaining,
                    };

                    heap.push(state);
                } else {
                    for i in 0..remaining.len() {
                        let mut remaining = remaining.clone();
                        let next = remaining.swap_remove(i);

                        let shortest_path = graph.shortest_path(elephant.target, next);

                        let elephant = Runner {
                            target: next,
                            flow_rate: valves.get(&next).wrap_err("missing next")?.flow_rate,
                            steps_left: shortest_path,
                        };

                        let state = State {
                            you,
                            elephant,
                            time,
                            pressure,
                            remaining,
                        };

                        heap.push(state);
                    }
                }
            }
            (0, _) => {
                let elephant = Runner {
                    target: elephant.target,
                    flow_rate: elephant.flow_rate,
                    steps_left: elephant.steps_left - 1,
                };

                let Some(Valve { flow_rate, .. }) = valves.get(&you.target) else { continue };
                let pressure = pressure + time * *flow_rate;
                let time = time - 1;

                if remaining.is_empty() {
                    let state = State {
                        you: Runner::max(),
                        elephant,
                        time,
                        pressure,
                        remaining,
                    };

                    heap.push(state);
                } else {
                    for i in 0..remaining.len() {
                        let mut remaining = remaining.clone();
                        let next = remaining.swap_remove(i);

                        let shortest_path = graph.shortest_path(you.target, next);

                        let you = Runner {
                            target: next,
                            flow_rate: valves.get(&next).wrap_err("missing next")?.flow_rate,
                            steps_left: shortest_path,
                        };

                        let state = State {
                            you,
                            elephant,
                            time,
                            pressure,
                            remaining,
                        };

                        heap.push(state);
                    }
                }
            }
            (_, _) => {
                let min = you.steps_left.min(elephant.steps_left);

                let you = Runner {
                    target: you.target,
                    flow_rate: you.flow_rate,
                    steps_left: you.steps_left - min,
                };

                let elephant = Runner {
                    target: elephant.target,
                    flow_rate: elephant.flow_rate,
                    steps_left: elephant.steps_left - min,
                };

                let state = State {
                    you,
                    elephant,
                    time: time - min as i16,
                    pressure,
                    remaining,
                };

                heap.push(state);
            }
        }
    }

    println!("ITERS={iters}");

    Ok(best)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveName([u8; 2]);

impl ValveName {
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<&[u8; 2]> for ValveName {
    #[inline]
    fn from(name: &[u8; 2]) -> Self {
        Self(*name)
    }
}

impl FromStr for ValveName {
    type Err = std::array::TryFromSliceError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().try_into()
    }
}

impl TryFrom<&[u8]> for ValveName {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

#[derive(Clone, Eq)]
struct Valve {
    name: ValveName,
    flow_rate: i16,
    tunnels: Vec<ValveName>,
}

impl PartialEq for Valve {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Valve {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn fmt_row(row: &[u8], f: &mut Formatter<'_>) -> FmtResult {
            let mut iter = row.iter();

            if let Some(n) = iter.next() {
                match n {
                    255 => f.write_str(" x ")?,
                    n => write!(f, "{n:^3}")?,
                }

                for n in iter {
                    match n {
                        255 => f.write_str(" x ")?,
                        n => write!(f, "{n:^3}")?,
                    }
                }
            }

            Ok(())
        }

        let mut rows = self.matrix.chunks_exact(self.vertex_count);

        if let Some(row) = rows.next() {
            fmt_row(row, f)?;

            for row in rows {
                f.write_str("\n")?;
                fmt_row(row, f)?;
            }
        }

        write!(f, "\n{:?}", self.indices)
    }
}
