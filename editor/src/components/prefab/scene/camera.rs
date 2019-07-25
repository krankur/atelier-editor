use std::default::Default;
use yew::prelude::*;

pub enum CameraMsg {}

#[derive(Default, Clone, PartialEq)]
pub struct Perspective {
    aspect: Option<f64>,
    fovy: Option<f64>,
    znear: Option<f64>,
    zfar: Option<f64>,
}

#[derive(Default)]
pub struct Camera {
    perspective: Option<Perspective>,
}

impl Camera {
    pub fn new() -> Self {
        Camera { perspective: None }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub perspective: Option<Perspective>,
}

impl Component for Camera {
    type Message = CameraMsg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Camera { perspective: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.perspective = props.perspective;
        true
    }
}

impl Renderable<Camera> for Camera {
    fn view(&self) -> Html<Self> {
        html! {
          <div>

          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props { perspective: None }
    }
}
