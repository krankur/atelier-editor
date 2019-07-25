use std::default::Default;

use yew::html::Renderable;
use yew::prelude::*;

pub enum FormMsg {}

#[derive(Clone, PartialEq)]
pub enum ComponentsForm {}

#[derive(Default)]
pub struct Form {}

impl Form {
    pub fn new() -> Self {
        Form {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Form {
    type Message = FormMsg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Form {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Form> for Form {
    fn view(&self) -> Html<Self> {
        html! {
          <div class="uk-width-1-1", >
            <h1 class="uk-align-center", >{ "UI Prefab" }</h1>
            <form class="uk-form-stacked", >
              <button class="uk-button uk-button-default uk-width-1-1 uk-align-center", type="button", >{ "Components" }</button>
                <div uk-dropdown="", >
                    <ul class="uk-nav uk-dropdown-nav", >
                        <li class="", ><a href="#", >{ "Component A" }</a></li>
                        <li><a href="#",>{ "Component B" }</a></li>
                    </ul>
                </div>
            </form>
          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}
