use std::default::Default;
use std::string::ToString;
use yew::prelude::*;

use yew::services::ConsoleService;

pub enum Msg {
    CreateClicked,
    CancelClicked,
}

pub struct OpenProjectModal {
    name: String,
    on_create: Option<Callback<Msg>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub on_create: Option<Callback<Msg>>,
}

impl OpenProjectModal {
    pub fn new(name: String) -> Self {
        let mut console = ConsoleService::new();
        console.info("open project modal create modal new!");
        OpenProjectModal {
            name,
            on_create: Some(Callback::from(|_| return)),
        }
    }
}

impl Component for OpenProjectModal {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut console = ConsoleService::new();
        if props.on_create.is_some() {
            console.info("open project modal create!");
        } else {
            console.info("open project modal create none!");
        }
        OpenProjectModal {
            name: props.name,
            on_create: props.on_create,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut console = ConsoleService::new();
        if props.on_create.is_some() {
            console.info("open project modal create!");
        } else {
            console.info("open project modal create none!");
        }
        self.name = props.name;
        self.on_create = props.on_create;
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
        let mut console = ConsoleService::new();
        console.info("open project modal props default!");
        Props {
            name: "New Project".to_string(),
            on_create: Some(Callback::from(|_| return)),
            // on_create: None,
        }
    }
}
