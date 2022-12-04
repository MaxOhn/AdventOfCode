use yew::prelude::*;

use super::{
    input_select::{Day, SelectDay},
    input_textarea::TextArea,
};

#[derive(Clone, Default)]
pub struct InputForm {
    pub text: Option<String>,
    pub day: Day,
}

pub struct FilledInputForm {
    pub input: String,
    pub day: Day,
}

impl From<InputForm> for FilledInputForm {
    fn from(value: InputForm) -> Self {
        Self {
            input: value.text.expect("missing input"),
            day: value.day,
        }
    }
}

pub enum InputFormMsg {
    Select(Day),
    Text(String),
}

#[derive(PartialEq, Properties)]
pub struct InputFormProps {
    pub onsubmit: Callback<FilledInputForm>,
}

impl Component for InputForm {
    type Message = InputFormMsg;
    type Properties = InputFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputFormMsg::Select(day) => self.day = day,
            InputFormMsg::Text(text) => self.text = Some(text),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.to_owned();

        let onsubmit = ctx.props().onsubmit.reform(move |e: SubmitEvent| {
            e.prevent_default();

            state.clone().into()
        });

        let select_update = ctx.link().callback(InputFormMsg::Select);
        let text_update = ctx.link().callback(InputFormMsg::Text);

        html! {
            <form {onsubmit}>
                <TextArea onchange={text_update} />
                <br/>
                <SelectDay onchange={select_update} />
                <input type="submit" value="Solve"/>
            </form>
        }
    }
}
