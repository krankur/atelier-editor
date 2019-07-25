use std::default::Default;

use crate::components::input::text_input_modal::TextInputModal;
use crate::core::model::MainWindow;

use yew::prelude::*;
use yew::services::ConsoleService;

pub enum Msg {
    Exit,
    ShowNewPrefab,
    ShowNewProject,
}

#[allow(dead_code)]
pub struct NavBar {
    import_prefab_modal: TextInputModal,
    projects: Vec<String>,
    onsignal: Option<Callback<MainWindow>>,
    console: ConsoleService,
    ws_connected: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub import_prefab_modal: TextInputModal,
    pub projects: Vec<String>,
    pub onsignal: Option<Callback<MainWindow>>,
    pub ws_connected: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            import_prefab_modal: TextInputModal::new("Import Prefab"),
            projects: vec![],
            onsignal: None,
            ws_connected: false,
        }
    }
}

impl NavBar {
    pub fn get_view(&self) -> Html<Self> {
        self.view()
    }
}

impl Component for NavBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavBar {
            import_prefab_modal: props.import_prefab_modal,
            projects: props.projects,
            onsignal: props.onsignal,
            console: ConsoleService::new(),
            ws_connected: props.ws_connected,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Exit => true,
            Msg::ShowNewPrefab => match self.onsignal {
                Some(ref _cb) => true,
                None => {
                    self.console.log("No CB arranged");
                    false
                }
            },
            Msg::ShowNewProject => true,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.import_prefab_modal = props.import_prefab_modal;
        self.onsignal = props.onsignal;
        self.ws_connected = props.ws_connected;
        true
    }
}

impl Renderable<NavBar> for NavBar {
    fn view(&self) -> Html<Self> {
        html! {
            <>
            <div style=" display: block; white-space: nowrap; backgorund-color: #000; width: 40px; height: 100%; ",></div>
            <nav uk-navbar="", class="uk-navbar-container noselect", >
                <div class="uk-navbar-left", >
                    <ul class="uk-navbar-nav", >
                        <p href={ "#" }, class="editor-title uk-navbar-item", >{"Amethyst Editor"}</p>
                        <li class="",>
                            <a>{ "Projects" }</a>
                            <div class="uk-navbar-dropdown", >
                                <ul class="uk-nav uk-navbar-dropdown-nav", >
                                    <li>
                                        <a href="#new-project-modal", uk-toggle="", >{ "New Project" }</a>
                                    </li>
                                    <li>
                                        <a href="#oc-project-browser", uk-toggle="", >{ "Open Project" }</a>
                                    </li>
                                    <li><a href="#not-implemented", uk-toggle="", >{ "Preferences" }</a></li>
                                </ul>
                            </div>
                        </li>
                        <li>
                            <a>{ "Prefabs" }</a>
                                <div class="uk-navbar-dropdown", >
                                <ul class="uk-nav uk-navbar-dropdown-nav", >
                                    <li>
                                        <a href="#new-prefab-modal", uk-toggle="", >{ "New" }</a>
                                    </li>
                                    <li>
                                        <a href="#import-prefab-freeform", uk-toggle="", >{ "Load from RON or Rust" }</a>
                                    </li>
                                </ul>
                            </div>
                        </li>
                        <li><a href="#not-implemented", uk-toggle="", >{ "Help" }</a></li>
                        <li>
                            <a href="#",>
                                <span class="uk-icon uk-margin-small-right", uk-icon=if self.ws_connected {"icon: check"} else {"icon: close"},></span>
                                {"Server Connection"}
                            </a>
                        </li>

                    </ul>
                </div>
            </nav>
            </>
        }
    }
}
