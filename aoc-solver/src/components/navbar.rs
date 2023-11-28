use leptos::{component, view, IntoView};

use crate::year::Year;

#[component]
pub fn Navbar<Y: Fn() -> Year + 'static>(year: Y) -> impl IntoView {
    view! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">
                    <a href="/AdventOfCode">
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
                            <NavbarYear year=2015/>
                            <NavbarYear year=2022/>
                            <NavbarYear year=2023/>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavbarYear(year: u16) -> impl IntoView {
    view! {
        <a href=move || format!("/AdventOfCode/{year}") class="navbar-item">
            { year }
        </a>
    }
}
