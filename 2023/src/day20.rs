use std::collections::VecDeque;

use aoc_rust::{util::numbers::lcm, Solution};
use eyre::{ContextCompat, Result};
use fxhash::FxHashMap as HashMap;

pub fn run(input: &str) -> Result<Solution> {
    let (modules, dsts) = parse_input(input.trim())?;

    let p1 = part1(modules.clone(), &dsts);
    let p2 = part2(modules, &dsts)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;
type Destinations<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input<'a>(input: &'a str) -> Result<(Modules<'a>, Destinations<'a>)> {
    let mut modules = Modules::default();
    let mut dsts = Destinations::default();

    for line in input.lines() {
        let (front, back) = line.split_once(" -> ").wrap_err("missing arrow")?;

        let (name, module) = if let Some(name) = front.strip_prefix('%') {
            (name, Module::FlipFlop { is_on: false })
        } else if let Some(name) = front.strip_prefix('&') {
            (
                name,
                Module::Conjunction {
                    prev_inputs: Vec::new(),
                },
            )
        } else {
            (front, Module::Broadcaster)
        };

        modules.insert(name, module);
        dsts.insert(name, back.split(',').map(str::trim).collect());
    }

    for (dst, module) in modules.iter_mut() {
        if let Module::Conjunction { prev_inputs } = module {
            for (src, dsts) in dsts.iter() {
                if dsts.contains(&dst) {
                    prev_inputs.push((src, Pulse::Low));
                }
            }
        }
    }

    Ok((modules, dsts))
}

fn part1<'a>(mut modules: Modules<'a>, dsts: &Destinations<'a>) -> u64 {
    let mut pending = VecDeque::new();

    let mut count_low = 0;
    let mut count_high = 0;

    for _ in 1..=1000 {
        let broadcaster = modules.entry("broadcaster").or_default();
        count_low += 1;

        if let Some(next_pulse) = broadcaster.propagate(Pulse::Low, "button") {
            for dst in dsts["broadcaster"].iter() {
                pending.push_back(("broadcaster", *dst, next_pulse));
            }
        }

        while let Some((src, dst, pulse)) = pending.pop_front() {
            match pulse {
                Pulse::High => count_high += 1,
                Pulse::Low => count_low += 1,
            }

            let dst_module = modules.entry(dst).or_default();

            if let Some(next_pulse) = dst_module.propagate(pulse, src) {
                for next_dst in dsts[dst].iter() {
                    pending.push_back((dst, *next_dst, next_pulse));
                }
            }
        }
    }

    count_low * count_high
}

fn part2<'a>(mut modules: Modules<'a>, dsts: &Destinations<'a>) -> Result<u64> {
    let (rx_src, _) = dsts
        .iter()
        .find(|(src, dsts)| {
            dsts.contains(&"rx") && matches!(modules[*src], Module::Conjunction { .. })
        })
        .wrap_err("missing conjunction module that sends to rx")?;

    let mut cycles: HashMap<_, Option<u64>> = dsts
        .iter()
        .filter(|(_, dsts)| dsts.contains(rx_src))
        .map(|(name, _)| (*name, None))
        .collect();

    let mut pending = VecDeque::new();

    for i in 1.. {
        let broadcaster = modules.entry("broadcaster").or_default();

        if let Some(next_pulse) = broadcaster.propagate(Pulse::Low, "button") {
            for dst in dsts["broadcaster"].iter() {
                pending.push_back(("broadcaster", *dst, next_pulse));
            }
        }

        while let Some((src, dst, pulse)) = pending.pop_front() {
            if let Some(cycle) = cycles.get_mut(src) {
                if matches!(pulse, Pulse::High) {
                    cycle.get_or_insert(i);
                }
            }

            let dst_module = modules.entry(dst).or_default();

            if let Some(next_pulse) = dst_module.propagate(pulse, src) {
                for next_dst in dsts[dst].iter() {
                    pending.push_back((dst, *next_dst, next_pulse));
                }
            }
        }

        let cycle = cycles
            .values()
            .copied()
            .try_fold(1, |prod, cycle| Some(lcm(prod, cycle?)));

        if let Some(cycle) = cycle {
            return Ok(cycle);
        }
    }

    unreachable!()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Default)]
enum Module<'a> {
    FlipFlop {
        is_on: bool,
    },
    Conjunction {
        prev_inputs: Vec<(&'a str, Pulse)>,
    },
    Broadcaster,
    #[default]
    Other,
}

impl Module<'_> {
    fn propagate(&mut self, pulse: Pulse, src: &str) -> Option<Pulse> {
        match self {
            Module::FlipFlop { is_on } => {
                if pulse == Pulse::High {
                    return None;
                }

                *is_on = !*is_on;

                Some(if *is_on { Pulse::High } else { Pulse::Low })
            }
            Module::Conjunction { prev_inputs } => {
                if let Some((_, prev_pulse)) = prev_inputs.iter_mut().find(|(name, _)| *name == src)
                {
                    *prev_pulse = pulse;
                }

                let all_high = prev_inputs.iter().all(|(_, pulse)| *pulse == Pulse::High);

                Some(if all_high { Pulse::Low } else { Pulse::High })
            }
            Module::Broadcaster => Some(pulse),
            Module::Other => None,
        }
    }
}
