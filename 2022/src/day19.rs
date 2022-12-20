use std::str::FromStr;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let blueprints = parse_blueprints(input)?;

    let p1 = part1(&blueprints);
    let p2 = part2(&blueprints);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(blueprints: &[Blueprint]) -> u16 {
    blueprints
        .iter()
        .zip(1..)
        .map(|(blueprint, id)| blueprint.quality_level::<24>(id))
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> u16 {
    blueprints
        .iter()
        .take(3)
        .map(Blueprint::max_geodes::<32>)
        .product()
}

fn parse_blueprints(input: &str) -> Result<Box<[Blueprint]>> {
    input.lines().map(Blueprint::from_str).collect()
}

#[derive(Copy, Clone, Default)]
struct State {
    minute: u8,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,

    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,

    dont_buy: DontBuy,
}

#[derive(Copy, Clone, Default)]
struct DontBuy {
    bitset: u8,
}

macro_rules! dont_buy_ore {
    ( $( $const_name:ident: $const_val:literal, $set:ident, $get:ident ;)* ) => {
        impl DontBuy {
            $(
                const $const_name: u8 = $const_val;

                fn $set(&mut self, flag: bool) {
                    self.bitset |= Self:: $const_name * flag as u8;
                }

                fn $get(self) -> bool {
                    self.bitset & Self:: $const_name > 0
                }
            )*
        }
    }
}

dont_buy_ore! {
    ORE:      0b0001, set_ore,      ore;
    CLAY:     0b0010, set_clay,     clay;
    OBSIDIAN: 0b0100, set_obsidian, obsidian;
    GEODE:    0b1000, set_geode,    geode;
}

impl State {
    fn collect_resources(&mut self) {
        self.minute += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn buy_ore(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore.cost_ore
            && !self.dont_buy.ore()
            && self.ore_robots < blueprint.max_cost_ore
    }

    fn buy_clay(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.clay.cost_ore
            && !self.dont_buy.clay()
            && self.clay_robots < blueprint.max_cost_clay
    }

    fn buy_obsidian(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.obsidian.cost_ore
            && self.clay >= blueprint.obsidian.cost_clay
            && !self.dont_buy.obsidian()
            && self.obsidian_robots < blueprint.max_cost_obsidian
    }

    fn buy_geode(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode.cost_ore
            && self.obsidian >= blueprint.geode.cost_obsidian
            && !self.dont_buy.geode()
    }
}

struct Blueprint {
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,

    max_cost_ore: u16,
    max_cost_clay: u16,
    max_cost_obsidian: u16,
}

impl Blueprint {
    fn quality_level<const DEADLINE: u8>(&self, id: u16) -> u16 {
        self.max_geodes::<DEADLINE>() * id
    }

    fn max_geodes<const DEADLINE: u8>(&self) -> u16 {
        let mut stack = vec![State {
            ore_robots: 1,
            ..Default::default()
        }];

        let mut max_geodes = 0;

        while let Some(mut state) = stack.pop() {
            if state.geodes > max_geodes {
                max_geodes = state.geodes;
            }

            if state.minute == DEADLINE {
                continue;
            }

            let remaining = (DEADLINE - state.minute) as u16;

            let max_geodes_possible =
                ((remaining - 1) * remaining) / 2 + remaining * state.geode_robots + state.geodes;

            if max_geodes_possible < max_geodes {
                continue;
            }

            let ore = state.buy_ore(self);
            let clay = state.buy_clay(self);
            let obsidian = state.buy_obsidian(self);
            let geode = state.buy_geode(self);

            state.collect_resources();

            if ore {
                let state = State {
                    ore_robots: state.ore_robots + 1,
                    ore: state.ore - self.ore.cost_ore,
                    dont_buy: DontBuy::default(),
                    ..state
                };

                stack.push(state);
            }

            if clay {
                let state = State {
                    clay_robots: state.clay_robots + 1,
                    ore: state.ore - self.clay.cost_ore,
                    dont_buy: DontBuy::default(),
                    ..state
                };

                stack.push(state);
            }

            if obsidian {
                let state = State {
                    obsidian_robots: state.obsidian_robots + 1,
                    ore: state.ore - self.obsidian.cost_ore,
                    clay: state.clay - self.obsidian.cost_clay,
                    dont_buy: DontBuy::default(),
                    ..state
                };

                stack.push(state);
            }

            if geode {
                let state = State {
                    geode_robots: state.geode_robots + 1,
                    ore: state.ore - self.geode.cost_ore,
                    obsidian: state.obsidian - self.geode.cost_obsidian,
                    dont_buy: DontBuy::default(),
                    ..state
                };

                stack.push(state);
            }

            state.dont_buy.set_ore(ore);
            state.dont_buy.set_clay(clay);
            state.dont_buy.set_obsidian(obsidian);
            state.dont_buy.set_geode(geode);

            stack.push(state);
        }

        max_geodes
    }
}

impl FromStr for Blueprint {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(": ").wrap_err("missing `: `")?;
        let mut robots = rest.split(". ");

        let ore: OreRobot = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid ore robot")?;

        let clay: ClayRobot = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid clay robot")?;

        let obsidian: ObsidianRobot = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid obsidian robot")?;

        let geode: GeodeRobot = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid geode robot")?;

        let max_cost_ore = ore
            .cost_ore
            .max(clay.cost_ore)
            .max(obsidian.cost_ore)
            .max(geode.cost_ore);

        let max_cost_clay = obsidian.cost_clay;
        let max_cost_obsidian = geode.cost_obsidian;

        Ok(Self {
            ore,
            clay,
            obsidian,
            geode,

            max_cost_ore,
            max_cost_clay,
            max_cost_obsidian,
        })
    }
}

struct OreRobot {
    cost_ore: u16,
}

impl FromStr for OreRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cost_ore = s
            .split(' ')
            .nth_back(1)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid ore cost")?;

        Ok(Self { cost_ore })
    }
}

struct ClayRobot {
    cost_ore: u16,
}

impl FromStr for ClayRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cost_ore = s
            .split(' ')
            .nth_back(1)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid ore cost")?;

        Ok(Self { cost_ore })
    }
}

struct ObsidianRobot {
    cost_ore: u16,
    cost_clay: u16,
}

impl FromStr for ObsidianRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        let cost_clay = iter
            .nth_back(1)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid clay cost")?;

        let cost_ore = iter
            .nth_back(2)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid ore cost")?;

        Ok(Self {
            cost_ore,
            cost_clay,
        })
    }
}

struct GeodeRobot {
    cost_ore: u16,
    cost_obsidian: u16,
}

impl FromStr for GeodeRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        let cost_obsidian = iter
            .nth_back(1)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid obsidian cost")?;

        let cost_ore = iter
            .nth_back(2)
            .and_then(|n| n.parse().ok())
            .wrap_err("invalid ore cost")?;

        Ok(Self {
            cost_ore,
            cost_obsidian,
        })
    }
}
