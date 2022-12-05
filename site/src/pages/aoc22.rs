use std::time::Duration;

use aoc22::prelude::Solution;
use eyre::Result;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::components::{FilledInputForm, InputForm};

#[derive(Default)]
pub struct Aoc22 {
    solution: Option<Result<DaySolution>>,
}

impl Component for Aoc22 {
    type Message = Result<DaySolution>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.solution = Some(msg);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|FilledInputForm { day, input }| {
            let start = Instant::now();
            let res = (day.run)(&input);
            let elapsed = start.elapsed().max(Duration::from_millis(1));

            res.map(|solution| DaySolution {
                day: day.day,
                solution,
                elapsed,
            })
        });

        let solution_res = match &self.solution {
            Some(Ok(solution)) => html! { <DayResult solution={solution.to_owned()} /> },
            Some(Err(err)) => {
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
            None => html! {},
        };

        html! {
            <div>
                <InputForm onsubmit={onsubmit} />
                { solution_res }
            </div>
        }
    }
}

#[derive(Clone)]
pub struct DaySolution {
    day: u8,
    solution: Solution,
    elapsed: Duration,
}

impl PartialEq for DaySolution {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}

struct DayResult;

#[derive(PartialEq, Properties)]
struct DayResultProps {
    solution: DaySolution,
}

impl Component for DayResult {
    type Message = ();
    type Properties = DayResultProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let DaySolution {
            day,
            solution,
            elapsed,
        } = &ctx.props().solution;

        html! {
            <table class="table ml-5">
                <thead>
                    <tr>
                        <th colspan="2">{ "Day " }{ day }</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <th>{ "Part 1" }</th>
                        <th>{ &solution.part1 }</th>
                    </tr>
                    <tr>
                        <th>{ "Part 2" }</th>
                        <th>{ &solution.part2 }</th>
                    </tr>
                </tbody>
                <tfoot>
                <tr>
                <th>{ "Elapsed" }</th>
                <th>{ format!("{elapsed:?}") }</th>
                </tr>
                </tfoot>
            </table>
        }
    }
}
