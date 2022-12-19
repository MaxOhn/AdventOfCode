use std::str::FromStr;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let blueprints = parse_blueprints(input)?;

    let p1 = part1(&blueprints);
    let p2 = part2(&blueprints);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(blueprints: &[Blueprint]) -> i32 {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| blueprint.quality_level::<24>(i as i32 + 1))
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> i32 {
    blueprints
        .iter()
        .take(3)
        .map(Blueprint::max_geodes::<32>)
        .product()
}

fn parse_blueprints(input: &str) -> Result<Box<[Blueprint]>> {
    input.lines().map(Blueprint::from_str).collect()
}

#[derive(Clone, Debug, Default)]
struct State {
    minute: i32,

    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,

    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    dont_buy_ore: bool,
    dont_buy_clay: bool,
    dont_buy_obsidian: bool,
    dont_buy_geode: bool,
}

impl State {
    fn step(&mut self) {
        self.minute += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn buy_ore(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore.cost_ore && !self.dont_buy_ore
    }

    fn buy_clay(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.clay.cost_ore && !self.dont_buy_clay
    }

    fn buy_obsidian(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.obsidian.cost_ore
            && self.clay >= blueprint.obsidian.cost_clay
            && !self.dont_buy_obsidian
    }

    fn buy_geode(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode.cost_ore
            && self.obsidian >= blueprint.geode.cost_obsidian
            && !self.dont_buy_geode
    }
}

struct Blueprint {
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
}

impl Blueprint {
    fn quality_level<const DEADLINE: i32>(&self, id: i32) -> i32 {
        self.max_geodes::<DEADLINE>() * id
    }

    fn max_geodes<const DEADLINE: i32>(&self) -> i32 {
        let max_cost_ore = self
            .ore
            .cost_ore
            .max(self.clay.cost_ore)
            .max(self.obsidian.cost_ore)
            .max(self.geode.cost_ore);

        let max_cost_clay = self.obsidian.cost_clay;
        let max_cost_obsidian = self.geode.cost_obsidian;

        let mut stack = Vec::new();

        stack.push(State {
            ore_robots: 1,
            ..Default::default()
        });

        let mut max_geodes = 0;

        while let Some(mut inventory) = stack.pop() {
            if inventory.geodes > max_geodes {
                max_geodes = inventory.geodes;
            }

            if inventory.minute == DEADLINE {
                continue;
            }

            let ore = inventory.buy_ore(self) && inventory.ore_robots < max_cost_ore;
            let clay = inventory.buy_clay(self) && inventory.clay_robots < max_cost_clay;
            let obsidian =
                inventory.buy_obsidian(self) && inventory.obsidian_robots < max_cost_obsidian;
            let geode = inventory.buy_geode(self);

            inventory.step();

            if ore {
                let inventory = State {
                    ore_robots: inventory.ore_robots + 1,
                    ore: inventory.ore - self.ore.cost_ore,
                    dont_buy_ore: false,
                    dont_buy_clay: false,
                    dont_buy_obsidian: false,
                    dont_buy_geode: false,
                    ..inventory
                };

                stack.push(inventory);
            }

            if clay {
                let inventory = State {
                    clay_robots: inventory.clay_robots + 1,
                    ore: inventory.ore - self.clay.cost_ore,
                    dont_buy_ore: false,
                    dont_buy_clay: false,
                    dont_buy_obsidian: false,
                    dont_buy_geode: false,
                    ..inventory
                };

                stack.push(inventory);
            }

            if obsidian {
                let inventory = State {
                    obsidian_robots: inventory.obsidian_robots + 1,
                    ore: inventory.ore - self.obsidian.cost_ore,
                    clay: inventory.clay - self.obsidian.cost_clay,
                    dont_buy_ore: false,
                    dont_buy_clay: false,
                    dont_buy_obsidian: false,
                    dont_buy_geode: false,
                    ..inventory
                };

                stack.push(inventory);
            }

            if geode {
                let inventory = State {
                    geode_robots: inventory.geode_robots + 1,
                    ore: inventory.ore - self.geode.cost_ore,
                    obsidian: inventory.obsidian - self.geode.cost_obsidian,
                    dont_buy_ore: false,
                    dont_buy_clay: false,
                    dont_buy_obsidian: false,
                    dont_buy_geode: false,
                    ..inventory
                };

                stack.push(inventory);
            }

            inventory.dont_buy_ore |= ore;
            inventory.dont_buy_clay |= clay;
            inventory.dont_buy_obsidian |= obsidian;
            inventory.dont_buy_geode |= geode;
            stack.push(inventory);
        }

        max_geodes
    }
}

// 1472 < x

impl FromStr for Blueprint {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(": ").wrap_err("missing `: `")?;
        let mut robots = rest.split(". ");

        let ore = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid ore robot")?;

        let clay = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid clay robot")?;

        let obsidian = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid obsidian robot")?;

        let geode = robots
            .next()
            .wrap_err("missing line")?
            .parse()
            .wrap_err("invalid geode robot")?;

        Ok(Self {
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

struct OreRobot {
    cost_ore: i32,
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
    cost_ore: i32,
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
    cost_ore: i32,
    cost_clay: i32,
}

impl FromStr for ObsidianRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        let cost_clay = (&mut iter)
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
    cost_ore: i32,
    cost_obsidian: i32,
}

impl FromStr for GeodeRobot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        let cost_obsidian = (&mut iter)
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
