use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum Msg {
    CreateClicked,
    CancelClicked,
}

pub struct OpenProjectModal {
    name: String,
    on_create: Callback<Msg>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub on_create: Option<Callback<Msg>>,
}

impl OpenProjectModal {
    pub fn new(name: String) -> Self {
        OpenProjectModal {
            name,
            on_create: Callback::from(|_| return),
        }
    }
}

impl Component for OpenProjectModal {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        OpenProjectModal {
            name: props.name,
            on_create: props.on_create.unwrap(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name;
        self.on_create = props.on_create.unwrap();
        true
    }
}

impl Renderable<OpenProjectModal> for OpenProjectModal {
    fn view(&self) -> Html<Self> {
        html! {
          <div id="open-project-modal", uk-modal="", >
            <div class="uk-modal-dialog uk-modal-body", >
              <h3 class="uk-modal-title", >{ "New Project" }</h3>
                <div class="uk-margin-top", >
                  <label class="uk-form-label",>{ "Project Name" }
                    <input class="uk-input uk-margin-small-left", type="text", placeholder="Project name...", />
                  </label>
                  <p class="uk-text-right", >
                    <button class="uk-button uk-button-primary uk-margin-right", type="button", uk-toggle="target: #open-project-modal", onclick=|_| { Msg::CreateClicked}, >{ "Create" }</button>
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
            name: "New Project".to_string(),
            // on_create: Callback::from(|_| return),
            on_create: None,
        }
    }
}
