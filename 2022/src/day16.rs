use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    hash::Hash,
    mem,
    str::FromStr,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let valves = Valves::from_str(input)?;

    let p1 = part1(&valves);
    let p2 = part2(&valves);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(valves: &Valves) -> u16 {
    let start = State {
        valve: 0,
        pressure: 0,
        time: 30,
        opened: Opened::new(),
    };

    let mut heap = BinaryHeap::new();
    heap.push((u16::MAX, start));
    let mut best = 0;

    while let Some((bound, state)) = heap.pop() {
        if bound < best {
            // the best value from the heap is worse than the current best, stop iterating
            break;
        } else if state.pressure > best {
            best = state.pressure;
        }

        for (next_valve, dist) in valves.dists(state.valve).iter().enumerate() {
            if state.opened.is_open(next_valve) {
                continue;
            }

            let Some(next_time) = state.time.checked_sub(*dist + 1) else { continue };
            let next_pressure = state.pressure + next_time as u16 * valves.flow_rates[next_valve];
            let next_opened = state.opened.open(next_valve);

            // calculate upper bound by assuming from now on we get all remaining
            // valves in best-case order with cheap distances between them
            let flow_rates = valves
                .sorted_flow_rate_indices
                .iter()
                .copied()
                .filter(|&idx| !next_opened.is_open(idx))
                .map(|idx| valves.flow_rates[idx]);

            let upper_pressure: u16 = (0..=next_time)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(flow_rates)
                .map(|(time, flow_rate)| time as u16 * flow_rate)
                .sum();

            let next_bound = next_pressure + upper_pressure;

            // the upper bound is already less than the current best, ignore state
            if next_bound < best {
                continue;
            }

            let next_state = State {
                valve: next_valve as u8,
                pressure: next_pressure,
                time: next_time,
                opened: next_opened,
            };

            heap.push((next_bound, next_state));
        }
    }

    best
}

fn part2(valves: &Valves) -> u16 {
    let start = State {
        valve: 0,
        pressure: 0,
        time: 26,
        opened: Opened::new(),
    };

    // no bound checking because it could prune away "bad"
    // states that the elephant may take
    let mut stack = vec![start];
    let mut best = 0;
    let mut cache = HashMap::new();

    while let Some(state) = stack.pop() {
        // cache values so the elephant can make use of the same calculation
        cache
            .entry(state.opened)
            .and_modify(|pressure| *pressure = state.pressure.max(*pressure))
            .or_insert(state.pressure);

        if state.pressure > best {
            best = state.pressure;
        }

        for (next_valve, dist) in valves.dists(state.valve).iter().enumerate() {
            if state.opened.is_open(next_valve) {
                continue;
            }

            let Some(next_time) = state.time.checked_sub(*dist + 1) else { continue };
            let next_pressure = state.pressure + next_time as u16 * valves.flow_rates[next_valve];
            let next_opened = state.opened.open(next_valve);

            let next_state = State {
                valve: next_valve as u8,
                pressure: next_pressure,
                time: next_time,
                opened: next_opened,
            };

            stack.push(next_state);
        }
    }

    let mut pressures: Vec<_> = cache.into_iter().collect();
    pressures.sort_unstable_by_key(|(_, pressure)| Reverse(*pressure));

    // find the best pair with a disjoint set of opened valves
    let best = pressures
        .iter()
        .enumerate()
        .fold(0, |best, (i, (my_opened, my_pressure))| {
            pressures[i + 1..]
                .iter()
                .map(|(eleph_opened, eleph_pressure)| (eleph_opened, my_pressure + eleph_pressure))
                .take_while(|(_, pressure)| *pressure > best)
                .find(|(&eleph_opened, _)| my_opened.is_disjoint(eleph_opened))
                .map(|(_, pressure)| pressure)
                .unwrap_or(best)
        });

    best
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    valve: u8,
    pressure: u16,
    time: u8,
    opened: Opened,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Opened {
    bitset: u32,
}

impl Opened {
    fn new() -> Self {
        // the first bit is always set to denote valve `AA`
        Self { bitset: 0x00000001 }
    }

    fn is_disjoint(self, other: Self) -> bool {
        self.bitset & other.bitset == 1
    }

    fn open(self, idx: usize) -> Self {
        Self {
            bitset: self.bitset | (1 << idx),
        }
    }

    fn is_open(self, idx: usize) -> bool {
        (self.bitset & (1 << idx)) > 0
    }
}

struct Valves {
    flow_rates: Box<[u16]>,
    sorted_flow_rate_indices: Box<[usize]>,
    dists: Box<[u8]>,
}

impl Valves {
    fn len(&self) -> usize {
        self.flow_rates.len()
    }

    fn dists(&self, valve: u8) -> &[u8] {
        let len = self.len();
        let idx = (valve as usize) * len;

        &self.dists[idx..idx + len]
    }
}

impl FromStr for Valves {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut indices = HashMap::new();
        indices.insert("AA", 0);
        let mut next_idx = 1;

        let mut get_idx = |name| match indices.entry(name) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let idx = *e.insert(next_idx);
                next_idx += 1;

                idx
            }
        };

        // validate input and pick out key data
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
                let flow_rate: u16 = n.parse().wrap_err("invalid flow rate")?;

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

        // parse tunnels and initialize adjacency matrix in one go
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

        // floyd-warshall to calculate shortest distances
        for k in 0..width {
            for i in 0..width {
                for j in 0..width {
                    let i_k = adjacency[i * width + k];
                    let k_j = adjacency[k * width + j];
                    let i_j = &mut adjacency[i * width + j];

                    if let Some(i_k_j) = i_k.checked_add(k_j).filter(|i_k_j| *i_j > *i_k_j) {
                        *i_j = i_k_j;
                    }
                }
            }
        }

        // remove all adjacency entries that belong to a valve with 0 flow rate
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
            flow_rates.len() <= mem::size_of::<Opened>() * 8,
            "got {} valves after reducing but we can't have more than {}",
            flow_rates.len(),
            mem::size_of::<Opened>() * 8,
        );

        // pre-calculate a sorted list for flow rates for later use
        let mut sorted_flow_rate_tuples: Vec<_> = flow_rates.iter().copied().enumerate().collect();
        sorted_flow_rate_tuples.sort_unstable_by_key(|(_, flow_rate)| Reverse(*flow_rate));

        removed.clear();
        let mut sorted_flow_rate_indices = removed;
        sorted_flow_rate_indices.extend(sorted_flow_rate_tuples.into_iter().map(|(i, _)| i));

        Ok(Self {
            flow_rates: flow_rates.into(),
            sorted_flow_rate_indices: sorted_flow_rate_indices.into(),
            dists: adjacency.into(),
        })
    }
}
