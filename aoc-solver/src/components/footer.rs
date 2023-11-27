use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <nav class="navbar is-fixed-bottom is-active is-light">
            <span class="navbar-item">
                <a class="button is-dark" href="https://github.com/MaxOhn/AdventOfCode" target="_blank">
                    <span class="icon">
                        <i class="fab fa-github"></i>
                    </span>
                    <span>
                        "Github"
                    </span>
                </a>
            </span>
        </nav>
    }
}
