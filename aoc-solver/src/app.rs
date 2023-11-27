use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Footer, Solver};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet
            href="https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css"
        />
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.2.1/css/all.min.css"
            integrity="sha512-MV7K8+y+gLIBoVD59lQIYicR65iaqukzvf/nwasF0nqhPay5w/9lJmVM2hMDcnK1OnMGCdVK+iQrJ7lzPJQd1w=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        />
        <Title text="Advent of Code Solver"/>

        <Router>
            <main>
                <Routes base="AdventOfCode".to_owned()>
                    <Route path=":year?" view=Solver/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
