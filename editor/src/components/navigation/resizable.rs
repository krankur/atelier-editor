use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use std::default::Default;
use stdweb::web::{document, EventListenerHandle, IEventTarget};
use stdweb::web::event::{MouseMoveEvent, MouseUpEvent};
use yew::prelude::*;
use yew::services::ConsoleService;

pub enum ResizableState {
    Resizing,
    Static,
}

pub enum ResizableMsg {
    StartResize,
}

pub struct Resizable {
    console: ConsoleService,
    state: ResizableState,
    props: Props,
}

impl Default for Resizable {
    fn default() -> Self {
        Resizable {
            console: ConsoleService::new(),
            state: ResizableState::Static,
            props: Props::default(),
        }
    }
}

impl Resizable {
    pub fn new() -> Self {
        Resizable {
            console: ConsoleService::new(),
            state: ResizableState::Static,
            props: Props::default(),
        }
    }
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub inner_template: Box<dyn Renderable<Resizable>>,
}

impl Component for Resizable {
    type Message = ResizableMsg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Resizable {
            console: ConsoleService::new(),
            state: ResizableState::Static,
            props: props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ResizableMsg::StartResize => {
                self.console.info("Resize started!");
                self.state = ResizableState::Resizing;

                let mut mouse_move_listener_handle = Some(document().add_event_listener( move |_: MouseMoveEvent| {
                    let mut console = ConsoleService::new();
                    console.info("resizing!");
                }));

                let mouse_up_listener_handle: Rc<RefCell<Option<EventListenerHandle>>> = Rc::new(RefCell::new(None));

                *mouse_up_listener_handle.borrow_mut() = Some(document().add_event_listener({
                    let mouse_up_listener_handle = mouse_up_listener_handle.clone();
                    move |_: MouseUpEvent| {
                        let mut console = ConsoleService::new();
                        console.info("Resize stopped!");

                        if mouse_move_listener_handle.is_some() {
                            mouse_move_listener_handle.take().unwrap().remove();
                        }
                        if let Some(mouse_up_listener_handle) = mouse_up_listener_handle.borrow_mut().take() {
                            mouse_up_listener_handle.remove();
                        }
                    }
                }));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Resizable> for Resizable {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="resizable",
                onmousedown=|_|ResizableMsg::StartResize, >
                { self.props.inner_template.deref() }
            </div>
        }
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            inner_template: Box::new(Resizable::default()),
        }
    }
}
