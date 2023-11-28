use aoc_rust::SolutionType;
use eyre::{Report, Result};
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalWith, View};

use crate::components::solver::DaySolution;

#[component]
pub fn Solution(solved: ReadSignal<Option<Result<DaySolution>>>) -> impl IntoView {
    let solved_opt = move || {
        solved.with(|opt| {
            opt.as_ref()
                .map(|res| res.as_ref().map(Clone::clone).map_err(solution_err))
        })
    };

    view! {
        { move || {
            match solved_opt() {
                Some(Ok(solution)) => {
                    let DaySolution { solution, elapsed } = solution;

                    view! {
                        <article class="message is-primary ml-5 mr-5">
                            <div class="message-body">
                                <table class="table">
                                    <tbody>
                                        <Part part=1 solution=solution.part1/>
                                        <Part part=2 solution=solution.part2/>
                                    </tbody>
                                    <tfoot>
                                        <tr>
                                            <th> "Elapsed" </th>
                                            <th>
                                                { format!("{elapsed:?}") }
                                            </th>
                                        </tr>
                                    </tfoot>
                                </table>
                            </div>
                        </article>
                    }.into_view()
                },
                Some(Err(err)) => view! {
                    <article class="message is-danger ml-5 mr-5">
                        <div class="message-header">
                            <p> "Error" </p>
                        </div>
                        <div class="message-body">
                            { err }
                        </div>
                    </article>
                }.into_view(),
                None => ().into_view(),
            }
        }}
    }
}

#[component]
fn Part(part: u8, solution: SolutionType) -> impl IntoView {
    view! {
        <tr>
            <th style="vertical-align: middle;">
                "Part " { part }
            </th>
            <th>
                <pre>
                    { solution.to_string() }
                </pre>
            </th>
        </tr>
    }
}

fn solution_err(err: &Report) -> View {
    let mut chain = err.chain();

    let Some(e) = chain.next() else {
        return ().into_view();
    };

    let first = view! {
        <p> { e.to_string() } </p>
    };

    let mapped_chain = chain.map(|e| {
        view! {
            <p> "- caused by: " { e.to_string() } </p>
        }
    });

    std::iter::once(first).chain(mapped_chain).collect_view()
}
