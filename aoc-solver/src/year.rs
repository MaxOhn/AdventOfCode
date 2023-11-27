use std::fmt::{Display, Formatter, Result as FmtResult};

use leptos::{Attribute, IntoAttribute, IntoView, View};
use leptos_router::{Params, ParamsError, ParamsMap};

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
pub struct Year(u16);

impl Year {
    pub fn last_day(self) -> u8 {
        match self.0 {
            2022 => 25,
            2023 => 1,
            _ => 0,
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
        Self(2022)
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
        let ptr = Box::into_raw(self).cast();

        // SAFETY: `Year` is a transparent wrapper around `u16`
        let inner = unsafe { Box::from_raw(ptr) };

        <u16 as IntoAttribute>::into_attribute_boxed(inner)
    }
}

impl IntoView for Year {
    fn into_view(self) -> View {
        <u16 as IntoView>::into_view(self.0)
    }
}
