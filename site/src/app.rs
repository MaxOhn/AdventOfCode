use yew::{html, Component, Context, Html};
use yew_router::{prelude::Redirect, BrowserRouter, Routable, Switch};

use crate::{
    components::{Footer, Navbar},
    pages::{aoc22::Aoc22, NotFound},
};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <BrowserRouter>
                    <Navbar />
                    <Switch<Route> render={Route::switch} />
                    <Footer />
                </BrowserRouter>
            </main>
        }
    }
}

#[derive(Copy, Clone, Routable, Eq, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/2022")]
    Aoc22,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    fn switch(self) -> Html {
        match self {
            Route::Home => html! { <Redirect<Route> to={Route::Aoc22} /> },
            Route::Aoc22 => html! { <Aoc22 /> },
            Route::NotFound => html! { <NotFound /> },
        }
    }
}
