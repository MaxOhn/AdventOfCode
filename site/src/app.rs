use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::{
    components::Footer,
    pages::{Aoc22, NotFound},
};

pub enum Msg {}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Route::switch} />
                <Footer />
            </BrowserRouter>
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
            Route::Home => html! { <Aoc22 /> },
            Route::Aoc22 => html! { <Aoc22 /> },
            Route::NotFound => html! { <NotFound /> },
        }
    }
}
