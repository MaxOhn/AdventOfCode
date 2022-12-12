use std::str::FromStr;

use aoc22::prelude::Solution;
use eyre::Result;
use web_sys::HtmlInputElement;
use yew::prelude::*;

macro_rules! day_from_str {
    ( $final_day:literal: $final_mod:ident, $( $n:literal: $mod:ident ,)* ) => {
        const FINAL_DAY: u8 = $final_day;

        impl Default for Day {
            fn default() -> Self {
                Self {
                    day: $final_day,
                    run: aoc22::$final_mod::run,
                }
            }
        }

        day_from_str!(@ $final_day: $final_mod, $( $n:$mod ,)*);
    };
    (@ $( $n:literal: $mod:ident ,)+ ) => {
        impl FromStr for Day {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( stringify!($n) => Ok(Self {
                        day: $n,
                        run: aoc22::$mod::run
                    }), )*
                    _ => Err(()),
                }
            }
        }
    }
}

day_from_str! {
    12: day12,
    11: day11,
    10: day10,
    9: day09,
    8: day08,
    7: day07,
    6: day06,
    5: day05,
    4: day04,
    3: day03,
    2: day02,
    1: day01,
}

pub struct SelectDay;

#[derive(PartialEq, Properties)]
pub struct SelectDayProps {
    pub onchange: Callback<Day>,
}

impl Component for SelectDay {
    type Message = ();
    type Properties = SelectDayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.props().onchange.reform(|e: Event| {
            e.target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse()
                .unwrap()
        });

        html! {
            <div>
                // <label class="label">{ "Select day" }</label>
                <div class="select">
                    <select name="day" {onchange}>
                        { final_day_to_options() }
                    </select>
                </div>
            </div>
        }
    }
}

fn final_day_to_options() -> Html {
    (1..=FINAL_DAY)
        .map(|day| day_to_option(day, day == FINAL_DAY))
        .collect()
}

fn day_to_option(day: u8, selected: bool) -> Html {
    html! {
        <option value={day.to_string()} selected={selected}>
            { format!("Day {day:0>2}") }
        </option>
    }
}

#[derive(Copy, Clone)]
pub struct Day {
    pub day: u8,
    pub run: fn(&str) -> Result<Solution>,
}
