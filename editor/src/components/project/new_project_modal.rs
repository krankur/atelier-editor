use log::info;
use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

pub enum Msg {
    CreateClicked,
    CancelClicked,
    InputChanged(String),
}

pub struct NewProjectModal {
    name: String,
    project_exists: bool,
    onsignal: Option<Callback<String>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub project_exists: bool,
    #[props(required)]
    pub onsignal: Callback<String>,
}

impl Default for Props {
    fn default() -> Props {
        Props {
            name: "Test Project".to_string(),
            project_exists: false,
            onsignal: Callback::from(|_| return),
        }
    }
}

impl Component for NewProjectModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut _link: ComponentLink<Self>) -> Self {
        NewProjectModal {
            name: props.name,
            project_exists: false,
            onsignal: Some(props.onsignal),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateClicked => {
                info!("Handling Msg::CreateClicked");
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(self.name.clone());
                }
            }
            Msg::CancelClicked => {}
            Msg::InputChanged(name) => {
                self.name = name;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name;
        self.project_exists = props.project_exists;
        true
    }
}

impl Renderable<NewProjectModal> for NewProjectModal {
    fn view(&self) -> Html<Self> {
        html! {
          <div id="new-project-modal", uk-modal="", >
            <div class="uk-modal-dialog uk-modal-body", >
              <h3 class="uk-modal-title", >{ "New Project" }</h3>
                <div class="uk-margin-top", >
                  <label class="uk-form-label",>{ "Project Name" }
                    <input class="uk-input uk-margin-small-left", type="text", placeholder="Project name...",
                    oninput=|e| {Msg::InputChanged(e.value.clone())},
                    value={self.name.clone()},/>
                  </label>
                  <p class="uk-text-right",>
                    <button class="uk-button uk-button-primary uk-margin-right", type="button", uk-toggle="target: #new-project-modal",
                        onclick=|_| Msg::CreateClicked ,>{ "Create" }</button>
                    <button class="uk-button uk-button-danger uk-modal-close uk-margin-right", type="button", >{ "Cancel" }</button>
                  </p>
                </div>
            </div>
          </div>
        }
    }
}
