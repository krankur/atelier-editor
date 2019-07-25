use std::default::Default;
use yew::prelude::*;

pub enum EditorToolbarMsg {}

#[derive(Default)]
pub struct EditorToolbar {}

impl EditorToolbar {
    pub fn new() -> Self {
        EditorToolbar {}
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for EditorToolbar {
    type Message = EditorToolbarMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EditorToolbar {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<EditorToolbar> for EditorToolbar {
    fn view(&self) -> Html<Self> {
        html! {
          <div id="editor-toolbar",>
            <img draggable="false", class="editor-icon", src="img/editor-toolbar.svg", />
          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}
