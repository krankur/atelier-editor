use std::default::Default;
use yew::prelude::*;

pub enum ControlMsg {}

#[derive(Default)]
pub struct Control {}

impl Control {
    pub fn new() -> Self {
        Control {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Control {
    type Message = ControlMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Control {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Control> for Control {
    fn view(&self) -> Html<Self> {
        html! {
          <div>

          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}
