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
            2015 => SolvedDays::new(0b0000000000000000001000000),
            2016 => SolvedDays::new(0b0000000001111111111111100),
            2022 => SolvedDays::new_up_to(25),
            2023 => SolvedDays::new_up_to(0),
            _ => SolvedDays::default(),
        }
    }

    pub fn solve_fn(self, day: SolvedDay) -> fn(&str) -> Result<Solution> {
        macro_rules! match_year_day {
            (
                $(
                    $year:literal: {
                        $(
                            $day:literal => $( $path:ident )::+
                        ),* $(,)?
                    }
                ,)*
            ) => {
                match self.0 {
                    $(
                        $year => match day.0 {
                            $(
                                $day => $( $path :: )* run,
                            )*
                            _ => |_| eyre::bail!("invalid day"),
                        }
                    )*
                    _ => |_| eyre::bail!("invalid year"),
                }
            };
        }

        match_year_day! {
            2015: {
                7 => aoc15_day07
            },
            2016: {
                3 => aoc16_day03,
                4 => aoc16_day04,
                5 => aoc16_day05,
                6 => aoc16_day06,
                7 => aoc16_day07,
                8 => aoc16_day08,
                9 => aoc16_day09,
                10 => aoc16_day10,
                11 => aoc16_day11,
                12 => aoc16_day12,
                13 => aoc16_day13,
                14 => aoc16_day14,
                15 => aoc16_day15,
                16 => aoc16_day16,
            },
            2022: {
                1 => aoc22::day01,
                2 => aoc22::day02,
                3 => aoc22::day03,
                4 => aoc22::day04,
                5 => aoc22::day05,
                6 => aoc22::day06,
                7 => aoc22::day07,
                8 => aoc22::day08,
                9 => aoc22::day09,
                10 => aoc22::day10,
                11 => aoc22::day11,
                12 => aoc22::day12,
                13 => aoc22::day13,
                14 => aoc22::day14,
                15 => aoc22::day15,
                16 => aoc22::day16,
                17 => aoc22::day17,
                18 => aoc22::day18,
                19 => aoc22::day19,
                20 => aoc22::day20,
                21 => aoc22::day21,
                22 => aoc22::day22,
                23 => aoc22::day23,
                24 => aoc22::day24,
                25 => aoc22::day25,
            },
            2023: {
            },
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
