use std::time::Duration;

use aoc22::prelude::Solution;
use eyre::Result;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::components::{FilledInputForm, InputForm};

#[derive(Default)]
pub struct Aoc22 {
    result: Option<DayResult>,
}

impl Component for Aoc22 {
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
    solution: Result<Solution>,
    elapsed: Duration,
}

impl DayResult {
    fn to_html(&self) -> Html {
        let Self {
            day,
            solution,
            elapsed,
        } = self;

        let solution_html = match solution {
            Ok(solution) => html! {
                <>
                    <p>{ format!("Part 1: {}", &solution.part1) }</p>
                    <p>{ format!("Part 2: {}", &solution.part2) }</p>
                    <p>{ format!("Elapsed: {elapsed:?}") }</p>
                </>
            },
            Err(err) => {
                let mut chain = err.chain();

                html! {
                    <>
                        if let Some(err)= chain.next() {
                            <p>{ err }</p>
                        }
                        { chain.map(|err| html!(<p>{ "- caused by: " } { err }</p>)).collect::<Html>() }
                    </>
                }
            }
        };

        html! {
            <div>
                <h2>{ format!("Day {day}:") }</h2>
                { solution_html }
            </div>
        }
    }
}
