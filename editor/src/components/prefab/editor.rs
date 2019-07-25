use std::default::Default;
use std::string::ToString;
use yew::prelude::*;
use yew::services::ConsoleService;

use super::editor_row::PrefabEditorRow;

use crate::components::prefab::scene::form::Form as SceneForm;
use crate::components::prefab::ui::form::Form as UIForm;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowMsg {
    AddRow,
    ShowPrefabForm(PrefabForms),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefabForms {
    UI,
    Scene,
}

/// Primary window for the Prefab Editor
pub struct PrefabEditorWindow {
    pub rows: Vec<PrefabEditorRow>,
    pub add_row_callback: Callback<()>,
    pub selected_prefab: Option<PrefabForms>,
    pub on_prefab_create: Callback<WindowMsg>,
    console: ConsoleService,
}

/// Props for the Prefab Editor Window
#[derive(Clone, PartialEq, Properties)]
pub struct PrefabEditorProps {
    pub on_prefab_create: Option<Callback<WindowMsg>>,
    pub rows: Vec<PrefabEditorRow>,
    pub add_row_callback: Option<Callback<()>>,
    pub selected_prefab: Option<PrefabForms>,
}

impl Default for PrefabEditorProps {
    fn default() -> Self {
        PrefabEditorProps {
            on_prefab_create: Some(Callback::from(|_| return)),
            // on_prefab_create: None,
            rows: vec![],
            add_row_callback: Some(Callback::from(|_| return)),
            // add_row_callback: None,
            selected_prefab: None,
        }
    }
}

impl Default for PrefabEditorWindow {
    fn default() -> Self {
        PrefabEditorWindow {
            console: ConsoleService::new(),
            rows: vec![],
            add_row_callback: Callback::from(|_| return),
            selected_prefab: None,
            on_prefab_create: Callback::from(|_| return),
        }
    }
}

impl PrefabEditorWindow {
    pub fn add_row(&mut self, name: String, value: String, add_row_callback: Callback<()>) {
        let row = PrefabEditorRow {
            name,
            value,
            on_add: add_row_callback,
        };
        self.rows.push(row)
    }
}

pub fn on_prefab_select(value: String) -> String {
    value
}

impl Component for PrefabEditorWindow {
    type Message = WindowMsg;
    type Properties = PrefabEditorProps;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut window = PrefabEditorWindow {
            console: ConsoleService::new(),
            rows: props.rows,
            add_row_callback: link.send_back(|_| WindowMsg::AddRow),
            selected_prefab: None,
            on_prefab_create: link.send_back(|_| WindowMsg::ShowPrefabForm(PrefabForms::Scene)),
        };
        window.add_row(
            "Field Name".to_string(),
            "Field Value".to_string(),
            window.add_row_callback.clone(),
        );
        window
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log("Message received");
        match msg {
            WindowMsg::AddRow => {
                self.add_row(
                    "Test1".to_string(),
                    "Test2".to_string(),
                    self.add_row_callback.clone(),
                );
            }
            WindowMsg::ShowPrefabForm(form) => {
                self.console.log("Received show prefab form");
                self.selected_prefab = Some(form);
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.rows = props.rows;
        self.add_row_callback = props.add_row_callback.unwrap();
        self.selected_prefab = props.selected_prefab;
        self.on_prefab_create = props.on_prefab_create.unwrap();
        true
    }
}

impl Renderable<PrefabEditorWindow> for PrefabEditorWindow {
    fn view(&self) -> Html<Self> {
        let f = {
            match &self.selected_prefab {
                Some(f) => match f {
                    PrefabForms::UI => {
                        html! {
                            <UIForm: />
                        }
                    }
                    PrefabForms::Scene => {
                        html! {
                            <SceneForm: />
                        }
                    }
                },
                None => {
                    html! {
                        <div />
                    }
                }
            }
        };

        html! {
            <div>
                { f }
            </div>
        }
    }
}
