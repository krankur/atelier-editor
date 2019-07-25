use std::default::Default;

use yew::prelude::*;

pub enum Msg {
    AddProject(String),
    DelProject(String),
}

#[derive(Clone, PartialEq)]
pub struct ProjectBrowser {
    projects: Vec<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub projects: Vec<String>,
}

impl Default for Props {
    fn default() -> Props {
        Props { projects: vec![] }
    }
}

impl Component for ProjectBrowser {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ProjectBrowser {
            projects: props.projects,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddProject(name) => {
                self.projects.push(name);
            }
            Msg::DelProject(_name) => {}
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.projects = props.projects;
        true
    }
}

impl Renderable<ProjectBrowser> for ProjectBrowser {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="oc-project-browser", uk-offcanvas="", >
              <div class="uk-offcanvas-bar", >
                <h3>{ "Projects" }</h3>
                <hr />
                <ul class="uk-nav uk-nav-default",>
                    { for self.projects.iter().map(|p| view_project(p.to_string())) }
                </ul>
              </div>
            </div>
        }
    }
}

fn view_project(project: String) -> Html<ProjectBrowser> {
    html! {
        <li><a href="#", >{ project }</a></li>
    }
}
