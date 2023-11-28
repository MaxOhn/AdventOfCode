use leptos::{component, view, For, IntoView};

use crate::day::SolvedDays;

#[component]
pub fn SelectDays<F: Fn() -> SolvedDays + Copy + 'static>(solved_days: F) -> impl IntoView {
    view! {
        <div class="select">
            <select name="day">
                <For
                    each=solved_days
                    key=move |day| *day
                    let:day
                >
                    { move || {
                        let selected = solved_days().last_day().is_some_and(|last| day == last);

                        view! {
                            <option
                                value={ day.to_string() }
                                selected={ selected }
                            >
                                { move || {
                                    format!("Day {day:0>2}")
                                }}
                            </option>
                        }
                    }}
                </For>
            </select>
        </div>
    }
}
