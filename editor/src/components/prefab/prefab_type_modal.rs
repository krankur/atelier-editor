use std::default::Default;
use std::string::ToString;
use yew::html::ChangeData;
use yew::prelude::*;

use crate::components::prefab::editor::PrefabForms;

pub enum PrefabNewModalMsg {
    TypeChange(ChangeData),
    CreateClicked,
}

#[derive(Default)]
pub struct PrefabNewModal {
    /// Initially selected value.
    pub selected: Option<PrefabForms>,
    /// Callback to handle changes.
    pub options: Vec<String>,
    onsignal: Option<Callback<String>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Initially selected value.
    pub selected: Option<PrefabForms>,
    /// Callback to handle changes.
    pub options: Vec<String>,
    #[props(required)]
    pub onsignal: Callback<String>,
}

impl PrefabNewModal {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PrefabNewModal {
            selected: None,
            options: vec![],
            onsignal: None,
        }
    }
}

impl Component for PrefabNewModal {
    type Message = PrefabNewModalMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PrefabNewModal {
            selected: props.selected,
            options: props.options,
            onsignal: Some(props.onsignal),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PrefabNewModalMsg::TypeChange(value) => match value {
                ChangeData::Value(_) => {}
                ChangeData::Select(se) => match se.value() {
                    Some(v) => {
                        if v == "ui" {
                            self.selected = Some(PrefabForms::UI);
                        } else if v == "scene" {
                            self.selected = Some(PrefabForms::Scene);
                        } else {
                            self.selected = None;
                        }
                    }
                    None => {
                        self.selected = None;
                    }
                },
                _ => {}
            },
            PrefabNewModalMsg::CreateClicked => {
                match &self.onsignal {
                    Some(signal) => {
                        match &self.selected {
                            Some(s) => match s {
                                PrefabForms::UI => {
                                    signal.emit("UI".to_string());
                                }
                                PrefabForms::Scene => {
                                    signal.emit("Scene".to_string());
                                }
                            },
                            None => {}
                        };
                    }
                    None => {}
                };
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.selected = props.selected;
        self.options = props.options;
        self.onsignal = Some(props.onsignal);
        true
    }
}

impl Renderable<PrefabNewModal> for PrefabNewModal {
    #[allow(unused_variables, unreachable_patterns)]
    fn view(&self) -> Html<Self> {
        let ui_s = String::from("ui");
        let scene_s = String::from("scene");
        html! {
          <div id="new-prefab-modal", uk-modal="", >
            <div class="uk-modal-dialog uk-modal-body", >
              <h3 class="uk-modal-title", >{ "Prefab Type" }</h3>
                <select class="uk-select", onchange=|event| PrefabNewModalMsg::TypeChange(event),>
                    <option value="ui", >{ "UI" }</option>
                    <option value="scene", >{ "Scene" }</option>
                </select>
                <div class="uk-margin-top", >
                  <p class="uk-text-right", >
                    <button class="uk-button uk-button-primary uk-margin-right", type="button", uk-toggle="target: #new-prefab-modal", onclick=|_| { PrefabNewModalMsg::CreateClicked}, >{ "Create" }</button>
                    <button class="uk-button uk-button-danger uk-modal-close uk-margin-right", type="button", >{ "Cancel" }</button>
                  </p>
                </div>
            </div>
          </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            selected: None,
            options: vec![],
            onsignal: Callback::from(|_| return),
        }
    }
}
