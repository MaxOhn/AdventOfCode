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
    const DEFAULT_YEAR: u16 = 2023;

    pub fn solved_days(self) -> SolvedDays {
        SolvedDays::new(crate::generated::solved_days(self.0))
    }

    pub fn solve_fn(self, day: SolvedDay) -> fn(&str) -> Result<Solution> {
        crate::generated::solve_fn(self.0, day.0)
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
