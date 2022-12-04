use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct TextArea;

#[derive(PartialEq, Properties)]
pub struct TextAreaProps {
    pub onchange: Callback<String>,
}

impl Component for TextArea {
    type Message = ();
    type Properties = TextAreaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx
            .props()
            .onchange
            .reform(|e: Event| e.target_unchecked_into::<HtmlInputElement>().value());

        html! {
            <textarea
                autofocus=true
                rows="10"
                cols="50"
                placeholder="Copy-paste your puzzle input in here"
                required=true
                {onchange}
            />
        }
    }
}
