use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum NotImplementedModalMsg {}

#[derive(Default)]
pub struct NotImplementedModal {}

impl NotImplementedModal {
    pub fn new() -> Self {
        NotImplementedModal {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for NotImplementedModal {
    type Message = NotImplementedModalMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        NotImplementedModal {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<NotImplementedModal> for NotImplementedModal {
    fn view(&self) -> Html<Self> {
        html! {
          <div id="not-implemented", uk-modal="", >
            <div class="uk-modal-dialog uk-modal-body", >
              <h3 class="uk-modal-title", >{ "Apologies!" }</h3>
                  <div class="uk-inline", >
                  <p>{ "We're sorry, but this functionality is not yet implemented."}</p>
                  </div>
                <div>
                  <p class="uk-text-right", >
                    <button class="uk-button uk-button-default uk-modal-close", type="button",>{ "Close" }</button>
                  </p>
                </div>
            </div>
          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}
