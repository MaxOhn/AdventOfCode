use std::time::Duration;

use aoc22::prelude::Solution;
use eyre::Result;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::components::{FilledInputForm, InputForm};

pub struct Aoc22 {
    solution: Option<Result<DaySolution>>,
}

impl Component for Aoc22 {
    type Message = Result<DaySolution>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { solution: None }
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

        html! {
            <div>
                <InputForm onsubmit={onsubmit} />
                { self.view_solution() }
            </div>
        }
    }
}

impl Aoc22 {
    fn view_solution(&self) -> Html {
        match &self.solution {
            Some(Ok(solution)) => {
                let DaySolution {
                    day,
                    solution,
                    elapsed,
                } = solution;

                html! {
                    <table class="table ml-5">
                        <thead>
                            <tr>
                                <th colspan="2">{ "Day " }{ day }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th style="vertical-align: middle;">{ "Part 1" }</th>
                                <th>
                                    <pre>{ &solution.part1 }</pre>
                                </th>
                            </tr>
                            <tr>
                                <th style="vertical-align: middle;">{ "Part 2" }</th>
                                <th>
                                    <pre>{ &solution.part2 }</pre>
                                </th>
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
        }
    }
}

pub struct DaySolution {
    day: u8,
    solution: Solution,
    elapsed: Duration,
}
