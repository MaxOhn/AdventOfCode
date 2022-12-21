use std::time::Duration;

use aoc22::prelude::Solution;
use eyre::Result;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::components::{FilledInputForm, InputForm};

macro_rules! day_from_str {
    ( $final_day:literal: $final_mod:ident, $( $n:literal: $mod:ident ,)* ) => {
        pub const FINAL_DAY: u8 = $final_day;

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
        impl std::str::FromStr for Day {
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
    21: day21,
    20: day20,
    19: day19,
    18: day18,
    17: day17,
    16: day16,
    15: day15,
    14: day14,
    13: day13,
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
                    <article class="message is-primary ml-5 mr-5">
                        <div class="message-body">
                            <table class="table">
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
                        </div>
                    </article>
                }
            }
            Some(Err(err)) => {
                let mut chain = err.chain();

                html! {
                        <article class="message is-danger ml-5 mr-5">
                            <div class="message-header">
                                <p>{ "Error" }</p>
                            </div>
                            <div class="message-body">
                                if let Some(err)= chain.next() {
                                    <p>{ err }</p>
                                }
                                { chain.map(|err| html!(<p>{ "- caused by: " } { err }</p>)).collect::<Html>() }
                            </div>
                        </article>
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

#[derive(Copy, Clone)]
pub struct Day {
    pub day: u8,
    pub run: fn(&str) -> Result<Solution>,
}
