use leptos::{component, view, IntoView, WriteSignal};
use leptos_router::FromFormData;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{components::SelectDays, year::Year};

#[component]
pub fn SolverForm<Y>(year: Y, set_solved: WriteSignal<Option<Result<String, ()>>>) -> impl IntoView
where
    Y: Fn() -> Year + Copy + 'static,
{
    let last_day = move || year().last_day();

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let Some(target) = ev.target() else {
            error!("Missing target on SubmitEvent");

            return;
        };

        let Some(form) = target.dyn_ref::<web_sys::HtmlFormElement>() else {
            error!("target is not an HtmlFormElement");

            return;
        };

        let form_data = web_sys::FormData::new_with_form(form).unwrap_throw();

        match SolveInput::from_form_data(&form_data) {
            Ok(input) => set_solved(Some(solve_day(input))),
            Err(err) => error!(%err, "Failed to create SolveInput"),
        }
    };

    view! {
        <form on:submit=on_submit class="mt-5 mr-5 mb-5 ml-5">
            <input type="hidden" name="year" value=year/>
            <div class="field">
                <label class="label">
                    "Puzzle input"
                </label>
                <textarea
                    class="textarea"
                    name="input"
                    autofocus="true"
                    rows=6
                    placeholder="Copy-paste your puzzle input in here"
                    required="true"
                />
            </div>
            <div class="field is-grouped">
                <div class="control">
                    <div>
                        <SelectDays last_day/>
                    </div>
                </div>
                <div class="control">
                    <input
                        class="button is-primary"
                        type="submit"
                        value="Solve"
                    />
                </div>
            </div>
        </form>
    }
}

#[derive(Clone, serde::Deserialize)]
struct SolveInput {
    year: Year,
    day: u8,
    input: String,
}

fn solve_day(input: SolveInput) -> Result<String, ()> {
    let SolveInput { year, day, input } = input;

    info!("solving for year={year} | day={day} | input={input}");

    // synthetic delay
    for i in 0..50_000_000_u64 {
        let _ = i / 3;
    }

    Ok(format!("Solution: year={year} | day={day} | input={input}"))
}
