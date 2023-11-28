use std::fmt::{Display, Formatter, Result as FmtResult};

use aoc_rust::Solution;
use eyre::{Report, Result};
use leptos::{Attribute, IntoAttribute, IntoView, View};
use leptos_router::{Params, ParamsError, ParamsMap};
use wasm_bindgen::JsValue;

use crate::day::{SolvedDay, SolvedDays};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Year(u16);

impl Year {
    const DEFAULT_YEAR: u16 = 2022;

    pub fn solved_days(self) -> SolvedDays {
        match self.0 {
            2022 => SolvedDays::new_up_to(25),
            2023 => SolvedDays::new_up_to(0),
            _ => SolvedDays::default(),
        }
    }

    pub fn solve_fn(self, day: SolvedDay) -> fn(&str) -> Result<Solution> {
        match self.0 {
            2022 => match day.0 {
                1 => aoc22::day01::run,
                2 => aoc22::day02::run,
                3 => aoc22::day03::run,
                4 => aoc22::day04::run,
                5 => aoc22::day05::run,
                6 => aoc22::day06::run,
                7 => aoc22::day07::run,
                8 => aoc22::day08::run,
                9 => aoc22::day09::run,
                10 => aoc22::day10::run,
                11 => aoc22::day11::run,
                12 => aoc22::day12::run,
                13 => aoc22::day13::run,
                14 => aoc22::day14::run,
                15 => aoc22::day15::run,
                16 => aoc22::day16::run,
                17 => aoc22::day17::run,
                18 => aoc22::day18::run,
                19 => aoc22::day19::run,
                20 => aoc22::day20::run,
                21 => aoc22::day21::run,
                22 => aoc22::day22::run,
                23 => aoc22::day23::run,
                24 => aoc22::day24::run,
                25 => aoc22::day25::run,
                _ => |_| eyre::bail!("invalid day"),
            },
            2023 => match day.0 {
                _ => |_| eyre::bail!("invalid day"),
            },
            _ => |_| eyre::bail!("invalid year"),
        }
    }
}

impl Params for Year {
    fn from_map(map: &ParamsMap) -> Result<Self, ParamsError> {
        Ok(map
            .get("year")
            .and_then(|year| year.parse().ok())
            .map(Self)
            .unwrap_or_default())
    }
}

impl Default for Year {
    fn default() -> Self {
        Self(Self::DEFAULT_YEAR)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl IntoAttribute for Year {
    fn into_attribute(self) -> Attribute {
        <u16 as IntoAttribute>::into_attribute(self.0)
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

impl IntoView for Year {
    fn into_view(self) -> View {
        <u16 as IntoView>::into_view(self.0)
    }
}

impl TryFrom<JsValue> for Year {
    type Error = Report;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .as_string()
            .and_then(|n| n.parse().ok().map(Self))
            .ok_or(eyre::eyre!("invalid year"))
    }
}
