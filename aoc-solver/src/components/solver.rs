use leptos::{component, create_signal, view, IntoView, SignalGet};
use leptos_router::use_params;

use crate::{
    components::{Navbar, SolverForm},
    year::Year,
};

#[component]
pub fn Solver() -> impl IntoView {
    let year_param = use_params::<Year>();
    let year = move || year_param.get().unwrap_or_default();
    let (solved, set_solved) = create_signal(None);

    view! {
        <Navbar year/>
        <SolverForm year set_solved/>
        { move || {
            match solved.get() {
                Some(Ok(solution)) => Some(solution),
                Some(Err(err)) => Some(format!("error: {err:#?}")),
                None => None,
            }
        }}
    }
}

// TODO: use ErrorBoundary for solve_day result
