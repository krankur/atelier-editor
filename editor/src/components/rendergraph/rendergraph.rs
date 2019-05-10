use std::default::Default;

use yew::prelude::*;

pub enum Msg {

}

#[derive(Clone, PartialEq)]
pub struct Props {
    
}

pub struct RenderGraph;

impl Default for Props {
    fn default() -> Props {
        Props { }
    }
}

impl Component for RenderGraph {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        RenderGraph {
            
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<RenderGraph> for RenderGraph {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="uk-flex-center uk-margin-small-top", uk-grid="", id="rendergraph_body", >

            </div>
        }
    }
}
