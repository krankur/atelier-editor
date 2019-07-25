use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum LightMsg {
    SetAmbient,
    SetPoint,
}

enum LightType {
    Ambient,
    Point,
}

impl ToString for LightType {
    fn to_string(&self) -> String {
        match *self {
            LightType::Ambient => "Ambient Light".to_string(),
            LightType::Point => "Point Light".to_string(),
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct AmbientColor {
    red: f64,
    blue: f64,
    green: f64,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct PointOptions {
    red: f64,
    blue: f64,
    green: f64,
    intensity: f64,
}

#[derive(Default)]
pub struct Light {
    pub ambient_color: Option<AmbientColor>,
    pub point_options: Option<PointOptions>,
}

impl Light {
    pub fn new() -> Self {
        Light {
            ambient_color: None,
            point_options: None,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub ambient_color: Option<AmbientColor>,
    pub point_options: Option<PointOptions>,
}

impl Component for Light {
    type Message = LightMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Light {
            ambient_color: props.ambient_color,
            point_options: props.point_options,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LightMsg::SetAmbient => {
                self.ambient_color = Some(AmbientColor::default());
            }
            LightMsg::SetPoint => {
                self.point_options = Some(PointOptions::default());
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.ambient_color = props.ambient_color;
        self.point_options = props.point_options;
        true
    }
}

impl Renderable<Light> for Light {
    fn view(&self) -> Html<Self> {
        let light_type: String;
        if self.point_options.is_some() {
            light_type = "Point Light".to_string();
        } else if self.ambient_color.is_some() {
            light_type = "Ambient Light".to_string();
        } else {
            light_type = "Choose a Light Type".to_string();
        }

        html! {
                <div>
                <button class="uk-button uk-button-default uk-width-1-2 uk-align-center uk-button-primary", type="button", >{ "Light Types" }</button>
                <div uk-dropdown="mode: click", >
                    <ul class="uk-nav uk-dropdown-nav", >
                        <li><a href="#", onclick=|_| LightMsg::SetAmbient, >{ LightType::Ambient }</a></li>
                        <li><a href="#", onclick=|_| LightMsg::SetPoint, >{ LightType::Point }</a></li>
                    </ul>
                </div>
                <legend class="uk-legend uk-margin-small-bottom", >{ light_type }</legend>
                <fieldset class="uk-margin-small-top uk-fieldset", >
                <div class="uk-flex-center uk-child-width-1-4", uk-grid="", >
                    <label class="uk-form-label",>{ "Blue" }
                        <input class="uk-input uk-form-width-small uk-margin-small-left", type="text", placeholder="0.0-1.0", />
                    </label>
                    <label class="uk-form-label", >{ "Green" }
                        <input class="uk-input uk-form-width-small uk-margin-small-left", type="text", placeholder="0.0-1.0", />
                    </label>
                    <label class="uk-form-label", >{ "Red" }
                        <input class="uk-input uk-form-width-small uk-margin-small-left", type="text", placeholder="0.0-1.0", />
                    </label>
                    <label class="uk-form-label", >{ "Intensity" }
                        <input class="uk-input uk-form-width-small uk-margin-small-left", type="text", placeholder="0.0-1.0", />
                    </label>
                </div>
            </fieldset>
            </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            ambient_color: None,
            point_options: None,
        }
    }
}
