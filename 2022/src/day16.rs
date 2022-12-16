use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let mut valves = HashMap::new();

    for line in input.lines() {
        let rest = line.strip_prefix("Valve ").wrap_err("invalid line")?;
        let (name, rest) = rest.split_once(' ').wrap_err("invalid name")?;

        let rest = rest
            .strip_prefix("has flow rate=")
            .wrap_err("invalid line `flow rate`")?;

        let (n, rest) = rest.split_once(';').wrap_err("invalid line `;`")?;
        let flow_rate = n.parse().wrap_err("invalid flow rate")?;

        let rest = match rest.strip_prefix(" tunnel leads to valve ") {
            Some(rest) => rest,
            None => rest
                .strip_prefix(" tunnels lead to valves ")
                .wrap_err("cmon")?,
        };

        let name = ValveName::try_from(name.as_bytes()).wrap_err("invalid valve name")?;

        let tunnels = rest
            .split(", ")
            .map(str::as_bytes)
            .map(ValveName::try_from)
            .collect::<Result<_, _>>()
            .wrap_err("invalid tunnel name")?;

        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };

        valves.insert(name, valve);
    }

    let p1 = part1(&valves)?;
    let p2 = part2(&valves)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(valves: &HashMap<ValveName, Valve>) -> Result<i16> {
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

fn part2(valves: &HashMap<ValveName, Valve>) -> Result<i16> {
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

impl Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = unsafe { std::str::from_utf8_unchecked(&self.0) };

        f.write_str(name)
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

#[derive(Eq)]
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
