use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;

pub struct Navbar {
    navbar_active: bool,
}

pub enum NavbarMsg {
    ToggleNavbar,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavbarMsg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">
                        <Link<Route> to={Route::Home}>{ "AoC Solver" }</Link<Route>>
                    </h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| NavbarMsg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Year" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Aoc22}>
                                    { "2022" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
