use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum RowMsg {
    AddRow,
    DelRow,
}

#[derive(Clone, PartialEq)]
pub struct PrefabEditorRow {
    pub name: String,
    pub value: String,
    pub on_add: Callback<()>,
}

#[allow(dead_code)]
impl PrefabEditorRow {
    fn new(name: String, value: String) -> Self {
        PrefabEditorRow {
            name,
            value,
            on_add: Callback::from(|_| return),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub value: String,
    pub on_add: Option<Callback<()>>,
}

impl Component for PrefabEditorRow {
    type Message = RowMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PrefabEditorRow {
            name: props.name,
            value: props.value,
            on_add: props.on_add.unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RowMsg::AddRow => self.on_add.emit(()),
            RowMsg::DelRow => self.on_add.emit(()),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name;
        self.value = props.value;
        self.on_add = props.on_add.unwrap();
        true
    }
}

impl Renderable<PrefabEditorRow> for PrefabEditorRow {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input class="uk-margin-small-right uk-input uk-width-1-4", type="text", placeholder="Field Name", />
                <input class="uk-margin-small-right uk-input uk-width-1-4", type="text", placeholder="Field Value", />
                <button class="uk-margin-small-right uk-button uk-button-primary uk-width-1-6", onclick=|_| RowMsg::AddRow, >{ "Add" }</button>
                <button class="uk-button uk-button-secondary uk-width-1-6", onclick=|_| RowMsg::DelRow, >{ "Delete" }</button>
            </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            name: "Field Name".to_string(),
            value: "Field Value".to_string(),
            // on_add: Callback::from(|_| return),
            on_add: None,
        }
    }
}
