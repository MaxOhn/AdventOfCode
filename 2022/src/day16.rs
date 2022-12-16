use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::prelude::*;

// TODO: cargo add regex & once_cell

pub fn run(input: &str) -> Result<Solution> {
    // run_naive(input)
    run_floyd_warshall(input)
}

type Valves = HashMap<ValveName, Valve>;

fn parse_valves(input: &str) -> Result<Valves> {
    let mut valves = HashMap::new();

    for line in input.lines() {
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

        valves.insert(name, valve);
    }

    Ok(valves)
}

struct Graph {
    matrix: Box<[Option<u8>]>,
    vertex_count: usize,
    indices: HashMap<ValveName, usize>,
}

impl Graph {
    fn from_valves(valves: &Valves) -> Self {
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
        let mut matrix = vec![None; len * len].into_boxed_slice();

        let mut next_idx = 0;
        let mut indices = HashMap::new();

        let mut get_idx = |name| match indices.entry(name) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
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
                matrix[idx] = Some(cost);
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

                    let new_cost = match (i_k, k_j) {
                        (Some(i_k), Some(k_j)) => i_k + k_j,
                        _ => continue,
                    };

                    match i_j {
                        Some(i_j) if i_j < new_cost => {}
                        Some(_) | None => self.matrix[i_j_idx] = Some(new_cost),
                    }
                }
            }
        }
    }

    fn shortest_path(&self, from: ValveName, to: ValveName) -> Option<u8> {
        let i = self.indices.get(&from)?;
        let j = self.indices.get(&to)?;
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
    let p2 = part2_floyd_warshall(&valves, &graph);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1_floyd_warshall(valves: &Valves, graph: &Graph) -> i16 {
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

            let Some(shortest_path) = graph.shortest_path(valve, next) else { continue };
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

fn part2_floyd_warshall(valves: &Valves, graph: &Graph) -> i16 {
    #[derive(Copy, Clone)]
    struct Runner {
        target: ValveName,
        steps_left: u8,
    }

    impl Runner {
        fn max() -> Self {
            Self {
                target: b"AA".into(),
                steps_left: u8::MAX,
            }
        }

        fn is_done(&self) -> bool {
            self.steps_left > 100
        }
    }

    struct State {
        you: Runner,
        elephant: Runner,
        time: i16,
        pressure: i16,
        remaining: Vec<ValveName>,
    }

    let mut remaining: Vec<_> = valves.values().map(|valve| valve.name).collect();
    remaining.retain(|name| name.as_str() != "AA");

    let you = Runner {
        target: b"AA".into(),
        steps_left: 0,
    };

    let elephant = Runner {
        target: b"AA".into(),
        steps_left: 0,
    };

    let start = State {
        you,
        elephant,
        time: 26,
        pressure: 0,
        remaining,
    };

    let mut stack = vec![start];
    let mut best = 0;

    let mut iters = 0;

    while let Some(state) = stack.pop() {
        iters += 1;

        if iters % 10_000_000 == 0 {
            println!("ITERS={iters} | LEN={}", stack.len());
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

                #[allow(clippy::comparison_chain)]
                if remaining.len() == 1 {
                    let next = *remaining.last().unwrap();
                    let Some(shortest_path) = graph.shortest_path(you.target, next) else { continue };

                    let you = Runner {
                        target: next,
                        steps_left: 0,
                    };

                    let state = State {
                        you,
                        elephant: Runner::max(),
                        time: time - shortest_path as i16,
                        pressure,
                        remaining: Vec::new(),
                    };

                    stack.push(state);
                } else if remaining.len() > 1 {
                    for i in 0..remaining.len() - 1 {
                        for j in i + 1..remaining.len() {
                            let mut remaining = remaining.clone();
                            let you_next = remaining.swap_remove(j);
                            let elephant_next = remaining.swap_remove(i);

                            let Some(shortest_path) = graph.shortest_path(you.target, you_next) else { continue };

                            let mut you = Runner {
                                target: you_next,
                                steps_left: shortest_path,
                            };

                            let Some(shortest_path) = graph.shortest_path(elephant.target, elephant_next) else { continue };

                            let mut elephant = Runner {
                                target: elephant_next,
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

                            stack.extend([state1, state2]);
                        }
                    }
                }
            }
            (_, 0) => {
                let you = Runner {
                    target: you.target,
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

                    stack.push(state);
                } else {
                    for i in 0..remaining.len() {
                        let mut remaining = remaining.clone();
                        let next = remaining.swap_remove(i);

                        let Some(shortest_path) = graph.shortest_path(elephant.target, next) else { continue };

                        let elephant = Runner {
                            target: next,
                            steps_left: shortest_path,
                        };

                        let state = State {
                            you,
                            elephant,
                            time,
                            pressure,
                            remaining,
                        };

                        stack.push(state);
                    }
                }
            }
            (0, _) => {
                let elephant = Runner {
                    target: elephant.target,
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

                    stack.push(state);
                } else {
                    for i in 0..remaining.len() {
                        let mut remaining = remaining.clone();
                        let next = remaining.swap_remove(i);

                        let Some(shortest_path) = graph.shortest_path(you.target, next) else { continue };

                        let you = Runner {
                            target: next,
                            steps_left: shortest_path,
                        };

                        let state = State {
                            you,
                            elephant,
                            time,
                            pressure,
                            remaining,
                        };

                        stack.push(state);
                    }
                }
            }
            (_, _) => {
                let min = you.steps_left.min(elephant.steps_left);

                let you = Runner {
                    target: you.target,
                    steps_left: you.steps_left - min,
                };

                let elephant = Runner {
                    target: elephant.target,
                    steps_left: elephant.steps_left - min,
                };

                let state = State {
                    you,
                    elephant,
                    time: time - min as i16,
                    pressure,
                    remaining,
                };

                stack.push(state);
            }
        }
    }

    println!("ITERS={iters}");

    best
}

pub fn run_naive(input: &str) -> Result<Solution> {
    let valves = parse_valves(input)?;

    let p1 = part1(&valves)?;
    let p2 = part2(&valves)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(valves: &Valves) -> Result<i16> {
    #[derive(Clone)]
    struct State<'v> {
        valve: &'v Valve,
        time: i16,
        pressure: i16,
        opened: Vec<ValveName>,
        path: Vec<(ValveName, i16, i16, Action)>,
    }

    let start = State {
        valve: valves.get(&b"AA".into()).wrap_err("missing valve `AA`")?,
        time: 30,
        pressure: 0,
        opened: Vec::new(),
        path: Vec::new(),
    };

    let mut stack = Vec::new();
    stack.push(start.clone());

    let mut seen = HashSet::new();

    let mut best = start;

    while let Some(state) = stack.pop() {
        if state.pressure > best.pressure {
            best = state.clone();
        }

        let State {
            valve,
            time,
            pressure,
            opened,
            path,
        } = state;

        for tunnel in valve.tunnels.iter() {
            let next_valve = valves.get(tunnel).wrap_err("missing tunnel")?;

            let next_time = time - 1;

            if next_time >= 0 && seen.insert((next_valve.name, next_time, pressure)) {
                let mut path = path.clone();
                path.push((next_valve.name, next_time, pressure, Action::DontOpen));

                let dont_open = State {
                    valve: next_valve,
                    time: next_time,
                    pressure,
                    opened: opened.clone(),
                    path,
                };

                stack.push(dont_open);
            }

            let next_time = time - 2;

            if next_time >= 0 && next_valve.flow_rate > 0 && !opened.contains(&next_valve.name) {
                let next_pressure = pressure + next_time * next_valve.flow_rate;

                if seen.insert((next_valve.name, next_time, next_pressure)) {
                    let mut path = path.clone();
                    path.push((next_valve.name, next_time, next_pressure, Action::Open));

                    let mut opened = opened.clone();
                    opened.push(next_valve.name);

                    let open = State {
                        valve: next_valve,
                        time: next_time,
                        pressure: next_pressure,
                        opened,
                        path,
                    };

                    stack.push(open);
                }
            }
        }
    }

    for (name, time, pressure, action) in best.path {
        println!("[{time}] {action:?} {name}: {pressure}");
    }

    Ok(best.pressure)
}

fn part2(valves: &Valves) -> Result<i16> {
    #[derive(Clone)]
    struct State<'v> {
        valve_you: &'v Valve,
        valve_elephant: &'v Valve,
        status_you: Status,
        status_elephant: Status,
        time: i16,
        pressure: i16,
        opened: Vec<ValveName>,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    enum Status {
        Opening,
        Ready,
    }

    let aa = valves.get(&b"AA".into()).wrap_err("missing valve `AA`")?;

    let start = State {
        valve_you: aa,
        valve_elephant: aa,
        status_you: Status::Ready,
        status_elephant: Status::Ready,
        time: 26,
        pressure: 0,
        opened: Vec::new(),
    };

    let mut stack = Vec::new();
    stack.push(start.clone());

    let mut seen = HashSet::new();
    let mut best = start;

    let mut next_you = Vec::new();
    let mut next_elephant = Vec::new();

    while let Some(state) = stack.pop() {
        if state.pressure > best.pressure {
            best = state.clone();

            println!(
                "New Best: {:?} -> {} [seen: {}]",
                best.opened,
                best.pressure,
                seen.len(),
            );
        }

        let State {
            valve_you,
            valve_elephant,
            status_you,
            status_elephant,
            time,
            pressure,
            opened,
        } = state;

        enum Next<'v> {
            Wait,
            DontOpen {
                valve: &'v Valve,
            },
            Open {
                valve: &'v Valve,
                added_pressure: i16,
            },
        }

        if status_you == Status::Ready {
            for tunnel in valve_you.tunnels.iter() {
                let next_valve = valves.get(tunnel).wrap_err("missing tunnel")?;

                let next_time = time - 1;

                if next_time >= 0 {
                    next_you.push(Next::DontOpen { valve: next_valve });
                }

                let next_time = time - 2;

                if next_time >= 0 && next_valve.flow_rate > 0 && !opened.contains(&next_valve.name)
                {
                    let added_pressure = next_time * next_valve.flow_rate;

                    let next = Next::Open {
                        valve: next_valve,
                        added_pressure,
                    };

                    next_you.push(next);
                }
            }
        } else {
            next_you.push(Next::Wait);
        }

        if status_elephant == Status::Ready {
            for tunnel in valve_elephant.tunnels.iter() {
                let next_valve = valves.get(tunnel).wrap_err("missing tunnel")?;

                let next_time = time - 1;

                if next_time >= 0 {
                    next_elephant.push(Next::DontOpen { valve: next_valve });
                }

                let next_time = time - 2;

                if next_time >= 0 && next_valve.flow_rate > 0 && !opened.contains(&next_valve.name)
                {
                    let added_pressure = next_time * next_valve.flow_rate;

                    let next = Next::Open {
                        valve: next_valve,
                        added_pressure,
                    };

                    next_elephant.push(next);
                }
            }
        } else {
            next_elephant.push(Next::Wait);
        }

        for next_you in next_you.drain(..) {
            for next_eleph in next_elephant.iter() {
                let state = match (&next_you, next_eleph) {
                    (
                        Next::Open {
                            valve: valve_you,
                            added_pressure: pressure_you,
                        },
                        Next::Open {
                            valve: valve_elephant,
                            added_pressure: pressure_eleph,
                        },
                    ) => {
                        let mut opened = opened.clone();
                        opened.push(valve_you.name);
                        opened.push(valve_elephant.name);

                        State {
                            valve_you,
                            valve_elephant,
                            status_you: Status::Ready,
                            status_elephant: Status::Ready,
                            time: time - 2,
                            pressure: pressure
                                + pressure_you
                                + (valve_you.name != valve_elephant.name) as i16 * pressure_eleph,
                            opened,
                        }
                    }
                    (
                        Next::Open {
                            valve: valve_you,
                            added_pressure,
                        },
                        Next::DontOpen {
                            valve: valve_elephant,
                        },
                    ) => {
                        let mut opened = opened.clone();
                        opened.push(valve_you.name);

                        State {
                            valve_you,
                            valve_elephant,
                            status_you: Status::Opening,
                            status_elephant: Status::Ready,
                            time: time - 1,
                            pressure: pressure + added_pressure,
                            opened,
                        }
                    }
                    (
                        Next::Open {
                            valve,
                            added_pressure,
                        },
                        Next::Wait,
                    ) => {
                        let mut opened = opened.clone();
                        opened.push(valve.name);

                        State {
                            valve_you: valve,
                            valve_elephant,
                            status_you: Status::Opening,
                            status_elephant: Status::Ready,
                            time: time - 1,
                            pressure: pressure + added_pressure,
                            opened,
                        }
                    }
                    (
                        Next::DontOpen { valve: valve_you },
                        Next::Open {
                            valve: valve_elephant,
                            added_pressure,
                        },
                    ) => {
                        let mut opened = opened.clone();
                        opened.push(valve_elephant.name);

                        State {
                            valve_you,
                            valve_elephant,
                            status_you: Status::Ready,
                            status_elephant: Status::Opening,
                            time: time - 1,
                            pressure: pressure + added_pressure,
                            opened,
                        }
                    }
                    (
                        Next::DontOpen { valve: valve_you },
                        Next::DontOpen {
                            valve: valve_elephant,
                        },
                    ) => State {
                        valve_you,
                        valve_elephant,
                        status_you: Status::Ready,
                        status_elephant: Status::Ready,
                        time: time - 1,
                        pressure,
                        opened: opened.clone(),
                    },
                    (Next::DontOpen { valve }, Next::Wait) => State {
                        valve_you: valve,
                        valve_elephant,
                        status_you: Status::Ready,
                        status_elephant: Status::Ready,
                        time: time - 1,
                        pressure,
                        opened: opened.clone(),
                    },
                    (
                        Next::Wait,
                        Next::Open {
                            valve,
                            added_pressure,
                        },
                    ) => {
                        let mut opened = opened.clone();
                        opened.push(valve.name);

                        State {
                            valve_you,
                            valve_elephant: valve,
                            status_you: Status::Ready,
                            status_elephant: Status::Opening,
                            time: time - 1,
                            pressure: pressure + added_pressure,
                            opened,
                        }
                    }
                    (Next::Wait, Next::DontOpen { valve }) => State {
                        valve_you,
                        valve_elephant: valve,
                        status_you: Status::Ready,
                        status_elephant: Status::Ready,
                        time: time - 1,
                        pressure,
                        opened: opened.clone(),
                    },
                    (Next::Wait, Next::Wait) => unreachable!(),
                };

                let key = (
                    state.valve_you,
                    state.valve_elephant,
                    state.status_you,
                    state.status_elephant,
                    state.time,
                    state.pressure,
                );

                if state.time >= 0 && seen.insert(key) {
                    stack.push(state);
                }
            }
        }

        next_elephant.clear();
    }

    Ok(best.pressure)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    DontOpen,
    Open,
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
        fn fmt_row(row: &[Option<u8>], f: &mut Formatter<'_>) -> FmtResult {
            let mut iter = row.iter();

            if let Some(n) = iter.next() {
                match n {
                    Some(n) => write!(f, "{n}")?,
                    None => f.write_str("x")?,
                }

                for n in iter {
                    match n {
                        Some(n) => write!(f, "{n}")?,
                        None => f.write_str("x")?,
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
