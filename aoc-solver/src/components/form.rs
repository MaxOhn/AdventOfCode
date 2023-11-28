use leptos::{component, view, IntoView};
use leptos_router::FromFormData;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{
    components::{solver::SolveInput, SelectDays},
    year::Year,
};

#[component]
pub fn SolverForm<Y, I>(year: Y, on_input: I) -> impl IntoView
where
    Y: Fn() -> Year + Copy + 'static,
    I: Fn(SolveInput) + 'static,
{
    let solved_days = move || year().solved_days();

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
            Ok(input) => on_input(input),
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
                        <SelectDays solved_days/>
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
