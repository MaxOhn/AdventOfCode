use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::pages::aoc22::{Day, FINAL_DAY};

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
