use std::default::Default;
use yew::prelude::*;

pub enum RemovalMsg {}

#[derive(Default)]
pub struct Removal {}

impl Removal {
    pub fn new() -> Self {
        Removal {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Removal {
    type Message = RemovalMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Removal {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Removal> for Removal {
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
