use leptos::{component, view, IntoView, Memo, SignalWith};
use leptos_router::{use_navigate, use_router, NavigateOptions};
use web_sys::MouseEvent;

use crate::year::Year;

#[component]
pub fn Navbar<Y: Fn() -> Year + 'static>(year: Y) -> impl IntoView {
    let path = use_router().pathname();

    view! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">
                    <a href=path>
                        "AoC Solver"
                    </a>
                </h1>
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link has-text-weight-bold">
                            { year }
                        </div>
                        <div class="navbar-dropdown">
                            <NavbarYear year=2015 path/>
                            <NavbarYear year=2016 path/>
                            <NavbarYear year=2022 path/>
                            <NavbarYear year=2023 path/>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavbarYear(year: u16, path: Memo<String>) -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        let search = path.with(|path| format!("{path}?year={year}"));

        let navigate = use_navigate();

        let options = NavigateOptions {
            scroll: false,
            replace: false,
            ..Default::default()
        };

        navigate(&search, options);

        ev.prevent_default();
        ev.stop_propagation();
    };

    view! {
        <a class="navbar-item" on:click=on_click>
            { year }
        </a>
    }
}
