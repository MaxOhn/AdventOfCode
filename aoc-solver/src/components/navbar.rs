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
                            <a href="/AdventOfCode/2022" class="navbar-item">
                                "2022"
                            </a>
                            <a href="/AdventOfCode/2023" class="navbar-item">
                                "2023"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
