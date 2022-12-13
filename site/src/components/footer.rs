use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar is-fixed-bottom is-active is-light">
                <span class="navbar-item">
                    <a class="button is-dark" href="https://github.com/MaxOhn/AdventOfCode" target="_blank">
                        <span class="icon">
                            <i class="fab fa-github"></i>
                        </span>
                        <span>{ "Github" }</span>
                    </a>
                </span>
            </nav>
        }
    }
}
