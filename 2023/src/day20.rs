use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

use aoc_rust::{util::numbers::lcm, Solution};
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Rc<RefCell<Module<'a>>>> {
    let mut modules = HashMap::new();
    let mut dsts = HashMap::new();

    for line in input.lines() {
        let (front, back) = line.split_once(" -> ").unwrap();

        let (name, kind) = if let Some(name) = front.strip_prefix('%') {
            (
                name,
                ModuleKind::FlipFlop {
                    status: Status::Off,
                },
            )
        } else if let Some(name) = front.strip_prefix('&') {
            (
                name,
                ModuleKind::Conjunction {
                    prev_pulses: Vec::new(),
                },
            )
        } else {
            (front, ModuleKind::Broadcaster)
        };

        let module = Rc::new(RefCell::new(Module {
            name,
            kind,
            dst: Vec::new(),
        }));

        modules.insert(name, module);
        dsts.insert(name, back.split(',').map(str::trim).collect::<Vec<_>>());
    }

    for module in modules.values() {
        let mut module = module.borrow_mut();
        let module_name = module.name;

        if let ModuleKind::Conjunction { prev_pulses } = &mut module.kind {
            for (name, dsts) in dsts.iter() {
                if dsts.contains(&module_name) {
                    prev_pulses.push((name, Pulse::Low));
                }
            }
        }
    }

    for (name, dsts) in dsts {
        for dst in dsts {
            let dst = modules.entry(dst).or_insert_with(|| {
                Rc::new(RefCell::new(Module {
                    name: dst,
                    kind: ModuleKind::Other {
                        received_low: false,
                    },
                    dst: Vec::new(),
                }))
            });

            let dst = Rc::clone(dst);
            modules[name].borrow_mut().dst.push(dst);
        }
    }

    modules
}

type PendingPulses<'a> = Vec<(&'a str, &'a str, Pulse)>;

fn part1(input: &str) -> u64 {
    let modules = parse_input(input);

    let mut pending: PendingPulses = Vec::new();
    let mut next_pending: PendingPulses = Vec::new();

    let mut count_low = 0;
    let mut count_high = 0;

    for _ in 1..=1000 {
        modules["broadcaster"]
            .borrow_mut()
            .propagate(Pulse::Low, "button", &mut pending);

        count_low += 1;

        while !pending.is_empty() {
            for (dst, src, pulse) in pending.drain(..) {
                match pulse {
                    Pulse::High => count_high += 1,
                    Pulse::Low => count_low += 1,
                }

                modules[dst]
                    .borrow_mut()
                    .propagate(pulse, &src, &mut next_pending);
            }

            mem::swap(&mut pending, &mut next_pending);
        }
    }

    count_low * count_high
}

fn part2(input: &str) -> u64 {
    let modules = parse_input(input);

    let (prev_rx, _) = modules
        .iter()
        .find(|(_, module)| {
            module
                .borrow()
                .dst
                .iter()
                .any(|dst| dst.borrow().name == "rx")
        })
        .unwrap();

    let mut cycles: HashMap<_, Option<u64>> = modules
        .iter()
        .filter(|(_, module)| {
            module
                .borrow()
                .dst
                .iter()
                .any(|dst| dst.borrow().name == *prev_rx)
        })
        .map(|(name, _)| (*name, None))
        .collect();

    let mut pending: PendingPulses = Vec::new();
    let mut next_pending: PendingPulses = Vec::new();

    for i in 1.. {
        if i % 1_000_000 == 0 {
            println!("i={i}");
        }

        modules["broadcaster"]
            .borrow_mut()
            .propagate(Pulse::Low, "button", &mut pending);

        while !pending.is_empty() {
            for (dst, src, pulse) in pending.drain(..) {
                if let Some((_, cycle)) = cycles.iter_mut().find(|(name, _)| **name == src) {
                    if matches!(pulse, Pulse::High) {
                        cycle.get_or_insert(i);
                    }
                }

                modules[dst]
                    .borrow_mut()
                    .propagate(pulse, &src, &mut next_pending);
            }

            mem::swap(&mut pending, &mut next_pending);
        }

        let cycle = cycles
            .values()
            .copied()
            .try_fold(1, |prod, cycle| Some(lcm(prod, cycle?)));

        if let Some(cycle) = cycle {
            return cycle;
        }
    }

    unreachable!()
}

#[derive(Copy, Clone, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Status {
    On,
    Off,
}

#[derive(Debug)]
enum ModuleKind<'a> {
    FlipFlop { status: Status },
    Conjunction { prev_pulses: Vec<(&'a str, Pulse)> },
    Broadcaster,
    Other { received_low: bool },
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    dst: Vec<Rc<RefCell<Module<'a>>>>,
}

impl<'a> Module<'a> {
    fn propagate(&mut self, pulse: Pulse, from: &'a str, pending: &mut PendingPulses<'a>) {
        match &mut self.kind {
            ModuleKind::FlipFlop { status } => match pulse {
                Pulse::High => {}
                Pulse::Low => {
                    let pulse = match status {
                        Status::On => {
                            *status = Status::Off;

                            Pulse::Low
                        }
                        Status::Off => {
                            *status = Status::On;

                            Pulse::High
                        }
                    };

                    for dst in self.dst.iter_mut() {
                        pending.push((dst.borrow().name, self.name, pulse));
                    }
                }
            },
            ModuleKind::Conjunction { prev_pulses } => {
                if let Some((_, prev_pulse)) =
                    prev_pulses.iter_mut().find(|(name, _)| *name == from)
                {
                    *prev_pulse = pulse;
                }

                let all_high = prev_pulses
                    .iter()
                    .all(|(_, pulse)| matches!(pulse, Pulse::High));

                let pulse = if all_high { Pulse::Low } else { Pulse::High };

                for dst in self.dst.iter_mut() {
                    pending.push((dst.borrow().name, self.name, pulse));
                }
            }
            ModuleKind::Broadcaster => {
                for dst in self.dst.iter_mut() {
                    pending.push((dst.borrow().name, self.name, pulse));
                }
            }
            ModuleKind::Other { received_low } => {
                if matches!(pulse, Pulse::Low) {
                    *received_low = true;
                }
            }
        }
    }
}
