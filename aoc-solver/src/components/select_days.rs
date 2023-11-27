use leptos::{component, view, For, IntoView};

#[component]
pub fn SelectDays<F: Fn() -> u8 + Copy + 'static>(last_day: F) -> impl IntoView {
    view! {
        <div class="select">
            <select name="day">
                <For
                    each=move || (1..=last_day())
                    key=move |day| *day
                    let:day
                >
                    { move || {
                        let selected = day == last_day();

                        view! {
                            <option
                                value={ move || day.to_string() }
                                selected={ move || selected }
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
