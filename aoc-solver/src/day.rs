use std::fmt::{Display, Formatter, Result as FmtResult};

use eyre::Report;
use wasm_bindgen::JsValue;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SolvedDay(pub u8);

impl Display for SolvedDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl TryFrom<JsValue> for SolvedDay {
    type Error = Report;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .as_string()
            .and_then(|n| n.parse().ok().map(Self))
            .ok_or(eyre::eyre!("invalid day"))
    }
}

#[derive(Copy, Clone, Default)]
pub struct SolvedDays(u32);

impl SolvedDays {
    const MAX_DAY: u32 = 25;

    pub fn new(bits: u32) -> Self {
        Self(bits)
    }

    pub fn last_day(self) -> Option<SolvedDay> {
        let last_day = 25 + (u32::BITS - Self::MAX_DAY) - self.0.leading_zeros();

        (last_day > 0).then_some(SolvedDay(last_day as u8))
    }
}

impl IntoIterator for SolvedDays {
    type Item = SolvedDay;
    type IntoIter = SolvedDaysIter;

    fn into_iter(self) -> Self::IntoIter {
        SolvedDaysIter { state: self.into() }
    }
}

pub struct SolvedDaysIter {
    state: SolvedDaysIterState,
}

impl Iterator for SolvedDaysIter {
    type Item = SolvedDay;

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.state.idx();
        let solved = self.state.solved();

        loop {
            if idx >= 25 {
                return None;
            }

            let is_solved = (solved & (1 << idx)) > 0;
            self.state.incr();
            idx += 1;

            if is_solved {
                return Some(SolvedDay(idx as u8));
            }
        }
    }
}

#[derive(Copy, Clone)]
struct SolvedDaysIterState(
    /// Contains both the index and the solved days:
    /// |_|_|_|_|_|_|_|_|...|_|
    /// \_______/\_/\_________/
    ///     |     |         |
    ///     | 2 unused bits |
    /// 5 bits for index    |
    ///       25 bits for solved days
    u32,
);

impl SolvedDaysIterState {
    const IDX_LEN: u32 = 5;
    const IDX_SHIFT: u32 = u32::BITS - Self::IDX_LEN;
    const SOLVED_MASK: u32 = (1 << SolvedDays::MAX_DAY) - 1;

    fn idx(self) -> u32 {
        self.0 >> Self::IDX_SHIFT
    }

    fn solved(self) -> u32 {
        self.0 & Self::SOLVED_MASK
    }

    fn incr(&mut self) {
        self.0 = (self.0.rotate_left(Self::IDX_LEN) + 1).rotate_right(Self::IDX_LEN);
    }
}

impl From<SolvedDays> for SolvedDaysIterState {
    fn from(solved: SolvedDays) -> Self {
        Self(solved.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_days_last() {
        assert_eq!(SolvedDays(0x1000010).last_day().unwrap().0, 25);
        assert_eq!(SolvedDays(0x80C000).last_day().unwrap().0, 24);
        assert_eq!(SolvedDays(0b10).last_day().unwrap().0, 2);
        assert_eq!(SolvedDays(0b11).last_day().unwrap().0, 2);
        assert_eq!(SolvedDays(0b1).last_day().unwrap().0, 1);
        assert!(SolvedDays(0b0).last_day().is_none());
    }

    #[test]
    fn solved_days_iter() {
        let solved: Vec<_> = SolvedDays(0b11001000011000110110010001)
            .into_iter()
            .map(|solved| solved.0)
            .collect();

        assert_eq!(solved, vec![1, 5, 8, 9, 11, 12, 16, 17, 22, 25]);
    }
}
