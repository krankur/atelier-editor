//! Primary form for inputting a Scene Prefab

use std::default::Default;

use yew::html::Renderable;
use yew::prelude::*;

use crate::components::prefab::scene::camera::Camera;
use crate::components::prefab::scene::control::Control;
use crate::components::prefab::scene::graphics::Graphics;
use crate::components::prefab::scene::light::Light;
use crate::components::prefab::scene::removal::Removal;
use crate::components::prefab::scene::transform::Transform;

/// Message to tell the form which section to display
pub enum FormMsg {
    ShowLight,
    ShowTransform,
    ShowCamera,
    ShowControl,
    ShowGraphics,
    ShowRemoval,
}

#[derive(Clone, PartialEq)]
pub enum ComponentsForm {
    Light,
    Transform,
    Camera,
    Control,
    Graphics,
    Removal,
}

impl Default for ComponentsForm {
    fn default() -> ComponentsForm {
        ComponentsForm::Light
    }
}

pub struct Form {
    active_form: ComponentsForm,
}

impl Form {
    pub fn new() -> Self {
        Form {
            active_form: ComponentsForm::Light,
        }
    }
}

impl Default for Form {
     fn default() -> Self {
         Self::new()
     }
 }

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub active_form: ComponentsForm,
}

impl Component for Form {
    type Message = FormMsg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Form {
            active_form: ComponentsForm::Light,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FormMsg::ShowLight => {
                self.active_form = ComponentsForm::Light;
            }
            FormMsg::ShowTransform => {
                self.active_form = ComponentsForm::Transform;
            }
            FormMsg::ShowCamera => {
                self.active_form = ComponentsForm::Camera;
            }
            FormMsg::ShowControl => {
                self.active_form = ComponentsForm::Control;
            }
            FormMsg::ShowGraphics => {
                self.active_form = ComponentsForm::Graphics;
            }
            FormMsg::ShowRemoval => {
                self.active_form = ComponentsForm::Removal;
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.active_form = props.active_form;
        true
    }
}

impl Renderable<Form> for Form {
    fn view(&self) -> Html<Self> {
        // Expression that we use to decide which section to show. We evaluate further down.
        let f = {
            match self.active_form {
                ComponentsForm::Light => {
                    html! { <Light: ambient_color=None, point_options=None, /> }
                }
                ComponentsForm::Transform => {
                    html! { <Transform: rotation=None, translation=None, />}
                }
                ComponentsForm::Camera => {
                    html! { <Camera: /> }
                }
                ComponentsForm::Control => {
                    html! { <Control: /> }
                }
                ComponentsForm::Graphics => {
                    html! { <Graphics: /> }
                }
                ComponentsForm::Removal => {
                    html! { <Removal: /> }
                }
            }
        };
        html! {
          <div class="uk-width-1-1", >
            <h1 class="uk-align-center", >{ "Scene Prefab" }</h1>
            <form class="uk-form-stacked", >
              <button class="uk-button uk-button-default uk-width-1-1 uk-align-center", type="button", >{ "Components" }</button>
                <div uk-dropdown="mode: click", >
                    <ul class="uk-nav uk-dropdown-nav", >
                        <li><a href="#", onclick=|_| FormMsg::ShowLight, >{ "Lighting" }</a></li>
                        <li><a href="#", onclick=|_| FormMsg::ShowTransform,>{ "Transform" }</a></li>
                        <li><a href="#", onclick=|_| FormMsg::ShowCamera,>{ "Camera" }</a></li>
                        <li><a href="#", onclick=|_| FormMsg::ShowControl,>{ "Control" }</a></li>
                        <li><a href="#", onclick=|_| FormMsg::ShowGraphics,>{ "Graphics" }</a></li>
                        <li><a href="#", onclick=|_| FormMsg::ShowRemoval,>{ "Removal" }</a></li>
                    </ul>
                </div>
            <div>
            { f }
            </div>
            </form>
          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            active_form: ComponentsForm::Light,
        }
    }
}
