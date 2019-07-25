use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum TextInputModalMsg {
    ImportClicked,
    TextChanged(String),
    CancelClicked,
}

#[derive(Clone, PartialEq)]
pub struct TextInputModal {
    title: String,
    content: String,
    placeholder: String,
    button_text: String,
    id: String,
    on_import: Callback<String>,
    warning_hidden: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub content: String,
    pub button_text: String,
    pub placeholder: String,
    pub id: String,
    pub on_import: Option<Callback<String>>,
    pub warning_hidden: bool,
    pub onsignal: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Default".to_string(),
            content: "...".to_string(),
            placeholder: "...".to_string(),
            button_text: "Save".to_string(),
            id: "#generic-modal".to_string(),
            // on_import: Callback::from(|_| return),
            on_import: None,
            warning_hidden: true,
            onsignal: None,
        }
    }
}

impl TextInputModal {
    pub fn new(title: &str) -> Self {
        TextInputModal {
            title: title.to_string(),
            content: String::new(),
            placeholder: String::new(),
            button_text: String::new(),
            id: String::new(),
            on_import: Callback::from(|_| return),
            warning_hidden: true,
        }
    }
}

impl Default for TextInputModal {
    fn default() -> Self {
        TextInputModal {
            title: "Default".to_string(),
            content: "...".to_string(),
            placeholder: "...".to_string(),
            button_text: "Save".to_string(),
            id: "#generic-modal".to_string(),
            on_import: Callback::from(|_| return),
            warning_hidden: true,
        }
    }
}

impl Component for TextInputModal {
    type Message = TextInputModalMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextInputModal {
            title: props.title,
            content: props.content,
            placeholder: props.placeholder,
            button_text: props.button_text,
            on_import: props.on_import.unwrap(),
            id: props.id,
            warning_hidden: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextInputModalMsg::TextChanged(new_content) => {
                self.content = new_content;
            }
            TextInputModalMsg::ImportClicked => {
                self.warning_hidden = false;
            }
            TextInputModalMsg::CancelClicked => {}
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.on_import = props.on_import.unwrap();
        self.content = props.content;
        self.title = props.title;
        self.placeholder = props.placeholder;
        self.button_text = props.button_text;
        self.id = props.id;
        self.warning_hidden = props.warning_hidden;
        true
    }
}

impl Renderable<TextInputModal> for TextInputModal {
    fn view(&self) -> Html<Self> {
        html! {
          <div id={ &self.id }, uk-modal="", >
            <div class="uk-modal-dialog uk-modal-body", >
              <h3 class="uk-modal-title", >{ &self.title }</h3>
                <div class="uk-margin-top", >
                  <textarea class="uk-textarea", placeholder={ &self.placeholder }, rows=5, oninput=|e| TextInputModalMsg::TextChanged(e.value), ></textarea>
                  <p class="uk-text-right", >
                    <button class="uk-button uk-button-primary uk-margin-right", type="button", onclick=|_| { TextInputModalMsg::ImportClicked}, >{ &self.button_text }</button>
                    <button class="uk-button uk-button-danger uk-modal-close uk-margin-right", type="button", >{ "Cancel" }</button>
                  </p>
                </div>
            </div>
          </div>
        }
    }
}
