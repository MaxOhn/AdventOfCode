use yew::{Component, Context, Html, html};

pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <h1 class="title">
                    { "Page not found" }
                </h1>
                <h2 class="subtitle">
                    { "Page page does not seem to exist" }
                </h2>
            </div>
        }
    }
}
