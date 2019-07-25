use std::default::Default;
use yew::prelude::*;

type Translation = (f64, f64, f64);
type Rotation = (f64, f64, f64);
type Scale = (f64, f64, f64);

pub enum TransformMsg {}

#[derive(Default)]
pub struct Transform {
    pub translation: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Scale>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            translation: None,
            rotation: None,
            scale: None,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub translation: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Scale>,
}

impl Component for Transform {
    type Message = TransformMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Transform {
            translation: props.translation,
            rotation: props.rotation,
            scale: props.scale,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.translation = props.translation;
        self.rotation = props.rotation;
        self.scale = props.scale;
        true
    }
}

impl Renderable<Transform> for Transform {
    fn view(&self) -> Html<Self> {
        html! {
          <fieldset class="uk-margin-small-top uk-fieldset uk-form-horizontal", >
            <legend class="uk-legend uk-margin-bottom", >{ "Transform" }</legend>
            <div>
              <label class="uk-form-label uk-margin-small-left", >{ "Translation" }
                <input class="uk-input uk-form-width-medium", type="text", placeholder="6.0, 6.0, -6.0", />
              </label>
              <label class="uk-form-label uk-margin-small-left", >{ "Rotation" }
                <input class="uk-input uk-form-width-medium", type="text", placeholder="0.0, 0.0, 1.0, 0.0", />
              </label>
              <label class="uk-form-label uk-margin-small-left", >{ "Scale" }
                <input class="uk-input uk-form-width-medium", type="text", placeholder="0.0, 0.0, 1.0, 0.0", />
              </label>
            </div>
          </fieldset>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            translation: None,
            rotation: None,
            scale: None,
        }
    }
}
