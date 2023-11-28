use std::time::Duration;

use aoc_rust::Solution;
use eyre::Result;
use leptos::{component, create_signal, view, IntoView, SignalGet};
use leptos_router::use_params;
use wasm_timer::Instant;
use web_sys::FormData;

use crate::{
    components::{Navbar, Solution as SolutionComponent, SolverForm},
    day::SolvedDay,
    year::Year,
};

#[component]
pub fn Solver() -> impl IntoView {
    let year_param = use_params::<Year>();
    let year = move || year_param.get().unwrap_or_default();
    let (solved, set_solved) = create_signal(None);
    let on_input = move |input| set_solved(Some(solve(input)));

    view! {
        <Navbar year/>
        <SolverForm year on_input/>
        <SolutionComponent solved/>
    }
}

#[derive(Clone)]
pub struct SolveInput {
    year: Year,
    day: SolvedDay,
    input: String,
}

impl SolveInput {
    pub fn new(form: &FormData) -> Result<Self> {
        Ok(Self {
            year: form.get("year").try_into()?,
            day: form.get("day").try_into()?,
            input: form
                .get("input")
                .as_string()
                .ok_or(eyre::eyre!("invalid input"))?,
        })
    }
}

#[derive(Clone)]
pub struct DaySolution {
    pub solution: Solution,
    pub elapsed: Duration,
}

fn solve(input: SolveInput) -> Result<DaySolution> {
    let SolveInput { year, day, input } = input;

    let solve_fn = year.solve_fn(day);

    let start = Instant::now();
    let solution = solve_fn(&input)?;
    let elapsed = start.elapsed().max(Duration::from_millis(1));

    Ok(DaySolution { solution, elapsed })
}
