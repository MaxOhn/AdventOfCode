use std::time::Duration;

use aoc22::prelude::Solution;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::components::{FilledInputForm, InputForm};

#[derive(Default)]
pub struct Home {
    result: Option<DayResult>,
}

impl Component for Home {
    type Message = DayResult;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.result = Some(msg);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|FilledInputForm { day, input }| {
            let start = Instant::now();
            let solution = (day.run)(&input);
            let elapsed = start.elapsed();

            DayResult {
                day: day.day,
                solution,
                elapsed,
            }
        });

        html! {
            <div>
                <h1>{ "Advent Of Code 2022 Solver" }</h1>
                <InputForm onsubmit={onsubmit} />
                if let Some(ref res) = self.result {
                    { res.to_html() }
                }
            </div>
        }
    }
}

pub struct DayResult {
    day: u8,
    solution: Solution,
    elapsed: Duration,
}

impl DayResult {
    fn to_html(&self) -> Html {
        let Self {
            day,
            solution,
            elapsed,
        } = self;

        html! {
            <>
                <h1>{ "Day " } { day } { ":" }</h1>
                <p>{ "Part 1: " } { &solution.part1 }</p>
                <p>{ "Part 2: " } { &solution.part2 }</p>
                <h4>{ "Elapsed: " } { format!("{elapsed:?}") }</h4>
            </>
        }
    }
}
