use std::default::Default;
use yew::prelude::*;

pub enum GraphicsMsg {}

#[derive(Default)]
pub struct Graphics {}

impl Graphics {
    pub fn new() -> Self {
        Graphics {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Graphics {
    type Message = GraphicsMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Graphics {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Graphics> for Graphics {
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
