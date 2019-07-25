use failure::Error;
use log::info;

use serde_derive::{Deserialize, Serialize};

use crate::components::input::text_input_modal::TextInputModal;
use crate::components::navigation::navbar::NavBar;
use crate::components::navigation::not_implemented_modal::NotImplementedModal;
use crate::components::navigation::project_browser::ProjectBrowser;
use crate::components::navigation::editor_toolbar::EditorToolbar;
use crate::components::navigation::resizable::Resizable;
use crate::components::prefab::prefab_type_modal::PrefabNewModal;
use crate::components::prefab::scene::form::Form as SceneForm;
use crate::components::prefab::ui::form::Form as UIForm;
use crate::components::project::new_project_modal::NewProjectModal;
use crate::components::project::open_project_modal::OpenProjectModal;

use crate::storage::Prefab;

use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::ConsoleService;

/// Messages the model responds to
pub enum Msg {
    /// Quits the application
    Quit,
    CreateNewPrefab,
    CreateNewProject(String),
    ShowMainWindow(String),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
    WsConnected,
}

pub enum WsAction {
    Connect,
    Disconnect,
    Lost,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

/// Enum used to decide what is displayed in the main window
pub enum MainWindow {
    None,
    PrefabEditor(String),
    ProjectEditor,
}

/// Base model that keeps application state if needed
#[allow(dead_code)]
pub struct Model {
    state: State,
    storage: StorageService,
    console: ConsoleService,
    ws: WebSocketService,
    ws_task: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    ws_data: Option<u32>,
    project_exists: bool,
}

impl Model {
    fn choose_primary_window(&self) -> Html<Self> {
        match self.state.main_window {
            Some(ref mw) => match mw {
                MainWindow::None => self.empty_primary_window(),
                MainWindow::PrefabEditor(window) => self.prefab_editor_window(window),
                MainWindow::ProjectEditor => self.project_editor_window(),
            },
            None => self.empty_primary_window(),
        }
    }

    fn project_editor_window(&self) -> Html<Self> {
        html! {
            <div uk-grid="", class="uk-flex-center", >
                <div class="uk-width-1-4", ></div>
                <div class="uk-card uk-card-body uk-width-expand", >
                    <p>{ "Soon you'll be able to edit all sorts of things about your project here!" }</p>
                </div>
                <div class="uk-width-1-4", ></div>
            </div>
        }
    }

    fn prefab_editor_window(&self, window: &str) -> Html<Self> {
        let f = {
            if window == "Scene" {
                html! {
                    <SceneForm: />
                }
            } else if window == "UI" {
                html! {
                    <UIForm: />
                }
            } else {
                html! {
                    <div />
                }
            }
        };
        html! {
            <div uk-grid="", class="uk-flex-center", >
                <div class="uk-width-1-4", ></div>
                <div class="uk-card uk-card-body uk-width-expand", >
                    { f }
                </div>
                <div class="uk-width-1-4", ></div>
            </div>
        }
    }

    fn empty_primary_window(&self) -> Html<Self> {
        html! {
            <>
            <img draggable="false", class="engien-icon", src="img/amethyst_logo.svg", />
            <div uk-grid="", class="uk-flex-center", >
                <div class="uk-width-1-4", ></div>
                <div class="uk-card uk-card-body uk-width-expand", >
                    <h4> {"Hello!"} </h4>
                    <p> {"Your workspace looks a bit lonely...did you know can create a new project or open an existing one by clicking on Projects in the upper left?"} </p>
                    <p> {"If you'd like to just work on a prefab, click the Prefab menu along the top."} </p>
                </div>
                <div class="uk-width-1-4", ></div>
            </div>
            </>
        }
    }
}

/// The state of the application that is built up as the user works in the app
#[allow(dead_code)]
pub struct State {
    opened_prefab: Option<Prefab>,
    main_window: Option<MainWindow>,
    ws_connected: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            opened_prefab: None,
            main_window: Some(MainWindow::None),
            ws_connected: false,
        };

        Model {
            state,
            storage: StorageService::new(Area::Local),
            console: ConsoleService::new(),
            ws: WebSocketService::new(),
            ws_task: None,
            link,
            ws_data: None,
            project_exists: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Quit => {}
            Msg::CreateNewPrefab => {
                self.state.opened_prefab = Some(Prefab::new());
            }
            Msg::CreateNewProject(project_name) => {
                info!("Create new project request for project: {}", project_name);
            }
            Msg::ShowMainWindow(window) => {
                let w = window;
                if w == "UI" || w == "Scene" {
                    self.state.main_window = Some(MainWindow::PrefabEditor(w));
                } else {
                    self.state.main_window = None;
                }
            }
            // Websocket related messages
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = self.link.send_back(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.send_back(|status| match status {
                        WebSocketStatus::Opened => Msg::WsConnected,
                        WebSocketStatus::Closed => Msg::Ignore,
                        WebSocketStatus::Error => WsAction::Lost.into(),
                    });
                    let task = self
                        .ws
                        .connect("ws://localhost:9001/", callback, notification);

                    self.ws_task = Some(task);
                }
                WsAction::Disconnect => {}
                WsAction::Lost => {
                    self.console.log("WS connection was lost");
                }
            },
            Msg::WsReady(message) => {
                self.console
                    .log(&format! {"Received a message: {:?}", message});
            }
            Msg::WsConnected => {
                self.state.ws_connected = true;
            }
            Msg::Ignore => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    /// This is the main HTML section for the editor. All other parts of the Editor are contained in this div.
    fn view(&self) -> Html<Self> {
        html! {
            <div class="editor-wrapper",>
                <EditorToolbar: />
                <NotImplementedModal: />
                <OpenProjectModal: />
                <ProjectBrowser: />
                <PrefabNewModal: onsignal=|window| {Msg::ShowMainWindow(window)}, />
                <TextInputModal: title="Import Prefab", id="import-prefab-modal", placeholder="Paste your RON or Rust here...", button_text="Import", />
                <NewProjectModal: onsignal=|project_name| Msg::CreateNewProject(project_name), />
                <section class="editor",>
                    <header class="header",>
                        <NavBar: ws_connected={self.state.ws_connected}, />
                    </header>
                <section class="main",>
                    // <Resizable: />
                    <div>{ "nested" }</div>
                    {self.choose_primary_window()}
                </section>
                <footer class="footer",>
                </footer>
                </section>
                <footer class="info",>
                </footer>
            </div>
        }
    }
}
